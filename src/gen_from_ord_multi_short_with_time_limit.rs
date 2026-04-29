use std::{collections::HashSet, io::BufRead};
use std::fs::File;
use std::io::{BufReader};
use std::time::{Duration, Instant};

fn main() {

    let args_len = std::env::args().len();

    if args_len < 4 {
        println!("Usage: {} <limit in seconds> <pord_num> <task_file>", std::env::args().next().unwrap());
        return;
    }

    let mut time_limit = 0u64;
    
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {time_limit = val},
        Err(_e) => println!("First argument must be a number.")
    }

    let mut pord_num = 0usize;
    
    match std::env::args().nth(2).unwrap().parse() {
        Ok(val) => {pord_num = val},
        Err(_e) => println!("First argument must be a number.")
    }

    let task_file_path = std::env::args().nth(3).unwrap();

    let n = 8usize;
    let ord_filename =format!("ord{}_with_top.txt", n);
    let file = BufReader::new(File::open(ord_filename).expect("Cannot open file"));

    let mut cur_line_no = 0usize;
    for line in file.lines() {
        let cur_line = line.unwrap();
        cur_line_no+=1;
        

        if cur_line_no == pord_num {
            let task_file = BufReader::new(File::open(&task_file_path).expect("Cannot open file"));
            let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
            let pord = serde_json::from_str::<Vec<Vec<usize>>>(&cur_line).unwrap();
            
            eprintln!("Order: {pord:?}");
            let ts: Instant = Instant::now();
            for line in task_file.lines() {
                // let mut lalg_limpl = l_alglib::l_alg_alloc_limpl(n);
                // let mut positions = Vec::<(usize,usize)>::new();


                // let mut init_vector = Vec::<usize>::new();
                let line_str = line.unwrap();
                eprintln!("Init vector (str): {}", line_str);
                let init_vector = line_str.split(",").map(|v| v.trim().parse().unwrap()).collect();
                eprintln!("Init vector (int): {:?}", init_vector);
    
                l_alglib::l_alg_gen_from_ord_short_time_with_limit(Duration::new(time_limit, 0), &pord, &init_vector, &mut lalgs, true, true);

            }
            eprintln!("===\nTotal Computational Time: {:.4} s", ts.elapsed().as_secs_f32());
        }
    }
}
