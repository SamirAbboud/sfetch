mod functions;
mod layout;
mod logo;
mod parser;

use crate::config::Config;

fn get_text_color(config: &Config, logo_color: &str) -> String {
    if config.text_color == "logo" {
        logo_color.to_string()
    } else {
        config
            .text_color
            .parse::<usize>()
            .ok()
            .map(|color| crate::colors::get_color(color, true).to_string())
            .unwrap_or_default()
    }
}

pub fn render(config: &Config) {
    let (logo, logo_color) = logo::resolve_logo(config);
    let text_color = get_text_color(config, &logo_color);
    let layout = layout::render_layout(config, &text_color);
    if !config.print_logo {
        print!("{layout}");
        return;
    }
    let output = match config.logo_position.as_str() {
        "left" => logo::render_left(&logo, &layout, config.logo_info_whitespace),
        "top" => logo::render_top(&logo, &layout, config.logo_info_whitespace),
        "bottom" => logo::render_bottom(&logo, &layout, config.logo_info_whitespace),
        "right" => logo::render_right(&logo, &layout, config.logo_info_whitespace),
        _ => {
            eprintln!("Unsupported logo position: {}", config.logo_position);
            layout
        }
    };
    print!("{output}");
}
