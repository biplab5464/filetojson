use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    version ="1.0.0",
    about = "The tool is used to convert JSON files into one JSON Array"
)]
struct Command{
    ///Directory where json is present
    dir : String,
    ///no print to the stdout
    #[arg(long)]
    quite: bool,
    ///print in formated form in json, if not provided it print in compact from
    #[arg(long)]
    pretty: bool,
    ///Output directory fir the file
    #[arg(long, short)]
    output: Option<String>,
}


fn main() {
    let args = Command::parse();

    println!("got it = {:?}",args);

    let path = Path::new(&args.dir);
    
    if !path.exists(){
        panic!("Please make sure the Directory exist");
    }

    for entry in fs::read_dir(path).unwrap() {
        println!("{}",entry.unwrap().path().display());
    }

}
