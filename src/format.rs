pub fn format_duration(ms: u64) -> String {
    let total_secs = ms / 1000;
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    if hours > 0 {
        format!("{hours}h{mins:02}m")
    } else {
        format!("{mins}m")
    }
}

pub fn format_tokens(count: u64) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}k", count as f64 / 1_000.0)
    } else {
        format!("{count}")
    }
}

pub fn shorten_path(path: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    let shortened = if !home.is_empty() && path.starts_with(&home) {
        format!("~{}", &path[home.len()..])
    } else {
        path.to_string()
    };

    let parts: Vec<&str> = shortened.split('/').collect();
    if parts.len() <= 3 {
        return shortened;
    }
    let last_two = &parts[parts.len() - 2..];
    format!("…/{}", last_two.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // format_duration tests
    #[test]
    fn test_format_duration_zero_ms() {
        assert_eq!(format_duration(0), "0m");
    }

    #[test]
    fn test_format_duration_boundary_59999ms() {
        assert_eq!(format_duration(59999), "0m");
    }

    #[test]
    fn test_format_duration_one_minute() {
        assert_eq!(format_duration(60000), "1m");
    }

    #[test]
    fn test_format_duration_max_minutes() {
        assert_eq!(format_duration(3599999), "59m");
    }

    #[test]
    fn test_format_duration_one_hour() {
        assert_eq!(format_duration(3600000), "1h00m");
    }

    #[test]
    fn test_format_duration_one_hour_thirty_minutes() {
        assert_eq!(format_duration(5400000), "1h30m");
    }

    #[test]
    fn test_format_duration_full_day() {
        assert_eq!(format_duration(86400000), "24h00m");
    }

    // format_tokens tests
    #[test]
    fn test_format_tokens_zero() {
        assert_eq!(format_tokens(0), "0");
    }

    #[test]
    fn test_format_tokens_under_thousand() {
        assert_eq!(format_tokens(999), "999");
    }

    #[test]
    fn test_format_tokens_one_thousand() {
        assert_eq!(format_tokens(1000), "1.0k");
    }

    #[test]
    fn test_format_tokens_one_thousand_five_hundred() {
        assert_eq!(format_tokens(1500), "1.5k");
    }

    #[test]
    fn test_format_tokens_near_million() {
        assert_eq!(format_tokens(999999), "1000.0k");
    }

    #[test]
    fn test_format_tokens_one_million() {
        assert_eq!(format_tokens(1000000), "1.0M");
    }

    #[test]
    fn test_format_tokens_one_million_five_hundred_thousand() {
        assert_eq!(format_tokens(1500000), "1.5M");
    }

    // shorten_path tests
    #[test]
    fn test_shorten_path_short_path() {
        let path = "a/b/c";
        let result = shorten_path(path);
        assert!(!result.contains("…"));
    }

    #[test]
    fn test_shorten_path_long_path_with_truncation() {
        let path = "/home/user/projects/rust/claude-bar";
        let result = shorten_path(path);
        assert!(result.contains("…"));
        assert!(result.ends_with("claude-bar"));
    }

    #[test]
    fn test_shorten_path_long_path_preserves_last_two() {
        let path = "/home/user/a/b/c/d/e/f/g/h/i";
        let result = shorten_path(path);
        assert!(result.contains("…"));
        assert!(result.ends_with("h/i"));
    }
}
