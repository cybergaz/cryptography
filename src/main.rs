use colored::*;
use cryptography::{
    algo_store::ALGORITHMS,
    utils::{animate, config_select, on_interrupt},
};
use inquire::{Confirm, Select};

fn main() {
    // clearing out the terminal
    let _status = std::process::Command::new("clear").status();

    // welcome message
    animate(
        " Welcome to the cryptography playground ",
        20,
        "black",
        Some("cyan"),
    );
    println!();

    loop {
        // selecting an algorithm
        let selected_algo = Select::new("Select an algorithm", ALGORITHMS.to_vec())
            .with_render_config(config_select())
            .prompt();
        println!();

        // error handling for selecting an algorithm
        match selected_algo {
            Ok(result) => {
                println!("About {} :", result.name.bright_magenta().bold());
                animate(format!("{}", result.description).as_str(), 12, "grey", None);
                println!();
                (result.execute)()
            }
            Err(_e) => {
                on_interrupt();
            }
        }

        println!();
        // asking if user wants to start again
        let restart = Confirm::new("Do you want to start again? ")
            .with_default(true)
            .prompt();
        println!();

        // error handling for restart confirmation
        match restart {
            Ok(value) => {
                if !value {
                    break;
                }
            }
            Err(_e) => {
                on_interrupt();
            }
        }
    }
}
