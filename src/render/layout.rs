use crate::{
    colors,
    config::Config,
    render::{
        functions::{colorize_result, execute_function},
        parser::{extract_expressions, parse_arguments, parse_expression},
    },
};

pub fn render_layout(config: &Config, text_color: &str) -> String {
    let mut layout = config.layout.trim_matches('\n').to_string();
    let expressions: Vec<String> = extract_expressions(&layout)
        .into_iter()
        .map(str::to_string)
        .collect();

    for expression in expressions {
        let (name, arguments) = parse_expression(&expression);
        let arguments = parse_arguments(arguments);

        let result = execute_function(
            name,
            &arguments,
            config.colorize_functions,
            config.functions_color,
            text_color,
        );
        let result = if config.colorize_functions {
            colorize_result(result, config.functions_color)
        } else {
            result
        };

        let result = format!("{result}{text_color}");

        let placeholder = format!("{{{expression}}}");
        layout = layout.replace(&placeholder, &result);
    }

    if config.lstrip_info {
        layout = layout
            .lines()
            .map(str::trim_start)
            .collect::<Vec<_>>()
            .join("\n");
    }
    let reset = colors::reset();
    format!("{text_color}{layout}{reset}")
}
