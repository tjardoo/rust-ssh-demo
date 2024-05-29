use colored::Colorize;

pub fn print_command(command: &str) {
    println!("{} {}", "Executing:".black(), command.black(),);
}

pub fn print_exit_status(exit_status: i32, message: Option<String>) {
    if exit_status == 0 {
        println!(
            "{} {}",
            "Exit status:".black(),
            exit_status.to_string().black(),
        );
    } else {
        println!(
            "{} {}",
            "Exit status:".black(),
            exit_status.to_string().red(),
        );
    }

    if let Some(message) = message {
        if message.is_empty() {
            return;
        }

        println!("{}", message.red().italic());
    }
}
