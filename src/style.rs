//! ANSI style parsing for config system

/// Parse a space-separated style string into ANSI escape codes.
///
/// Supports 8 colors (black, red, green, yellow, blue, magenta, cyan, white)
/// and 4 modifiers (bold, dim, italic, underline).
/// Unknown tokens are silently ignored. Empty string returns empty string.
pub fn parse_style(style_str: &str) -> String {
    let mut result = String::new();

    for token in style_str.split_whitespace() {
        match token {
            // Foreground colors
            "black" => result.push_str("\x1b[30m"),
            "red" => result.push_str("\x1b[31m"),
            "green" => result.push_str("\x1b[32m"),
            "yellow" => result.push_str("\x1b[33m"),
            "blue" => result.push_str("\x1b[34m"),
            "magenta" => result.push_str("\x1b[35m"),
            "cyan" => result.push_str("\x1b[36m"),
            "white" => result.push_str("\x1b[37m"),
            // Modifiers
            "bold" => result.push_str("\x1b[1m"),
            "dim" => result.push_str("\x1b[2m"),
            "italic" => result.push_str("\x1b[3m"),
            "underline" => result.push_str("\x1b[4m"),
            // Unknown tokens are silently ignored
            _ => {}
        }
    }

    result
}

/// Apply a style string to content, wrapping with ANSI codes and reset.
///
/// If style_str is empty, returns content unchanged (no ANSI wrapping).
/// Otherwise, wraps content with the parsed style codes and adds reset code.
pub fn apply_style(content: &str, style_str: &str) -> String {
    if style_str.is_empty() {
        content.to_string()
    } else {
        format!("{}{}\x1b[0m", parse_style(style_str), content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_color_cyan() {
        assert_eq!(parse_style("cyan"), "\x1b[36m");
    }

    #[test]
    fn parse_single_modifier_dim() {
        assert_eq!(parse_style("dim"), "\x1b[2m");
    }

    #[test]
    fn parse_color_bold() {
        let result = parse_style("bold cyan");
        assert!(result.contains("\x1b[1m"));
        assert!(result.contains("\x1b[36m"));
    }

    #[test]
    fn parse_empty_string() {
        assert_eq!(parse_style(""), "");
    }

    #[test]
    fn parse_unknown_token() {
        assert_eq!(parse_style("unknown"), "");
    }

    #[test]
    fn parse_mixed_valid_invalid() {
        let result = parse_style("bold unknown cyan");
        assert!(result.contains("\x1b[1m"));
        assert!(result.contains("\x1b[36m"));
        assert!(!result.contains("unknown"));
    }

    #[test]
    fn apply_style_with_content() {
        assert_eq!(apply_style("hello", "cyan"), "\x1b[36mhello\x1b[0m");
    }

    #[test]
    fn apply_style_empty_style() {
        assert_eq!(apply_style("hello", ""), "hello");
    }

    #[test]
    fn apply_style_with_modifiers() {
        assert_eq!(
            apply_style("hello", "bold dim"),
            "\x1b[1m\x1b[2mhello\x1b[0m"
        );
    }

    #[test]
    fn matches_render_constants() {
        // From render.rs:10-17
        const CYAN: &str = "\x1b[36m";
        const GREEN: &str = "\x1b[32m";
        const YELLOW: &str = "\x1b[33m";
        const RED: &str = "\x1b[31m";
        const DIM: &str = "\x1b[2m";

        assert_eq!(parse_style("cyan"), CYAN);
        assert_eq!(parse_style("green"), GREEN);
        assert_eq!(parse_style("yellow"), YELLOW);
        assert_eq!(parse_style("red"), RED);
        assert_eq!(parse_style("dim"), DIM);
    }
}
