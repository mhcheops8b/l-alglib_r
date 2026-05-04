use std::{collections::HashSet, io::BufRead};
use std::fs::File;
use std::io::{BufReader};


fn main() {

    let args_len = std::env::args().len();

    if args_len < 3 {
        println!("Usage: {} <pord_filename> <pord_num> [init_vec_string]", std::env::args().next().unwrap());
        return;
    }

    let ord_filename = std::env::args().nth(1).unwrap();
    let file = BufReader::new(File::open(ord_filename).expect("Cannot open pord file."));

    let pord_num:usize = match std::env::args().nth(2).unwrap().parse::<usize>() {
        Ok(val) => val,
        Err(_e) => {eprintln!("Second argument must be a number."); return}
    };

    let mut init_vector = Vec::<usize>::new();
    if args_len == 4 {
        let init_vector_str = std::env::args().nth(3).unwrap();
        eprintln!("Init vector (str): {}", init_vector_str);
        init_vector = init_vector_str.split(",").map(|v| v.trim().parse().unwrap()).collect();
        eprintln!("Init vector (int): {:?}", init_vector);
    }

    let mut cur_line_no = 0usize;
    for line in file.lines() {
        let cur_line = line.unwrap();
        cur_line_no+=1;
        
        if cur_line_no == pord_num {

            let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
            let pord = serde_json::from_str::<Vec<Vec<usize>>>(&cur_line).unwrap();
            
            eprintln!("Order: {pord:?}");

            l_alglib::l_alg_gen_from_ord_short_time(&pord, &init_vector, &mut lalgs, true, true);            
        }
    }    
}

