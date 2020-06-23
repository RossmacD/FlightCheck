use exitfailure::ExitFailure;
use failure::ResultExt;
use text_io::read;
use structopt::StructOpt;
use std::process::exit;

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
    for line in content.lines() {
        println!("[-]  {}", line);
        if !check_or_throw()? {
            exit(126)
        }
        println!("[X]  {}", line);
        i += 1;
    }
    Ok(())
}

fn check_or_throw() -> Result<bool, ExitFailure> {
    let input:String = read!("{}\n");
    // .with_context(|_| format!("FlightCheck: Error reading file",))?
    Ok(input == "y" || input == "Y")
}
