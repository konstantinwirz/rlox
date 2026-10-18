use clap::{Parser, ValueEnum};
use rlox::{Lexer, Parser as LoxParser};

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Command {
    /// Print the tokens of the source file
    Tokenize,
    /// Print the syntax tree of the source file
    Parse,
    /// Evaluate an expression and print its result
    Eval,
    /// Run the source file as a program
    Run,
}

#[derive(Parser, Debug)]
struct Cli {
    cmd: Command,

    #[arg(short, long, value_name = "FILE")]
    file_name: String,
}

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let file_content = std::fs::read_to_string(&cli.file_name)?;

    match &cli.cmd {
        Command::Tokenize => {
            let lexer = Lexer::new(&file_content);
            for scan_result in lexer {
                match &scan_result {
                    Ok(token) => println!("{token}"),
                    Err(err) => eprintln!("{err}"),
                }
            }
        }
        Command::Parse => {
            let parser = LoxParser::new(&file_content);
            for parse_result in parser {
                match &parse_result {
                    Ok(expr) => println!("{expr}"),
                    Err(err) => eprint!("{err}"),
                }
            }
        }
        _ => todo!(),
    };

    Ok(())
}
