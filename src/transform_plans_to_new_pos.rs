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
            let mut positions = Vec::<(usize,usize)>::new();
            let mut positions_old = Vec::<(usize,usize)>::new();
                
            l_alglib::l_alg_init_get_positions_old(&pord, &mut positions_old); 
            l_alglib::l_alg_init_get_positions_new(&pord, &mut positions);


            for line in task_file.lines() {
                let mut init_vector = line.unwrap().split(",").map(|v| v.trim().parse().unwrap()).collect();    
                let trf_init_vector = l_alglib::transform_init_vector(pord.len(), &positions_old, &positions, &init_vector);
                let mut b_first=true;
                for e in trf_init_vector {
                    if b_first {
                        b_first = false;
                    }
                    else {
                        print!(",");
                    }
                    print!("{e}");
                }
                println!();//cprintln!("{:?}", trf_init_vector);
             
            }
        }
    }
}
