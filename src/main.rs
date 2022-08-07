use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use itertools::Itertools;
use urlencoding::decode;
use walkdir::WalkDir;

fn do_folder(path: &Path) {
    for e in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|x| x.file_type().is_file())
    {
        let path = e.path();
        println!("{path:?}");
        let ext = path
            .extension()
            .unwrap_or_else(|| std::ffi::OsStr::new(""))
            .to_str()
            .unwrap();
        if ext == "m3u" || ext == "m3u8" {
            do_file(path)
        }
    }
}

fn do_file(path: &Path) {
    let mut file = File::open(path).expect("Unable to open file");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read string");
    let final_ln = data.ends_with('\n');
    let mut data: String = data.lines().map(|x| decode(x).expect("UTF-8")).join("\n");
    if final_ln {
        data += "\n";
    }
    let mut file = File::create(path).expect("Unable to open file");
    file.write_all(data.as_bytes())
        .expect("Unable to write data");
    file.flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("NO file provided!");
    }
    for path in &args[1..] {
        let path = PathBuf::from(path);
        if path.is_file() {
            do_file(&path)
        } else if path.is_dir() {
            do_folder(&path)
        } else {
            panic!("quantum computer is not supported!")
        }
    }
}
