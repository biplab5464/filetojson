use clap::Parser;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;
use json::{stringify, stringify_pretty, JsonValue};

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
    ///print in compacted form in json, if not provided it print in pretty from
    #[arg(long)]
    compact: bool,
    ///Output directory for the file
    #[arg(long, short)]
    output: Option<String>,
}


fn main() {
    let args = Command::parse();

    //println!("got it = {:?}",args);

    let path = Path::new(&args.dir);
    
    if !path.exists() || path.is_file(){
        panic!("Please make sure the Directory exist and it is directory");
    }

    let mut json_array = JsonValue::new_array();

    for entry in path.read_dir().expect("seems like a permission issue"){
        match entry {
            Err(err) => panic!("Prolbem with reading the file {}",err),
            Ok(file_path) =>{
                let file = read_to_string(file_path.path()).expect("Seems like a permission issue");
                let json: JsonValue =
                json::parse(&file).expect("Unable to read json,\n Please check th json file");
                if !args.quite{
                    println!("added {:?}",file_path.path());
                }
                json_array.push(json).expect("the current file is not a json file");
            }
        }
    }

    let obj_string = match args.compact {
        true => stringify(json_array),
        false => stringify_pretty(json_array, 2),
    };

    let name = match args.output {
        None => "template.json".to_string(),
        Some(tmp_path) => tmp_path,
    };
    
    let mut write_file = File::create(name).expect("problem with opening the file maybe present before");
    write_file
            .write_all(obj_string.as_bytes())
            .expect("Probem with writeing the file");

}
