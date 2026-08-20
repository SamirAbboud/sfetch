use crate::fetch::UptimeLength;

pub fn extract_expressions(layout: &str) -> Vec<&str> {
    let mut expressions = Vec::new();
    let mut remaining = layout;

    while let Some(start) = remaining.find('{') {
        let after_start = &remaining[start + 1..];

        let Some(end) = after_start.find('}') else {
            break;
        };

        expressions.push(&after_start[..end]);
        remaining = &after_start[end + 1..];
    }

    expressions
}

pub fn parse_expression(expression: &str) -> (&str, &str) {
    let Some(open) = expression.find('(') else {
        return (expression, "");
    };

    let name = &expression[..open];
    let arguments = &expression[open + 1..expression.len() - 1];

    (name, arguments)
}

pub fn parse_arguments(arguments: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;

    for (index, character) in arguments.char_indices() {
        match character {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                result.push(arguments[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }

    if start < arguments.len() {
        result.push(arguments[start..].trim());
    }

    result
}

pub fn parse_bool_argument(arguments: &[&str], index: usize, default: bool) -> bool {
    arguments
        .get(index)
        .and_then(|arg| arg.parse::<bool>().ok())
        .unwrap_or(default)
}

pub fn parse_usize_argument(arguments: &[&str], index: usize, default: usize) -> usize {
    arguments
        .get(index)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or(default)
}

pub fn parse_string_argument<'a>(
    arguments: &'a [&'a str],
    index: usize,
    default: &'a str,
) -> &'a str {
    arguments
        .get(index)
        .map(|arg| arg.trim_matches(['"', '\'']))
        .unwrap_or(default)
}

pub fn parse_uptime_length(arguments: &[&str], index: usize) -> UptimeLength {
    match arguments.get(index).copied() {
        Some("full") => UptimeLength::Full,
        Some("medium") => UptimeLength::Medium,
        Some("short") => UptimeLength::Short,
        _ => UptimeLength::Full,
    }
}
