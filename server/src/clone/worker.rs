use rocket::tokio;

use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use tokio::process::Command;

pub async fn clone_worker_run(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("bash")
        .arg("-c")
        .arg("echo test")
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("bash stdout: {}", stdout);
    if !stderr.is_empty() {
        eprintln!("bash stderr: {}", stderr);
    }

    if !output.status.success() {
        return Err(format!("bash exited with {}", output.status).into());
    }

    Ok(())
}
