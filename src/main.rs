use structopt::StructOpt;
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
   
    println!("{:#?}", args);
}
