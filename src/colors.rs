pub struct Colors {
    foreground: [String; 16],
    background: [String; 16],
    reset: String,
}

impl Colors {
    pub fn new() -> Self {
        Self {    
            background: [
                "\x1b[40m".to_string(),  // black background
                "\x1b[41m".to_string(),  // red background
                "\x1b[42m".to_string(),  // green background
                "\x1b[43m".to_string(),  // yellow background
                "\x1b[44m".to_string(),  // blue background
                "\x1b[45m".to_string(),  // magenta background
                "\x1b[46m".to_string(),  // cyan background
                "\x1b[47m".to_string(),  // white background
                "\x1b[1;40m".to_string(), // bright black background
                "\x1b[1;41m".to_string(), // bright red background
                "\x1b[1;42m".to_string(), // bright green background
                "\x1b[1;43m".to_string(), // bright yellow background
                "\x1b[1;44m".to_string(), // bright blue background
                "\x1b[1;45m".to_string(), // bright magenta background
                "\x1b[1;46m".to_string(), // bright cyan background
                "\x1b[1;47m".to_string(), // bright white background
            ],

            foreground: [
                "\x1b[30m".to_string(),  // black
                "\x1b[31m".to_string(),  // red
                "\x1b[32m".to_string(),  // green
                "\x1b[33m".to_string(),  // yellow
                "\x1b[34m".to_string(),  // blue
                "\x1b[35m".to_string(),  // magenta
                "\x1b[36m".to_string(),  // cyan
                "\x1b[37m".to_string(),  // white
                "\x1b[1;30m".to_string(), // bright black
                "\x1b[1;31m".to_string(), // bright red
                "\x1b[1;32m".to_string(), // bright green
                "\x1b[1;33m".to_string(), // bright yellow
                "\x1b[1;34m".to_string(), // bright blue
                "\x1b[1;35m".to_string(), // bright magenta
                "\x1b[1;36m".to_string(), // bright cyan
                "\x1b[1;37m".to_string(), // bright white
            ],

            reset: "\x1b[0m".to_string(),
        }
    }

    pub fn reset(&self) -> &str {
        &self.reset
    }
    
    pub fn get_color(&self, key: usize, fg: bool) -> &str {
        if fg {
            &self.foreground[key]
        } else {
            &self.background[key]
        }
    }
} 


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn gets_foreground_color() {
        let colors = Colors::new();

        assert_eq!(colors.get_color(0, true), "\x1b[30m");
        assert_eq!(colors.get_color(1, true), "\x1b[31m");
        assert_eq!(colors.get_color(15, true), "\x1b[1;37m");
    }

    #[test]
    fn gets_background_color() {
        let colors = Colors::new();

        assert_eq!(colors.get_color(0, false), "\x1b[40m");
        assert_eq!(colors.get_color(1, false), "\x1b[41m");
        assert_eq!(colors.get_color(15, false), "\x1b[1;47m");
    }

    #[test]
    fn gets_reset_color() {
        let colors = Colors::new();

        assert_eq!(colors.reset(), "\x1b[0m");
    }
}
