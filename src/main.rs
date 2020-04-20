use structopt::StructOpt;
use std::{fs::File, io::prelude::*, io::BufReader, io::Write};
use failure::ResultExt;
use exitfailure::ExitFailure;
#[derive(StructOpt, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    let content = File::open(&args.path).with_context(|_| format!("Could not read the file {:?}", &args.path))?;
    let reader = BufReader::new(content);

    let stdout = std::io::stdout();
    let mut handle = std::io::BufWriter::new(stdout);

    for line in reader.lines() {
        writeln!(handle, "{}", line.unwrap())?;
    }
    
    handle.flush().unwrap();
    
    Ok(())

}
