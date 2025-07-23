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
) -> Result<(), Box<dyn std::error::Error>> {
    let repo_dir = PathBuf::from(CLONE_STORAGE_PATH).join(repo.id.to_string() + ".git");

    // ensure dir exists
    fs::create_dir_all(CLONE_STORAGE_PATH).await?;

    // if repo dir exists - remove it
    if fs::metadata(&repo_dir).await.is_ok() {
        fs::remove_dir_all(&repo_dir).await?;
    }

    // clone the repository
    let src = repo.git_source;
    let status = Command::new("git")
        .args(&["clone", "--mirror", &src, repo_dir.to_str().unwrap()])
        .output()
        .await?;
    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        return Err(format!("git clone failed: {}", stderr).into());
    }

    // set url target
    let tgt = repo.git_target;
    let status = Command::new("git")
        .current_dir(&repo_dir)
        .args(&["remote", "set-url", "--push", "origin", &tgt])
        .output()
        .await?;
    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        return Err(format!("git remote set-url failed: {}", stderr).into());
    }

    // push repository to target
    let status = Command::new("git")
        .current_dir(&repo_dir)
        .args(&["push", "--mirror", "origin"])
        .output()
        .await?;
    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        return Err(format!("git push --mirror failed: {}", stderr).into());
    }

    println!("Successfully mirrored repo {} to {}", repo.id, tgt);

    Ok(())
}
