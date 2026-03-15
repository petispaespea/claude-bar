use std::borrow::Cow;

fn single_style(token: &str) -> Option<&'static str> {
    match token {
        "black" => Some("\x1b[30m"),
        "red" => Some("\x1b[31m"),
        "green" => Some("\x1b[32m"),
        "yellow" => Some("\x1b[33m"),
        "blue" => Some("\x1b[34m"),
        "magenta" => Some("\x1b[35m"),
        "cyan" => Some("\x1b[36m"),
        "white" => Some("\x1b[37m"),
        "bold" => Some("\x1b[1m"),
        "dim" => Some("\x1b[2m"),
        "italic" => Some("\x1b[3m"),
        "underline" => Some("\x1b[4m"),
        _ => None,
    }
}

pub fn parse_style(style_str: &str) -> Cow<'static, str> {
    if style_str.is_empty() {
        return Cow::Borrowed("");
    }
    if let Some(code) = single_style(style_str) {
        return Cow::Borrowed(code);
    }
    let mut result = String::new();
    for token in style_str.split_whitespace() {
        if let Some(code) = single_style(token) {
            result.push_str(code);
        }
    }
    Cow::Owned(result)
}

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

    #[test]
    fn single_tokens_are_borrowed() {
        assert!(matches!(parse_style(""), Cow::Borrowed(_)));
        assert!(matches!(parse_style("dim"), Cow::Borrowed(_)));
        assert!(matches!(parse_style("cyan"), Cow::Borrowed(_)));
        assert!(matches!(parse_style("bold"), Cow::Borrowed(_)));
    }

    #[test]
    fn compound_styles_are_owned() {
        assert!(matches!(parse_style("bold cyan"), Cow::Owned(_)));
        assert!(matches!(parse_style("red dim"), Cow::Owned(_)));
    }
}
