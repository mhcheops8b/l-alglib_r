use std::{collections::HashSet, io::BufRead};
use std::fs::File;
use std::io::{BufReader};
use std::time::Instant;

fn main() {
    
    let args_len = std::env::args().len();

    if args_len < 4 {
        println!("Usage: {} <pord_filename> <pord_num> <task_file>", std::env::args().next().unwrap());
        return;
    }

    let file = BufReader::new(File::open(std::env::args().nth(1).unwrap()).expect("Cannot open file"));

    let pord_num = match std::env::args().nth(2).unwrap().parse::<usize>() {
        Ok(val) => val,
        Err(_e) => {eprintln!("First argument must be a number."); return;}
    };

    let task_file_path = std::env::args().nth(3).unwrap();

    let mut cur_line_no = 0usize;
    for line in file.lines() {
        let cur_line = line.unwrap();
        cur_line_no += 1;
        

        if cur_line_no == pord_num {
            let ts = Instant::now();
            let task_file = BufReader::new(File::open(&task_file_path).expect("Cannot open file"));
            let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
            let pord = serde_json::from_str::<Vec<Vec<usize>>>(&cur_line).unwrap();

            eprintln!("Order: {pord:?}");

            for line in task_file.lines() {
                let line_str = line.unwrap();
                eprintln!("Init vector (str): {}", line_str);
                let init_vector = line_str.split(",").map(|v| v.trim().parse().unwrap()).collect();
                eprintln!("Init vector (int): {:?}", init_vector);
    
                l_alglib::l_alg_gen_from_ord(&pord, &init_vector, &mut lalgs, true, true);
            }
            eprintln!("===\nTotal Computational Time: {:.4} s", ts.elapsed().as_secs_f32());
        }
    }
}
