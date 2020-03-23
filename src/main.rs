use structopt::StructOpt;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String,
    
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(short="h", long="help")]
    help: bool,

    #[structopt(short="V", long="version")]
    version: bool,

    #[structopt(short="v", long="verbose")]
    verbose: bool,
}

fn main() {
    let args = Cli::from_args();
    let f = File::open(&args.path)
        .expect("Can't read file");

    let reader = std::io::BufReader::new(f);
    for l in reader.lines() {
        let line = l.unwrap();
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}