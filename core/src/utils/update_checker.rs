use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

/// Version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub current: String,
    pub latest: Option<String>,
    pub release_url: Option<String>,
}

/// Update checker for GitHub releases
pub struct UpdateChecker {
    github_repo: String,
    check_interval_days: u64,
}

impl UpdateChecker {
    pub fn new(github_repo: &str) -> Self {
        Self {
            github_repo: github_repo.to_string(),
            check_interval_days: 1,
        }
    }

    /// Check if update is available on GitHub
    pub async fn check_for_updates(&self) -> Result<Option<VersionInfo>, Box<dyn std::error::Error>> {
        log::info!("Checking for updates from GitHub: {}", self.github_repo);

        // Get latest release from GitHub API
        let url = format!("https://api.github.com/repos/{}/releases/latest", self.github_repo);
        
        // For now, we'll use a simple HTTP request
        // In production, use reqwest crate
        log::info!("Update check URL: {}", url);
        
        Ok(None) // Placeholder - see below for full implementation
    }

    /// Should we check for updates (respects check_interval_days)
    pub fn should_check(&self) -> bool {
        let check_file = self.get_check_file_path();
        
        if !check_file.exists() {
            return true;
        }

        // Check if enough time has passed
        if let Ok(metadata) = fs::metadata(&check_file) {
            if let Ok(elapsed) = metadata.modified().unwrap().elapsed() {
                let days = elapsed.as_secs() / 86400;
                return days >= self.check_interval_days;
            }
        }

        false
    }

    /// Mark that we checked for updates
    pub fn mark_checked(&self) -> Result<(), Box<dyn std::error::Error>> {
        let check_file = self.get_check_file_path();
        fs::write(&check_file, "")?;
        Ok(())
    }

    fn get_check_file_path(&self) -> std::path::PathBuf {
        let config_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        config_dir.join("kvm-pro").join(".last_update_check")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_checker_creation() {
        let checker = UpdateChecker::new("yourusername/kvm-pro");
        assert_eq!(checker.github_repo, "yourusername/kvm-pro");
    }
}
