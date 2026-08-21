use crate::{config::{Config, LogoConfig}, logos};

fn visible_width(text: &str) -> usize {
    let mut width = 0;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip ANSI escape sequence.
            if chars.peek() == Some(&'[') {
                chars.next();

                for ch in chars.by_ref() {
                    if ch.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            width += 1;
        }
    }

    width
}

pub fn resolve_logo(config: &Config) -> (String, String) {
    match &config.logo {
        LogoConfig::Name(name) => {
            let key = if name == "auto" { "" } else { name.as_str() };

            logos::get_logos_values(key)
        }

        LogoConfig::Custom { logo, main_color } => (
            logos::colorize_logo(logo),
            crate::colors::get_color(*main_color, true).to_string(),
        ),
    }
}

pub fn render_left(logo: &str, layout: &str, whitespace: usize) -> String {
    let logo_lines: Vec<&str> = logo.lines().collect();
    let info_lines: Vec<&str> = layout.lines().collect();

    let logo_width = logo_lines
        .iter()
        .map(|line| visible_width(line))
        .max()
        .unwrap_or(0);

    let gap = " ".repeat(whitespace);
    let leading_empty = logo_lines.iter().take_while(|line| line.is_empty()).count();

    let line_count = logo_lines.len().max(info_lines.len() + leading_empty);

    let mut output = String::new();

    for i in 0..line_count {
        let logo_line = logo_lines.get(i).copied().unwrap_or("");

        let info_line = if i >= leading_empty {
            info_lines.get(i - leading_empty).copied().unwrap_or("")
        } else {
            ""
        };

        output.push_str(logo_line);

        let padding = logo_width.saturating_sub(visible_width(logo_line));
        output.push_str(&" ".repeat(padding));
        output.push_str(&gap);
        output.push_str(info_line);
        output.push('\n');
    }

    output
}

pub fn render_right(logo: &str, layout: &str, whitespace: usize) -> String {
    let logo_lines: Vec<&str> = logo.lines().collect();
    let info_lines: Vec<&str> = layout.lines().collect();

    let info_width = info_lines
        .iter()
        .map(|line| visible_width(line))
        .max()
        .unwrap_or(0);

    let gap = " ".repeat(whitespace);
    let leading_empty = logo_lines.iter().take_while(|line| line.is_empty()).count();

    let line_count = logo_lines.len().max(info_lines.len() + leading_empty);

    let mut output = String::new();

    for i in 0..line_count {
        let logo_line = logo_lines.get(i).copied().unwrap_or("");

        let info_line = if i >= leading_empty {
            info_lines.get(i - leading_empty).copied().unwrap_or("")
        } else {
            ""
        };

        output.push_str(info_line);

        let padding = info_width.saturating_sub(visible_width(info_line));
        output.push_str(&" ".repeat(padding));
        output.push_str(&gap);

        output.push_str(logo_line);
        output.push('\n');
    }

    output
}

pub fn render_top(logo: &str, layout: &str, whitespace: usize) -> String {
    let gap = "\n".repeat(whitespace);

    format!("{logo}{gap}{layout}\n")
}

pub fn render_bottom(logo: &str, layout: &str, whitespace: usize) -> String {
    let gap = "\n".repeat(whitespace);

    format!("{layout}{gap}{logo}\n")
}
