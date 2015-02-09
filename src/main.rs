//
// XadillaX created at 2015-02-09 11:09:56
//
// Copyright (c) 2015 Huaban.com, all rights
// reserved
//
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

use std::old_io as io;
use std::old_io::{fs, File, Open, Truncate, Read, Write};
use std::old_io::fs::PathExtensions;
use docopt::Docopt;

static USAGE: &'static str = "
Usage: crlf2lf [options] <DIR>
       crlf2lf --help

Options:
  -r --recur
  -h --help
";

#[derive(RustcDecodable)]
struct Args {
    arg_DIR: String,
    flag_recur: bool,
    flag_help: bool
}

fn is_binary(vec: &Vec<u8>) -> bool {
    for i in 0..vec.len() {
        if vec[i] <= 8 {
            return true;
        }
    }

    return false;
}

fn crlf2lf(path: &Path) {
    let display_filename = path.display();

    print!("Converting {}... ", display_filename);

    let mut file_read;
    let mut file_write;
    match File::open_mode(path, Open, Read) {
        Ok(f) => {
            file_read = f;
        },
        Err(e) => { println!("Err - {}", e.desc); return; }
    };

    let vec;
    match file_read.read_to_end() {
        Ok(v) => {
            vec = v;
        },
        Err(e) => { println!("Err - {}", e.desc); return; }
    };

    if is_binary(&vec) {
        println!("Ignored because of binary.");
        return;
    }

    // convert \r\n to \n
    let mut another_vec: Vec<u8> = vec![];
    for i in 0..vec.len() {
        if vec[i] == 13 {
            if i < vec.len() - 1 && vec[i + 1] == 10 {
                continue;
            }

            if i > 0 && vec[i - 1] == 10 {
                continue;
            }
        }

        another_vec.push(vec[i]);
    }

    match File::open_mode(path, Truncate, Write) {
        Ok(f) => {
            file_write = f;
        },
        Err(e) => { println!("Err - {}", e.desc); return; }
    };

    // write back
    match file_write.write(&another_vec) {
        Ok(_) => { println!("Done."); return; },
        Err(e) => { println!("Err - {}", e.desc); return; }
    };
}

fn walk(dir: &Path, recur: bool) -> io::IoResult<()> {
    if dir.is_dir() {
        let contents = try!(fs::readdir(dir));
        for entry in contents.iter() {
            if entry.is_dir() {
                if(recur) {
                    try!(walk(entry, recur));
                }
            } else {
                crlf2lf(entry);
            }
        }

        return Ok(());
    } else {
        crlf2lf(dir);
        return Ok(());
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let directory: String = args.arg_DIR;
    let recursive: bool = args.flag_recur;

    let dir: Path = Path::new(directory);
    match walk(&dir, recursive) {
        Err(why) => println!("An error occurred: {}", why.desc),
        Ok(_) => println!("Done.")
    };
}

