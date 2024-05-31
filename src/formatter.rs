use crate::{cli::Command, commands::info::InfoCommand, formatters, OptionalFormatter};

pub fn find_formatter(command: &Command) -> OptionalFormatter {
    match command {
        Command::Info(info_command) => match info_command {
            InfoCommand::ServiceInstalledCheck { service: _ } => {
                wrap_formatter(formatters::info::service_installed_check::format)
            }
            InfoCommand::Temperature => wrap_formatter(formatters::info::temperature::format),
            _ => None,
        },
        _ => None,
    }
}

fn wrap_formatter<F>(formatter: F) -> OptionalFormatter
where
    F: Fn(&str) -> String + 'static,
{
    Some(Box::new(formatter))
}
