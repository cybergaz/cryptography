use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use colored::*;
use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

/// Prints out the text character by character with a delay of `speed` milliseconds and a color of `color` and a background color of `oncolor` if provided.
pub fn animate(text: &str, speed: u64, color: &str, oncolor: Option<&str>) {
    match oncolor {
        Some(bgcolor) => {
            for a in text.chars() {
                print!("{}", a.to_string().color(color).on_color(bgcolor).bold());
                stdout().flush().unwrap();
                sleep(Duration::from_millis(speed));
            }
            println!();
        }
        None => {
            for a in text.chars() {
                print!("{}", a.to_string().color(color).bold());
                stdout().flush().unwrap();
                sleep(Duration::from_millis(speed));
            }
            println!();
        }
    }
}

/// Configuration for `select` component from inquire crate.
pub fn config_select() -> RenderConfig {
    RenderConfig::default()
        .with_prompt_prefix(Styled::new("*").with_fg(Color::DarkGreen))
        .with_selected_option(Some(
            StyleSheet::new()
                .with_fg(Color::DarkBlue)
                .with_attr(Attributes::BOLD),
        ))
        .with_highlighted_option_prefix(Styled::new("->").with_fg(Color::DarkGrey))
        .with_help_message(StyleSheet::new().with_fg(Color::DarkGrey))
        .with_answer(StyleSheet::new().with_fg(Color::DarkGreen))
        .with_canceled_prompt_indicator(Styled::new("✖").with_fg(Color::LightRed))
}

/// configuration for `text` component from inquire crate.
pub fn config_text() -> RenderConfig {
    RenderConfig::default()
        .with_prompt_prefix(
            Styled::new("+")
                .with_fg(Color::DarkYellow)
                .with_attr(Attributes::BOLD),
        )
        .with_text_input(StyleSheet::new().with_fg(Color::LightCyan))
        .with_answer(StyleSheet::new().with_fg(Color::DarkGreen))
}

/// on interrupt handler
pub fn on_interrupt() {
    println!(
        "{} {}",
        "✖".red(),
        "Operation Aborted".bright_magenta().bold()
    );
    std::process::exit(0);
}

// #[macro_export]
// macro_rules! animate {
//     ($text:expr, $speed:expr) => {
//         for a in $text.chars() {
//             print!("{}", a.to_string().black().on_white());
//             std::io::stdout().flush().unwrap();
//             std::thread::sleep($speed);
//         }
//         println!()
//     };
// }
