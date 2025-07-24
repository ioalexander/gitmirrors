use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;

use chrono::Utc;
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
use crate::utils::crypto::sanitize_ssh_key;
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

        // catch panics synchronously around the creation of the future
        let future = catch_unwind(AssertUnwindSafe(|| {
            clone_worker_run_single_repo(pool, repo)
        }));

        match future {
            Ok(fut) => {
                // now await the future and handle its error normally
                if let Err(e) = fut.await {
                    eprintln!("Failed to clone repo {}: {:?}", repo_id, e);
                }
            }
            Err(panic) => {
                eprintln!(
                    "Panic occurred while creating clone future for repo {}: {:?}",
                    repo_id, panic
                );
            }
        }
    }

    Ok(())
}

pub async fn clone_worker_run_single_repo(
    pool: &Pool<ConnectionManager<PgConnection>>,
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

    let source_key_opt = repo
        .git_source_secret_key
        .as_ref()
        .map(|k| sanitize_ssh_key(k));
    let target_key_opt = repo
        .git_target_secret_key
        .as_ref()
        .map(|k| sanitize_ssh_key(k));

    let repo_dir = PathBuf::from(CLONE_STORAGE_PATH).join(format!("{}.git", repo_id));

    // Ensure clone & key directories exist
    fs::create_dir_all(CLONE_STORAGE_PATH).await?;
    fs::create_dir_all(KEY_STORAGE_PATH).await?;

    // Cleanup target dir if it already exists
    if fs::metadata(&repo_dir).await.is_ok() {
        fs::remove_dir_all(&repo_dir).await?;
    }

    // Write source key to file
    let source_key_path = PathBuf::from(KEY_STORAGE_PATH).join(format!("{}_source_key", repo_id));
    if let Some(source_key) = repo.git_source_secret_key {
        write_key_file(&source_key_path, &source_key).await?;
    }

    // Write target key to file
    let target_key_path = PathBuf::from(KEY_STORAGE_PATH).join(format!("{}_target_key", repo_id));
    if let Some(target_key) = repo.git_target_secret_key {
        write_key_file(&target_key_path, &target_key).await?;
    }

    // Sanity checks on keys before using
    let check_key = |path: &PathBuf| -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !path.exists() {
            return Err(format!("SSH key file missing: {}", path.display()).into());
        }
        let meta = std::fs::metadata(path)?;
        if meta.len() == 0 {
            return Err(format!("SSH key file is empty: {}", path.display()).into());
        }
        // Try opening file for read to confirm accessibility
        let mut f = std::fs::File::open(path)?;
        let mut buf = [0u8; 1];
        if f.read(&mut buf)? == 0 {
            return Err(format!("SSH key file unreadable: {}", path.display()).into());
        }
        Ok(())
    };

    if let Some(source_key) = source_key_opt.as_ref() {
        if !source_key.trim().is_empty() {
            check_key(&source_key_path)?;
        }
    }
    if target_key_opt.is_some() {
        check_key(&target_key_path)?;
    }

    // Use absolute canonicalized paths in SSH commands
    let source_key_path = if source_key_opt.is_some() {
        Some(source_key_path.canonicalize()?)
    } else {
        None
    };
    let target_key_path = if target_key_opt.is_some() {
        Some(target_key_path.canonicalize()?)
    } else {
        None
    };

    // Build SSH command for source key
    let git_ssh_source = source_key_path
        .as_ref()
        .map(|p| format!("ssh -i {} -o StrictHostKeyChecking=no", p.display()));

    let mut cmd = Command::new("git");
    cmd.args([
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
        cleanup_keys(
            &source_key_path.unwrap_or_default(),
            &target_key_path.unwrap_or_default(),
        )
        .await;
        return Err(format!(
            "git clone failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let output = Command::new("git")
        .current_dir(&repo_dir)
        .args(["remote", "set-url", "--push", "origin", &repo.git_target])
        .output()
        .await?;
    if !output.status.success() {
        cleanup_keys(
            &source_key_path.unwrap_or_default(),
            &target_key_path.unwrap_or_default(),
        )
        .await;
        return Err(format!(
            "git remote set-url failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let git_ssh_target = target_key_path
        .as_ref()
        .map(|p| format!("ssh -i {} -o StrictHostKeyChecking=no", p.display()));

    let mut cmd = Command::new("git");
    cmd.current_dir(&repo_dir)
        .args(["push", "--mirror", "origin"]);
    if let Some(ref ssh_cmd) = git_ssh_target {
        cmd.env("GIT_SSH_COMMAND", ssh_cmd);
    }
    let output = cmd.output().await?;
    if !output.status.success() {
        cleanup_keys(
            &source_key_path.unwrap_or_default(),
            &target_key_path.unwrap_or_default(),
        )
        .await;
        return Err(format!(
            "git push --mirror failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    cleanup_keys(
        &source_key_path.unwrap_or_default(),
        &target_key_path.unwrap_or_default(),
    )
    .await;

    clone_worker_mark_repo_as_cloned(pool, repo.id).await?;

    Ok(())
}

pub async fn clone_worker_mark_repo_as_cloned(
    pool: &Pool<ConnectionManager<PgConnection>>,
    repo_id: uuid::Uuid,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool.clone();

    tokio::task::spawn_blocking(move || {
        let mut conn = pool
            .get()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        diesel::update(repository.filter(id.eq(repo_id)))
            .set(last_clone_at.eq(Utc::now().naive_utc()))
            .execute(&mut conn)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        Ok(())
    })
    .await?
}

async fn write_key_file(path: &PathBuf, key: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(path).await?;

    // ensure that key file ends with new line. Otherwise it will fail
    let mut content = key.trim_end().to_string();
    content.push('\n');

    file.write_all(content.as_bytes()).await?;

    tokio::task::spawn_blocking({
        let path = path.clone();
        move || std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
    })
    .await??;

    Ok(())
}

async fn cleanup_keys(source: &PathBuf, target: &PathBuf) {
    let _ = fs::remove_file(source).await;
    let _ = fs::remove_file(target).await;
}
