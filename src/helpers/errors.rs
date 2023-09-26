use crate::program::Program;
use colored::Colorize;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Error {
    Syntax,
    NotImplemented,
    FileNotFound,
    Type,
    UnknownToken,
    UnexpectedToken,
    Critical,
    TooCharactersOnLine,
    ForbiddenCharacter,
    WrongFormat,
}

fn error_to_string(error: &Error) -> &'static str {
    match error {
        Error::Syntax => "Syntax",
        Error::NotImplemented => "NotImplemented",
        Error::FileNotFound => "FileNotFound",
        Error::Type => "Type",
        Error::UnknownToken => "UnknownToken",
        Error::UnexpectedToken => "UnexpectedToken",
        Error::Critical => "Critical",
        Error::TooCharactersOnLine => "TooCharactersOnLine",
        Error::ForbiddenCharacter => "ForbiddenCharacter",
        Error::WrongFormat => "WrongFormat",
    }
}

// fn get_code_number(kind: &Error) -> u8 {
//     match kind {
//         Error::Syntax => 1,
//         Error::NotImplemented => 2,
//         Error::FileNotFound => 1,
//         Error::Type => 1,
//         Error::UnknownToken => 1,
//         Error::UnexpectedToken => 1,
//         Error::Critical => 2,
//         Error::TooCharactersOnLine => 1,
//         Error::ForbiddenCharacter => 1,
//         Error::WrongFormat => 1,
//     }
// }

fn to_stderr(program: &Program, kind: &Error, message: String, is_warning: bool) -> String {
    let location = format!(
        "{} {}:{}:{}",
        "-->".blue(),
        program.get_filename(),
        program.get_name(),
        program.get_line()
    );
    format!(
        "{}: {}: `{}`\n{}",
        if is_warning {
            "warning".yellow()
        } else {
            "error".red()
        },
        error_to_string(kind).cyan(),
        message,
        location,
    )
}

pub fn raise(program: &Program, kind: Error, message: String) {
    let stderr: String = to_stderr(program, &kind, message, false);
    eprintln!("{}", stderr);
    // std::process::exit();
}

pub fn warn(program: &Program, kind: Error, message: String) {
    let stderr: String = to_stderr(program, &kind, message, true);
    println!("{}", stderr);
}
