use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let args_len = std::env::args().len();

    if args_len < 2 {
        println!("Usage: {} ord_filename ordnum filename [0=min(default),1=max_repr]", std::env::args().next().unwrap());
        return;
    }
    
    let ord_filename = std::env::args().nth(1).unwrap();
    let ord_file = BufReader::new(File::open(ord_filename).expect("Cannot open `ord_filename` file"));
    let mut ord_num = 1usize;    
    match std::env::args().nth(2).unwrap().trim().parse() {
        Ok(val) => {ord_num = val},
        Err(_e) => println!("Argument `ordnum` must be a number.")
    }
    
    let filename = std::env::args().nth(3).unwrap();

    let file = BufReader::new(File::open(filename).expect("Cannot open file"));

    let mut repr_kind = 2usize;
    let mut b_min_repr = true;
    if args_len >= 5 {
        match std::env::args().nth(4).unwrap().trim().parse() {
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
            eprintln!("Unknown repr type: 0, 1 are expected.");
            return;
        }
    }

    let mut ord_idx = 0;
    let mut desired_order = Vec::<Vec<usize>>::new();
    let mut b_found = false;
    for ord_line in ord_file.lines() {
        ord_idx+=1;

        if ord_idx == ord_num {
            desired_order = serde_json::from_str::<Vec<Vec<usize>>>(&ord_line.unwrap()).unwrap();
            b_found = true;
            break;
        }
    }

    if !b_found {
        eprintln!("Order number {ord_num} does not exists.");
        return;
    }

    // eprintln!("{}, {}, {}", std::env::args().len(), b_min_repr, b_canonical);
    for line in file.lines() {
        let lalg = serde_json::from_str::<Vec<Vec<usize>>>(&line.unwrap()).unwrap();//l_alglib::parse_vector(&line.unwrap());
        
        println!("{:?}", l_alglib::l_alg_get_repr_with_target_ord(&lalg, &desired_order, b_min_repr).unwrap());
    }

    // return;
}