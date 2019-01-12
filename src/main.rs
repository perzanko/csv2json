extern crate serde_json;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::collections::HashMap;
use std::env;
use std::process;
use std::time::Instant;

type Lines = Vec<String>;
type Nav = String;
type NavKeys = Vec<String>;
type Rows = Vec<HashMap<String, String>>;

fn main() {
    let start_time = Instant::now();
    let (input_file, output_file) = parse_args();

    if is_str_empty(&input_file) || is_str_empty(&output_file) {
      show_help();
      process::exit(0x0100);
    }

    let nav_with_lines: (Nav, Lines) = read_file(&input_file).unwrap();
    let nav: NavKeys = fetch_keys(&nav_with_lines.0);
    let rows: Rows = generate_rows(&nav_with_lines.1, &nav);
    let json = serde_json::to_string(&rows).unwrap();

    match write_file(&output_file, &json) {
      Ok(()) => println!("Ok, done! - {}", output_file),
      Err(err) => println!("Something went wrong. {}", err),
    };

    println!("Rows: {}", rows.len());
    println!("Elapsed: {} ms", get_elapsed_time(start_time));
}

fn parse_args() -> (String, String) {
  let mut input_file = String::new();
  let mut output_file = String::new();
  let mut args: Vec<String> = env::args().collect();
  args.remove(0);
  args
    .chunks(2)
    .for_each(|x| {
      match x[0].as_ref() {
        "--input" => { input_file = x[1].clone(); },
        "-i" => { input_file = x[1].clone(); },
        "--output" => { output_file = x[1].clone(); },
        "-o" => { output_file = x[1].clone(); },
        _ => { }
      }
    });

  (input_file, output_file)
}

fn read_file(file_path: &String) -> Result<(Nav, Lines), io::Error> {
    let file = File::open(file_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut lines = Vec::new();
    let mut nav = String::new();
    for (i, line) in buf_reader.lines().enumerate() {
      let line = line.unwrap();
      if i == 0 { nav = line } else { lines.push(line) }
    }
    Ok((nav, lines))
}

fn write_file(file_path: &String, data: &String) -> Result<(), io::Error> {
  let mut file = File::create(file_path)?;
  file.write_all(&data.as_bytes())?;
  Ok(())
}

fn fetch_keys(keys_str: &String) -> NavKeys {
  let mut i: i16 = -1;
  keys_str
    .split(",")
    .collect::<Vec<&str>>()
    .into_iter()
    .map(|key| { i += 1; String::from(key) })
    .collect()
}

fn generate_rows(lines: &Lines, nav: &NavKeys) -> Vec<HashMap<String, String>> {
  lines
    .into_iter()
    .map(|line| {
      let mut hash: HashMap<String, String> = HashMap::new();
      let mut i = 0;
      let nav = nav;
      line
        .split(",")
        .collect::<Vec<&str>>()
        .into_iter()
        .for_each(|x| { i += 1; hash.insert(nav[i - 1].clone(), String::from(x)); });
      hash
    })
    .collect()
}

fn show_help() {
  print!("
                                               Created by perzanko
                                                              ----
 .o88b. .d8888. db    db .d888b.    d88b .d8888.  .d88b.  d8b   db 
d8P  Y8 88'  YP 88    88 VP  `8D    `8P' 88'  YP .8P  Y8. 888o  88 
8P      `8bo.   Y8    8P    odD'     88  `8bo.   88    88 88V8o 88 
8b        `Y8b. `8b  d8'  .88'       88    `Y8b. 88    88 88 V8o88 
Y8b  d8 db   8D  `8bd8'  j88.    db. 88  db   8D `8b  d8' 88  V888 
 `Y88P' `8888Y'    YP    888888D Y8888P  `8888Y'  `Y88P'  VP   V8P 
                                                                   
This tool provides simple and efficient csv to json conversion.

Usage: csv2json --input [path] --output [path]

  -h or --help                              print this help
  -i or --input                             path of CSV file
  -o or --output                            output path of converted JSON

")
}

fn is_str_empty(text: &String) -> bool {
  if text.trim().len() == 0 { true } else { false }
}

fn get_elapsed_time(start_time: Instant) -> String {
  let x = start_time.elapsed();
  ((x.as_secs() * 1_000) + (x.subsec_nanos() / 1_000_000) as u64).to_string()
}