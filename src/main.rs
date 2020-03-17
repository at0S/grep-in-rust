use structopt::StructOpt;
use std::io::BufRead;

#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(short="h", long="help")]
    print_help: bool,

    #[structopt(short="v", long="version")]
    print_version: bool,
}

fn main() {
    let args = Cli::from_args();
    let f = std::fs::File::open(&args.path)
        .expect("Can't read file");

    let reader = std::io::BufReader::new(f);
    for l in reader.lines() {
        let line = l.unwrap();
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}