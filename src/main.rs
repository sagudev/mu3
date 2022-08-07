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
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(filename)
            .expect("Unable to open file");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Unable to read string");
        let data: String = data.lines().map(|x| decode(x).expect("UTF-8")).join("\n");
        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(data.as_bytes())
            .expect("Unable to write data");
    }
}
