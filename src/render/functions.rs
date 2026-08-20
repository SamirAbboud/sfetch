use crate::{
    colors,
    fetch,
    render::parser::{
        parse_bool_argument, parse_string_argument, parse_uptime_length, parse_usize_argument,
    },
};

pub fn execute_function(
    name: &str,
    arguments: &[&str],
    colorize_functions: bool,
    functions_color: usize,
    text_color: &str,
) -> String {
    match name {
        "distro" => {
            let architecture = parse_bool_argument(arguments, 0, true);
            fetch::distro(architecture)
        }

        "kernel" => {
            let small = parse_bool_argument(arguments, 0, false);
            fetch::kernel(small)
        }

        "model" => {
            let version = parse_bool_argument(arguments, 0, false);
            fetch::model(version)
        }

        "wm" => {
            let protocol = parse_bool_argument(arguments, 0, true);
            fetch::wm(protocol)
        }

        "shell" => {
            let version = parse_bool_argument(arguments, 0, true);
            fetch::shell(version)
        }

        "gpu" => {
            let full_name = parse_bool_argument(arguments, 0, true);
            let colorize = parse_bool_argument(arguments, 1, false);

            let reset_color = if colorize_functions {
                colors::get_color(functions_color, true)
            } else {
                text_color
            };

            fetch::gpu(full_name, colorize, reset_color)
        }

        "gpu_driver" => {
            let single_driver = parse_bool_argument(arguments, 0, false);

            fetch::gpu_driver(single_driver)
        }

        "cpu" => {
            let round_to = parse_usize_argument(arguments, 0, 2);
            let full_name = parse_bool_argument(arguments, 1, false);
            let colorize = parse_bool_argument(arguments, 2, false);

            fetch::cpu(round_to, full_name, colorize)
        }

        "memory" => {
            let gib = parse_bool_argument(arguments, 0, true);
            let round_to = parse_usize_argument(arguments, 1, 2);
            let colorize = parse_bool_argument(arguments, 2, false);

            let reset_color = if colorize_functions {
                colors::get_color(functions_color, true)
            } else {
                text_color
            };

            fetch::memory(gib, round_to, colorize, reset_color)
        }

        "disk" => {
            let path = parse_string_argument(arguments, 0, "/");
            let colorize = parse_bool_argument(arguments, 1, true);
            let file_system = parse_bool_argument(arguments, 2, true);
            let percent = parse_bool_argument(arguments, 3, true);
            let round_mem_to = parse_usize_argument(arguments, 4, 2);

            let reset_color = if colorize_functions {
                colors::get_color(functions_color, true)
            } else {
                text_color
            };

            fetch::disk(
                path,
                colorize,
                file_system,
                percent,
                round_mem_to,
                reset_color,
            )
        }

        "colors" | "color_palette" => {
            let background = parse_bool_argument(arguments, 0, false);
            let character = parse_string_argument(arguments, 1, " ");
            let normal_only = parse_bool_argument(arguments, 2, true);

            fetch::color_palette(background, character, normal_only)
        }

        "uptime" => {
            let up = parse_bool_argument(arguments, 0, false);
            let length = parse_uptime_length(arguments, 1);

            fetch::uptime(up, length)
        }

        "hostname" => fetch::hostname(),
        "monitor" => fetch::monitor(),
        "terminal" => fetch::terminal(),
        "packages" => fetch::packages(),
        "gtk_theme" => fetch::gtk_theme(),
        "icon_theme" => fetch::icon_theme(),
        "gtk_font" => fetch::gtk_font(),
        "cursor_theme" => fetch::cursor_theme(),
        _ => format!("{{{name}}}"),
    }
}

pub fn colorize_result(result: String, color: usize) -> String {
    format!(
        "{}{}{}",
        colors::get_color(color, true),
        result,
        colors::reset()
    )
}
