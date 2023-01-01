
use std::fs::File;
use std::io::Read;

fn main() {

    let json_path = "./test/sample-execute.json";
    let mut json = File::open(json_path).unwrap();

    let mut buffer = String::new();
    json.read_to_string(&mut buffer);

    println!("{}", buffer);

}

