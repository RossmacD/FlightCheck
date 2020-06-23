use ansi_term::Colour::{Green, Red};
use crossterm::{cursor, QueueableCommand};
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, stdout, Write};
use std::process::exit;
use std::{thread, time};
use structopt::StructOpt;
use text_io::read;

#[derive(StructOpt)]
#[structopt(
    name = "FlightCheck",
    version = "1.0",
    about = "A simple command line checklist.",
    author = "Ross MacDonald - http://RossMacD.com"
)]
struct CliArgs {
    #[structopt(parse(from_os_str), default_value = ".fcheck")]
    checklist_path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = CliArgs::from_args();
    let content = std::fs::read_to_string(&args.checklist_path).with_context(|_| {
        format!(
            "FlightCheck: Could not find file `{:?}`",
            &args.checklist_path
        )
    })?;
    let mut stdout = stdout();
    for line in content.lines() {
        stdout.queue(cursor::SavePosition);
        stdout.write(format!("{} {}\n", Red.bold().paint("[-]"), line).as_bytes());
        stdout.flush();
        if !check_or_throw()? {
            exit(126)
        }
        stdout.queue(cursor::RestorePosition);
        stdout.write(format!("{} {}\n", Green.paint("[X]"), line).as_bytes());
        stdout.flush();
    }
    Ok(())
}

fn check_or_throw() -> Result<bool, ExitFailure> {
    let input: String = read!("{}\n");
    // .with_context(|_| format!("FlightCheck: Error reading file",))?
    Ok(input == "y" || input == "Y")
}

// fn printItem() -> Result<(), ExitFailure> {
//     // using the macro
//     execute!(
//         stdout(),
//         SetForegroundColor(Color::Blue),
//         SetBackgroundColor(Color::Red),
//         Print("Styled text here."),
//         ResetColor
//     )?;
//     Ok(())
// }
