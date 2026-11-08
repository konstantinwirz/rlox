use std::process::{ExitCode, Termination};

use clap::{Parser, ValueEnum};
use rlox::{Interpreter, Lexer, LoxError, LoxErrorKind, Parser as LoxParser};

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Command {
    /// Print the tokens of the source file
    Tokenize,
    /// Print the syntax tree of the source file
    Parse,
    /// Run the source file as a program
    Run,
}

#[derive(Parser, Debug)]
struct Cli {
    cmd: Command,

    #[arg(short, long, value_name = "FILE")]
    file_name: String,
}

fn main() -> impl Termination {
    let cli = Cli::parse();

    let file_content = match std::fs::read_to_string(&cli.file_name) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    match &cli.cmd {
        Command::Tokenize => Lexer::new(&file_content)
            .map(|result| handle_result(result.map(Some), |item| println!("{item}")))
            .max()
            .map_or(ExitCode::SUCCESS, ExitCode::from),
        Command::Parse => LoxParser::new(&file_content)
            .map(|result| handle_result(result, |item| println!("{item}")))
            .max()
            .map_or(ExitCode::SUCCESS, ExitCode::from),
        Command::Run => {
            let mut interpreter = Interpreter::new(LoxParser::new(&file_content));
            ExitCode::from(handle_result(interpreter.run().map(Some), |_| {}))
        }
    }
}

fn handle_result<R>(result: Result<Option<R>, LoxError>, ok_fn: fn(R) -> ()) -> u8 {
    match result {
        Ok(Some(item)) => {
            ok_fn(item);
            0
        }
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{err}");
            match err.kind {
                LoxErrorKind::ScanError => 65,
                LoxErrorKind::ParseError => 65,
                LoxErrorKind::RuntimeError => 70,
            }
        }
    }
}
