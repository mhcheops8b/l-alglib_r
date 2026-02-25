use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let args_len = std::env::args().len();

    if args_len < 2 {
        println!("Usage: {} filename [0=min(default),1=max_repr]", std::env::args().next().unwrap());
        return;
    }
    let filename = std::env::args().nth(1).unwrap();

    let file = BufReader::new(File::open(filename).expect("Cannot open file"));

    let mut repr_kind = 0usize;
    let mut b_min_repr = true;
    if args_len == 3 {
        match std::env::args().nth(1).unwrap().parse() {
            Ok(val) => {repr_kind = val},
            Err(_e) => println!("First argument must be a number.")
        }
        if repr_kind == 0 {
            b_min_repr = true;
        }
        else if repr_kind == 1 {
            b_min_repr = false;
        }
        else {
            eprintln!("Uknown repr type: 0, 1 are expected.");
            return;
        }
    }

    for line in file.lines() {
        let lalg = l_alglib::parse_vector(&line.unwrap());
        
        println!("{:?}", l_alglib::l_alg_get_repr(&lalg, b_min_repr));
    }

    // return;
}