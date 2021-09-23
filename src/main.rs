use regex::Regex;

use std::fs;
use std::fs::File;

use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // first question
    how_would_you_like_to_solve_the_problem();
    // secound question
    write_the_code_to_fill_the_square_like_this();
}

fn write_the_code_to_fill_the_square_like_this() {
    let mut counter = 1;
    let mut adder = 0;

    println!("{:-^1$}", "", 43);
    for k in 1..10 {
        print!("| ");
        for i in 1..10 {
            print!("{:<3} ", counter + adder);
            if i % 3 == 0 {
                adder += 6;
                print!("| ");
            };

            counter += 1;
        }
        if k % 3 == 0 && k != 1 {
            counter += adder;
            adder = 0;
            counter -= 6;

            print!("\n{:-^1$}", "", 43);
        } else {
            adder = 0;
            counter -= 6;
        }
        print!("\n");
    }
}

fn how_would_you_like_to_solve_the_problem() {
    let re = Regex::new(r"((\(\d{3}\) ?)|(\d{3}-))?\d{3}-\d{4}").expect("don't correct regex");

    let mut f = File::create("list.txt").expect("don't possible to make file");
    let paths = fs::read_dir("./website").expect("don't find dir");

    for path in paths {
        let odp = path.expect("data from dir isn't a path").path();

        if odp.extension().expect("don't find extension") == "html" {
            let value = odp.to_str().expect("fail convert path to str");

            if let Ok(lines) = read_lines(&odp) {
                for line in lines {
                    if let Ok(ip) = line {
                        if re.is_match(&ip) {
                            let format_value = format!("{}\n", value);
                            f.write_all(&format_value.as_bytes())
                                .expect("error convert path from String as bytes");
                            break;
                        }
                    }
                }
            }
        }
    }

    f.sync_all().expect("Wrtite to file error");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
