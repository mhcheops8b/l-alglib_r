use std::{collections::HashSet, io::BufRead};
use std::fs::File;
use std::io::{BufReader};

fn main() {

    let args_len = std::env::args().len();

    if args_len < 5 || args_len == 6 {
        println!("Usage: {} <iterations limit> <print limit> <pord_filename> <pord_num> [<need transform> <init_vec_string>]", std::env::args().next().unwrap());
        return;
    }

    let iter_limit = match std::env::args().nth(1).unwrap().trim().parse::<usize>() {
        Ok(val) => val,
        Err(_e) => {println!("First argument must be a number (iter limit)."); return;}
    };

    let print_limit = match std::env::args().nth(2).unwrap().trim().parse::<usize>() {
        Ok(val) => val,
        Err(_e) => {println!("Second argument must be a number (print limit)."); return;}
    };   
    
    let pord_filename = std::env::args().nth(3).unwrap();
    if std::fs::exists(&pord_filename).is_err() {
        eprintln!("File with partial orders \"{pord_filename}\" does not exist.");
        return;
    }
    let file = BufReader::new(File::open(pord_filename).expect("Cannot open file"));

    let pord_num = match std::env::args().nth(4).unwrap().parse::<usize>() {
        Ok(val) => val,
        Err(_e) => {eprintln!("Third argument must be a number (idx of pord)."); return;}
    };

    let mut init_vector = Vec::<usize>::new();
    let mut b_need_transform = false;
    if args_len >= 7 {
        b_need_transform = match std::env::args().nth(5).unwrap().parse::<usize>() {
            Ok(val) => {match val {
                                0=> false, 
                                1=>true, 
                                _ => {eprintln!("Argument must be 0 or 1."); return;}
                              }
                            },
            Err(_e) => {eprintln!("First argument must be a number (iter limit)."); return;}
                        };

        let init_vector_str = std::env::args().nth(6).unwrap();
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
            let mut positions = Vec::<(usize,usize)>::new();
            let mut positions_old = Vec::<(usize,usize)>::new();
                
            l_alglib::l_alg_init_get_positions_old(&pord, &mut positions_old); 
            l_alglib::l_alg_init_get_positions_new(&pord, &mut positions);
 
            if b_need_transform {
                // eprintln!("Positions_old: {positions_old:?}");
                // eprintln!("Positions_new: {positions:?}");

                let trf_init_vector = l_alglib::transform_init_vector(pord.len(), &positions_old, &positions, &init_vector);
                eprintln!("Transformed init vector (int): {:?}", trf_init_vector);
                init_vector = trf_init_vector;
            }

            l_alglib::l_alg_gen_from_ord_short_iter_limit_new(iter_limit, print_limit, &pord, &init_vector, &mut lalgs, true, true);           
        }
    }    
}