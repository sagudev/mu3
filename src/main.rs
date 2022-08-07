use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use itertools::Itertools;
use urlencoding::decode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("NO file provided!");
    }
    for filename in &args[1..] {
        let mut file = File::open(&filename).expect("Unable to open file");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Unable to read string");
        let final_ln = data.ends_with('\n');
        let mut data: String = data.lines().map(|x| decode(x).expect("UTF-8")).join("\n");
        if final_ln {
            data += "\n";
        }
        let mut file = File::create(&filename).expect("Unable to open file");
        file.write_all(data.as_bytes())
            .expect("Unable to write data");
        file.flush().unwrap();
    }
}
