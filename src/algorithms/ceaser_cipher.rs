use crate::utils::{animate, config_select, config_text, on_interrupt};
use colored::*;
use inquire::{validator::Validation, Select, Text};

enum InputType {
    Ptext,
    Ctext,
    Shift,
}

enum Crypt {
    Encrypt,
    Decrypt,
}

use Crypt::*;

pub fn execute() {
    let action = Select::new("Select an action ", vec!["Encrypt", "Decrypt"])
        .with_render_config(config_select())
        .prompt();

    match action {
        Ok(value) => {
            if value == "Encrypt" {
                let text = read_input(InputType::Ptext);
                let shift = read_input(InputType::Shift);
                crypt(text, shift.parse::<u8>().unwrap(), Encrypt);
            } else {
                let text = read_input(InputType::Ctext);
                let shift = read_input(InputType::Shift);
                crypt(text, shift.parse::<u8>().unwrap(), Decrypt);
            }
        }
        Err(_e) => on_interrupt(),
    }
}

fn crypt(text: String, shift: u8, crypt: Crypt) {
    let mut crypted_text = String::new();

    for character in text.chars() {
        if character.is_ascii_alphabetic() {
            let shifted_char = match crypt {
                Encrypt => {
                    let mut shifted_char = character as u8 + shift;
                    if shifted_char > 122 {
                        shifted_char -= 26;
                    }
                    shifted_char
                }
                Decrypt => {
                    let mut shifted_char = character as u8 - shift;
                    if shifted_char < 97 {
                        shifted_char += 26;
                    }
                    shifted_char
                }
            };
            crypted_text.push(shifted_char as char);
        } else {
            crypted_text.push(character);
        }
    }
    println!();
    print!("{} {} ", "Encrypted Text".yellow(), ":");
    animate(format!("{}", crypted_text).as_str(), 15, "blue", None);
}

fn read_input(variant: InputType) -> String {
    let validator = |input: &str| match input.chars().find(|c| c.is_alphabetic()) {
        Some(_) => Ok(Validation::Invalid(
            "You might have accidentaly inserted some characters, only digits are allowed".into(),
        )),
        None => {
            if input.is_empty() {
                return Ok(Validation::Invalid("Please enter something".into()));
            }
            if input.parse::<usize>().unwrap() < 26 {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Please enter a number less than 26".into(),
                ))
            }
        }
    };

    let text = match variant {
        InputType::Ptext => Text::new("Enter the Plain Text : ")
            .with_placeholder("type something...")
            .with_render_config(config_text())
            .prompt(),
        InputType::Ctext => Text::new("Enter the Cipher Text : ")
            .with_placeholder("type something...")
            .with_render_config(config_text())
            .prompt(),
        InputType::Shift => Text::new("Enter the Shift : ")
            .with_placeholder("type something...")
            .with_validator(validator)
            .with_render_config(config_text())
            .prompt(),
    };

    match text {
        Ok(value) => return value,
        Err(_e) => {
            on_interrupt();
            return "...".to_string();
        }
    }
}
