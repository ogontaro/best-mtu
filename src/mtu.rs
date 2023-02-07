use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PingError {
    #[error("{0} size message is too long")]
    MessageTooLong(String),
}

pub fn check(size: u32) -> Result<u32> {
    let output = std::process::Command::new("ping")
        .args(["-c", "2"])
        .arg("-D")
        .args(["-s", size.to_string().as_str()])
        .arg("google.co.jp")
        .output()
        .context("ping execution failed")?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let result = stderr
        .lines()
        .find(|line| line.contains("Message too long"));
    match result {
        Some(_) => Err(PingError::MessageTooLong(size.to_string()).into()),
        None => Ok(size),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        let result = check(3000);
        match result {
            Ok(_) => {
                assert!(false);
            }
            Err(e) => match e.downcast_ref::<PingError>().unwrap() {
                PingError::MessageTooLong(_) => {}
            },
        }
        let result = check(1300);
        assert!(result.is_ok());
    }
}
