use std::env;
use std::fs::File;
use std::io::Read;

use itertools::Itertools;
use urlencoding::decode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(filename) = args.get(1) {
        let mut file = File::open(&filename).expect("Unable to open file");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Unable to read string");
        let data: String = data.lines().map(|x| decode(x).expect("UTF-8")).join("\n");
        println!("{}", data);
    } else {
        panic!("NO file provided!")
    }
}
