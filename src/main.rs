//! CLI for trying dbexec code

mod comp;
use std::env;
use std::fs;
use std::path;

const DBEXEC_EXT: &str = "dbexec";

const FILE_CONTAINS_ERROR: &str = "DBEXEC File Contains Some Error";
const INVALID_PATH: &str = "Invalid Path";
const NOT_DBEXEC_FILE: &str = "File Must End In .dbexec";
const READ_ERROR: &str = "Issue Reading File";

fn main() {
    let mut args = env::args();
    args.next();

    // Parsing path out
    let file_path: path::PathBuf = args.next()
        .expect(INVALID_PATH)
        .parse()
        .expect(INVALID_PATH);
    if file_path.extension().unwrap() != DBEXEC_EXT {
        eprintln!("{}", NOT_DBEXEC_FILE);
    }

    // Getting the actual code in correct form
    let string = fs::read_to_string(file_path).expect(READ_ERROR);
    let mut code: Vec<u16> = string.lines().map(|line| line.trim().parse().expect(FILE_CONTAINS_ERROR)).collect();
    code.resize(64, 6 << 12);
    let prog: [u16; 64] = code.try_into().expect(FILE_CONTAINS_ERROR);

    // Simulating
    let output = comp::run(prog);

    // Outputting debug info
    println!("{}", output);
}
