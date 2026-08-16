const BACKGROUND: [&str; 16] = [
    "\x1b[40m",  // black background
    "\x1b[41m",  // red background
    "\x1b[42m",  // green background
    "\x1b[43m",  // yellow background
    "\x1b[44m",  // blue background
    "\x1b[45m",  // magenta background
    "\x1b[46m",  // cyan background
    "\x1b[47m",  // white background
    "\x1b[1;40m", // bright black background
    "\x1b[1;41m", // bright red background
    "\x1b[1;42m", // bright green background
    "\x1b[1;43m", // bright yellow background
    "\x1b[1;44m", // bright blue background
    "\x1b[1;45m", // bright magenta background
    "\x1b[1;46m", // bright cyan background
    "\x1b[1;47m", // bright white background
];

const FOREGROUND: [&str; 16] = [
    "\x1b[30m",  // black
    "\x1b[31m",  // red
    "\x1b[32m",  // green
    "\x1b[33m",  // yellow
    "\x1b[34m",  // blue
    "\x1b[35m",  // magenta
    "\x1b[36m",  // cyan
    "\x1b[37m",  // white
    "\x1b[1;30m", // bright black
    "\x1b[1;31m", // bright red
    "\x1b[1;32m", // bright green
    "\x1b[1;33m", // bright yellow
    "\x1b[1;34m", // bright blue
    "\x1b[1;35m", // bright magenta
    "\x1b[1;36m", // bright cyan
    "\x1b[1;37m", // bright white
];

const RESET: &str = "\x1b[0m";

pub fn get_color(key: usize, fg: bool) -> &'static str {
    if fg {
        FOREGROUND[key]
    } else {
        BACKGROUND[key]
    }
}

pub fn reset() -> &'static str {
    RESET
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn gets_foreground_color() {
        assert_eq!(get_color(0, true), "\x1b[30m");
        assert_eq!(get_color(1, true), "\x1b[31m");
        assert_eq!(get_color(15, true), "\x1b[1;37m");
    }

    #[test]
    fn gets_background_color() {
        assert_eq!(get_color(0, false), "\x1b[40m");
        assert_eq!(get_color(1, false), "\x1b[41m");
        assert_eq!(get_color(15, false), "\x1b[1;47m");
    }

    #[test]
    fn gets_reset_color() {
        assert_eq!(reset(), "\x1b[0m");
    }
}
