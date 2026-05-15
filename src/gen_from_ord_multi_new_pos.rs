use std::{collections::HashSet, io::BufRead};
use std::fs::File;
use std::io::{BufReader};
use std::time::Instant;

fn main() {
    
    let args_len = std::env::args().len();

    if args_len < 4 {
        println!("Usage: {} <pord_num> <need_transform> <task_file>", std::env::args().next().unwrap());
        return;
    }

    let mut pord_num = 0usize;
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {pord_num = val},
        Err(_e) => println!("First argument must be a number.")
    }

    let mut b_need_transform = false;
    match std::env::args().nth(2).unwrap().parse() {
        Ok(val) => {match val {0=> b_need_transform = false, 1=> b_need_transform = true, _=> {eprintln!("Argument must be 0 or 1."); return;}}},
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
            let ts: Instant = Instant::now();
            let task_file = BufReader::new(File::open(&task_file_path).expect("Cannot open file"));
            let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
            let pord = serde_json::from_str::<Vec<Vec<usize>>>(&cur_line).unwrap();
            
            eprintln!("Order: {pord:?}");
            let mut positions = Vec::<(usize,usize)>::new();
            let mut positions_old = Vec::<(usize,usize)>::new();
                
            l_alglib::l_alg_init_get_positions_old(&pord, &mut positions_old); 
            l_alglib::l_alg_init_get_positions_new(&pord, &mut positions);
                       
            for line in task_file.lines() {
                // let mut lalg_limpl = l_alglib::l_alg_alloc_limpl(n);
                // let mut positions = Vec::<(usize,usize)>::new();


                // let mut init_vector = Vec::<usize>::new();
                let line_str = line.unwrap();
                eprintln!("Init vector (str): {}", line_str);
                let mut init_vector = line_str.split(",").map(|v| v.trim().parse().unwrap()).collect();
                eprintln!("Init vector (int): {:?}", init_vector);
    
                if b_need_transform {
                    // eprintln!("Positions_old: {positions_old:?}");
                    // eprintln!("Positions_new: {positions:?}");

                    let trf_init_vector = l_alglib::transform_init_vector(pord.len(), &positions_old, &positions, &init_vector);
                    eprintln!("Transformed init vector (int): {:?}", trf_init_vector);
                    init_vector = trf_init_vector;
                }

                l_alglib::l_alg_gen_from_ord_new(&pord, &init_vector, &mut lalgs, true, true, 10_000_000);
            }
            eprintln!("===\nTotal Computational Time: {:.4} s", ts.elapsed().as_secs_f32());
        }
    }
}
