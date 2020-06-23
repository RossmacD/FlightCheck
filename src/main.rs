use exitfailure::ExitFailure;
use failure::ResultExt;
use text_io::read;
use structopt::StructOpt;
use std::process::exit;
use std::io::{self,stdout, Write};
use ansi_term::Colour::{Green,Red};
use crossterm::{QueueableCommand, cursor};
use std::{thread, time};




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
    
    let mut i = 1;
    let mut stdout = stdout();
    for line in content.lines() {
        // using the macro
    // printItem();

    

    // for i in 1..10 {
    //     stdout.queue(cursor::SavePosition);
    //     stdout.write(format!("Here!!! {}", i).as_bytes());
    //     stdout.queue(cursor::RestorePosition);
    //     stdout.flush();
    //     thread::sleep(time::Duration::from_millis(500));
    // }


        stdout.queue(cursor::SavePosition);
        stdout.write(format!("{} {}\n", Red.bold().paint("[-]"), line).as_bytes());
        stdout.flush();
        if !check_or_throw()? {
            exit(126)
        }
        stdout.queue(cursor::RestorePosition);
        stdout.write(format!("{} {}\n", Green.paint("[X]"), line).as_bytes());
        stdout.flush();
        // println!("{}  {}", Green.paint("[X]"),line);
        i += 1;
    }
    Ok(())
}

fn check_or_throw() -> Result<bool, ExitFailure> {
    let input:String = read!("{}\n");
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