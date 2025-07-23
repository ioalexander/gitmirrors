use std::path::PathBuf;

use rocket::tokio;

use diesel::prelude::*;
use diesel::{
    PgConnection,
    dsl::sql,
    r2d2::{ConnectionManager, Pool},
    sql_types::{Bool, Interval},
};
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::schema::repository::dsl::*;
use tokio::process::Command;

use crate::models::RepositoryModel;

const CLONE_STORAGE_PATH: &str = "clone_storage/repositories/";
const KEY_STORAGE_PATH: &str = "clone_storage/keys/";

pub async fn clone_worker_fetch_due_repos(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<Vec<RepositoryModel>, diesel::result::Error> {
    let connection = &mut pool.get().unwrap();

    repository
        .filter(is_enabled.eq(true))
        .filter(sql::<Bool>(
            "(coalesce(last_clone_at, 'epoch'::timestamptz)
                  + (git_clone_period_seconds || ' seconds')::interval)
                  <= now()",
        ))
        .order(sql::<Interval>(
            "now() - coalesce(last_clone_at, 'epoch'::timestamptz) DESC",
        ))
        .limit(3)
        .load::<RepositoryModel>(connection)
}

pub async fn clone_worker_run(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let repositories_to_clone = clone_worker_fetch_due_repos(pool).await.unwrap();

    for repo in repositories_to_clone {
        let repo_id = repo.id;
        if let Err(e) = clone_worker_run_single_repo(repo).await {
            eprintln!("Failed to clone repo {}: {:?}", repo_id, e);
        }
    }

    Ok(())
}

pub async fn clone_worker_run_single_repo(
    repo: RepositoryModel,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // check for ssh binary
    let ssh_check = Command::new("ssh").arg("-V").output().await;

    if ssh_check.is_err() || !ssh_check.as_ref().unwrap().status.success() {
        return Err(
            "`ssh` binary not found or failed to run. Is `openssh-client` installed?".into(),
        );
    }

    let repo_id = repo.id.to_string();
    let source_key_opt = repo.git_source_secret_key.clone();
    let target_key_opt = repo.git_target_secret_key.clone();

    let repo_dir = PathBuf::from(CLONE_STORAGE_PATH).join(format!("{}.git", repo_id));

    // Ensure clone & key directories exist
    fs::create_dir_all(CLONE_STORAGE_PATH).await?;
    fs::create_dir_all(KEY_STORAGE_PATH).await?;

    // Cleanup target dir if it already exists
    if fs::metadata(&repo_dir).await.is_ok() {
        fs::remove_dir_all(&repo_dir).await?;
    }

    // Optional: write source key to file
    let source_key_path = PathBuf::from(KEY_STORAGE_PATH).join(format!("{}_source_key", repo_id));
    if let Some(source_key) = repo.git_source_secret_key {
        let mut file = fs::File::create(&source_key_path).await?;
        file.write_all(source_key.as_bytes()).await?;
        // fs::set_permissions(&source_key_path, std::fs::Permissions::from_mode(0o600))?;
    }

    // Optional: write target key to file
    let target_key_path = PathBuf::from(KEY_STORAGE_PATH).join(format!("{}_target_key", repo_id));
    if let Some(target_key) = repo.git_target_secret_key {
        let mut file = fs::File::create(&target_key_path).await?;
        file.write_all(target_key.as_bytes()).await?;
        // fs::set_permissions(&target_key_path, std::fs::Permissions::from_mode(0o600))?;
    }

    // Build SSH command for source key
    let git_ssh_source = if source_key_opt.is_some() {
        Some(format!(
            "ssh -i {} -o StrictHostKeyChecking=no",
            source_key_path.to_string_lossy()
        ))
    } else {
        None
    };

    // Clone source repo
    let mut cmd = Command::new("git");
    cmd.args(&[
        "clone",
        "--mirror",
        &repo.git_source,
        repo_dir.to_str().unwrap(),
    ]);
    if let Some(ref ssh_cmd) = git_ssh_source {
        cmd.env("GIT_SSH_COMMAND", ssh_cmd);
    }
    let output = cmd.output().await?;
    if !output.status.success() {
        cleanup_keys(&source_key_path, &target_key_path).await;
        return Err(format!(
            "git clone failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    // Set push remote URL
    let output = Command::new("git")
        .current_dir(&repo_dir)
        .args(&["remote", "set-url", "--push", "origin", &repo.git_target])
        .output()
        .await?;
    if !output.status.success() {
        cleanup_keys(&source_key_path, &target_key_path).await;
        return Err(format!(
            "git remote set-url failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    // Build SSH command for target key
    let git_ssh_target = if target_key_opt.is_some() {
        Some(format!(
            "ssh -i {} -o StrictHostKeyChecking=no",
            target_key_path.to_string_lossy()
        ))
    } else {
        None
    };

    // Push mirror to target
    let mut cmd = Command::new("git");
    cmd.current_dir(&repo_dir)
        .args(&["push", "--mirror", "origin"]);
    if let Some(ref ssh_cmd) = git_ssh_target {
        cmd.env("GIT_SSH_COMMAND", ssh_cmd);
    }
    let output = cmd.output().await?;
    if !output.status.success() {
        cleanup_keys(&source_key_path, &target_key_path).await;
        return Err(format!(
            "git push --mirror failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    // Cleanup
    cleanup_keys(&source_key_path, &target_key_path).await;

    println!(
        "Successfully mirrored repo {} to {}",
        repo.id, repo.git_target
    );
    Ok(())
}

async fn cleanup_keys(source: &PathBuf, target: &PathBuf) {
    let _ = fs::remove_file(source).await;
    let _ = fs::remove_file(target).await;
}
