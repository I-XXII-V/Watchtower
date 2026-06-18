use serde::Serialize;

/// Normalized health status string (not emoji)
pub fn health_to_string(emoji: &str) -> String {
    match emoji {
        "✅" => "healthy".to_string(),
        "⚠️" => "warning".to_string(),
        "🔴" => "inactive".to_string(),
        "🪦" => "dead".to_string(),
        _ => "unknown".to_string(),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PackageResult {
    pub name: String,
    pub version: String,
    pub health: String,
    pub description: Option<String>,
    pub latest_version: Option<String>,
    pub stale_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub healthy: u32,
    pub warning: u32,
    pub inactive: u32,
    pub dead: u32,
    pub unknown: u32,
}

impl Summary {
    pub fn new() -> Self {
        Self {
            healthy: 0,
            warning: 0,
            inactive: 0,
            dead: 0,
            unknown: 0,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ScanOutput {
    pub ecosystem: String,
    pub packages: Vec<PackageResult>,
    pub summary: Summary,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_to_string_healthy() {
        assert_eq!(health_to_string("✅"), "healthy");
    }

    #[test]
    fn test_health_to_string_warning() {
        assert_eq!(health_to_string("⚠️"), "warning");
    }

    #[test]
    fn test_health_to_string_inactive() {
        assert_eq!(health_to_string("🔴"), "inactive");
    }

    #[test]
    fn test_health_to_string_dead() {
        assert_eq!(health_to_string("🪦"), "dead");
    }

    #[test]
    fn test_health_to_string_unknown() {
        assert_eq!(health_to_string("❓"), "unknown");
    }

    #[test]
    fn test_health_to_string_fallback() {
        assert_eq!(health_to_string("🤷"), "unknown");
    }

    #[test]
    fn test_summary_new() {
        let s = Summary::new();
        assert_eq!(s.healthy, 0);
        assert_eq!(s.warning, 0);
        assert_eq!(s.inactive, 0);
        assert_eq!(s.dead, 0);
        assert_eq!(s.unknown, 0);
    }
}
