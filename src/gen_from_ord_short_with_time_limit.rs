use std::{collections::HashSet, io::BufRead};
use std::fs::File;
use std::io::{BufReader};
use std::time::{Duration};

fn main() {

    let args_len = std::env::args().len();

    if args_len < 3 {
        println!("Usage: {} <limit in seconds> <pord_num> [init_vec_string]", std::env::args().next().unwrap());
        return;
    }

    let mut time_limit = 0u64;
    match std::env::args().nth(1).unwrap().trim().parse() {
        Ok(val) => {time_limit = val},
        Err(_e) => println!("First argument must be a number (seconds).")
    }


    let mut pord_num = 0usize;
    match std::env::args().nth(2).unwrap().parse() {
        Ok(val) => {pord_num = val},
        Err(_e) => println!("First argument must be a number (# of pord).")
    }

    let mut init_vector = Vec::<usize>::new();
    if args_len == 4 {
        let init_vector_str = std::env::args().nth(3).unwrap();
        eprintln!("Init vector (str): {}", init_vector_str);
        init_vector = init_vector_str.split(",").map(|v| v.trim().parse().unwrap()).collect();
        eprintln!("Init vector (int): {:?}", init_vector);
    }
    // let n = 7usize;
    // let file = BufReader::new(File::open("ord7_with_top.txt").expect("Cannot open file"));
    let n = 8usize;
    let ord_filename =format!("ord{}_with_top.txt", n);
    let file = BufReader::new(File::open(ord_filename).expect("Cannot open file"));

    // let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
    let mut cur_line_no = 0usize;
    for line in file.lines() {
        let cur_line = line.unwrap();
        cur_line_no+=1;
        

        if cur_line_no == pord_num {

            let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
            // println!("{cur_line:?}");
            // let pord = l_alglib::parse_vector(&cur_line);
            let pord = serde_json::from_str::<Vec<Vec<usize>>>(&cur_line).unwrap();
            
            eprintln!("Order: {pord:?}");

            // l_alglib::l_alg_gen_from_ord_short_iter(&pord, &init_vector, &mut lalgs, true, true);
            l_alglib::l_alg_gen_from_ord_short_time_with_limit_old(Duration::new(time_limit, 0), &pord, &init_vector, &mut lalgs, true, true);           
        }
    }    
}

