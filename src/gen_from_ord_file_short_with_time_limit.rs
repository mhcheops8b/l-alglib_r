use std::{collections::HashSet, io::BufRead};
use std::fs::File;
use std::io::{BufReader};
use std::time::{Duration};

fn main() {

    let args_len = std::env::args().len();

    if args_len < 4 {
        println!("Usage: {} <limit in seconds> <pord_filename> <pord_num> [init_vec_string]", std::env::args().next().unwrap());
        return;
    }

    let time_limit = match std::env::args().nth(1).unwrap().trim().parse::<u64>() {
        Ok(val) => val,
        Err(_e) => {println!("First argument must be a number (seconds)."); return;}
    };
       
    let file = BufReader::new(File::open(std::env::args().nth(2).unwrap()).expect("Cannot open file"));

    let mut pord_num = 0usize;
    match std::env::args().nth(3).unwrap().parse() {
        Ok(val) => {pord_num = val},
        Err(_e) => println!("Third argument must be a number (# of pord).")
    }

    let mut init_vector = Vec::<usize>::new();
    if args_len == 5 {
        let init_vector_str = std::env::args().nth(4).unwrap();
        eprintln!("Init vector (str): {}", init_vector_str);
        init_vector = init_vector_str.split(",").map(|v| v.trim().parse().unwrap()).collect();
        eprintln!("Init vector (int): {:?}", init_vector);
    }

    let mut cur_line_no = 0usize;
    for line in file.lines() {
        let cur_line = line.unwrap();
        cur_line_no += 1;
        
        if cur_line_no == pord_num {

            let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
            // println!("{cur_line:?}");
            // let pord = l_alglib::parse_vector(&cur_line);
            let pord = serde_json::from_str::<Vec<Vec<usize>>>(&cur_line).unwrap();
            
            eprintln!("Order: {pord:?}");

            
            l_alglib::l_alg_gen_from_ord_short_time_with_limit_old(Duration::new(time_limit, 0), 
                      &pord, &init_vector, &mut lalgs, true, true);
            
        }
    }    
}

