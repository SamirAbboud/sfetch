pub fn run_command(command: &str, args: &[&str]) -> String {
    let output = std::process::Command::new(command)
        .args(args)
        .output();

    match output {
        Ok(output) => String::from_utf8(output.stdout)
            .unwrap_or_default()
            .trim()
            .to_string(),
        Err(_) => String::new(),
    }
}

pub fn command_exists(command: &str) -> bool {
    if let Ok(path) = std::env::var("PATH") {
        for directory in path.split(':') {
            let path = std::path::Path::new(directory).join(command);

            if path.is_file() {
                return true;
            }
        }
    }

    false
}

pub fn capitalize(value: &str) -> String {
    value
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();

            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn make_me_pretty(value: &str) -> String {
    value.trim().replace(['"', '\''], "")
}






















