use std::{collections::HashSet};
use std::time::Instant;

fn main() {
   
    let args_len = std::env::args().len();

    if args_len < 2 || args_len == 3 {
        println!("Usage: {} <pord_init_string> [<need transform> <starting_init_vec_string>]", std::env::args().next().unwrap());
        return;
    }

    let pord_init_string = std::env::args().nth(1).unwrap();
    
    eprintln!("Input Order (str): {}", pord_init_string);

    let pord_res = serde_json::from_str::<Vec<Vec<usize>>>(&pord_init_string);
    
    let pord = match pord_res {
        Ok(p) => p,
        Err(err) => {eprintln!("Error parsing json (pord): {err:?}"); return;}
    };
    
    let n = pord.len();

    let mut init_vector = Vec::<usize>::new();
    let mut b_need_transform =false;
    
    if args_len >= 4 {
        b_need_transform = match std::env::args().nth(2).unwrap().parse::<usize>() {
            Ok(val) => {match val {
                                0=> false, 
                                1=>true, 
                                _ => {eprintln!("Argument must be 0 or 1."); return;}
                              }
                            },
            Err(_e) => {eprintln!("First argument must be a number (iter limit)."); return;}
                        };
        
    
        
        let init_vector_str = std::env::args().nth(3).unwrap();
        eprintln!("Init vector (str): {}", init_vector_str);
        init_vector = init_vector_str.split(",").map(|v| v.trim().parse().unwrap()).collect();
        eprintln!("Init vector (int): {:?}", init_vector);
    }
    


    let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
    
    eprintln!("Order: {pord:?}");

    let mut lalg_limpl = l_alglib::l_alg_alloc_limpl(n);
    let mut positions = Vec::<(usize,usize)>::new();
    let mut positions_old = Vec::<(usize,usize)>::new();
    
    l_alglib::l_alg_init_from_ord(&mut lalg_limpl, &pord, n-1);
    l_alglib::l_alg_init_get_positions_new(&pord, &mut positions); 

    if b_need_transform {
    
        l_alglib::l_alg_init_get_positions_old(&pord, &mut positions_old); 
        
        eprintln!("Positions_old: {positions_old:?}");
        eprintln!("Positions_new: {positions:?}");
        
        let trf_init_vector = l_alglib::transform_init_vector(n, &positions_old, &positions, &init_vector);
        eprintln!("Transformed init vector (int): {:?}", trf_init_vector);
        init_vector = trf_init_vector;
    }
    //
    
    // let init_vector = trf_init_vector;
    // apply init_vector
    for i in 0usize..std::cmp::min(positions.len(), init_vector.len()) {
        let x = positions[i].0;
        let y = positions[i].1;
        let e = init_vector[i];
        eprintln!("{},{},{}", x, y, e);

        if e == n+1 {
            eprintln!("Skipping initialization of ({}, {})", x, y);
            continue;
        }
        if e == n-1 {
            eprintln!("Element at ({}, {}) cannot be equal to unit ({}).",x,y,n-1);
            return;
        }
        
        if lalg_limpl[y][x] == n-1 && lalg_limpl[y][e] != n-1 {
            eprintln!("Element at ({}, {}) needs to be greater than {} since {} <= {}.",x,y,y,y,x);
            return;
        }

        for t in 0..y {
            if lalg_limpl[t][y] == n-1 && lalg_limpl[x][t] != n+1 && lalg_limpl[lalg_limpl[x][t]][e] != n-1 {
                eprintln!("Element e={} at (x={}, y={}) needs to larger than {} since t={} <= y => x->t <= x->y.", e, x, y, lalg_limpl[x][t], t);
                return;
            }
        }

        lalg_limpl[x][y] = e;
        if !l_alglib::l_alg_test_ax4_partial(&lalg_limpl, true) {
            //eprintln!("Partial ax4 is not satisfied");
            return;
        }
        // perform tests TODO
    }
    
    for i in (0usize..std::cmp::min(positions.len(), init_vector.len())).rev() {
        if init_vector[i] != n+1 {
            positions.remove(i); 
        }
    }

    eprintln!("Positions: {positions:?}");
    eprintln!("Init limpl: {lalg_limpl:?}");
    let time_start = Instant::now();
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    l_alglib::gen_all_lalgs_rec(0, &positions, &mut lalg_limpl, n-1, &mut lalgs, &mut num_tested, &mut num_models);

    eprintln!("Computation time: {:.2} s", time_start.elapsed().as_secs_f32());
    eprintln!("Number recursive calls: {}", num_tested);
    eprintln!("Number of all models: {}", num_models);
    eprintln!("Number of representative models {}", lalgs.len());
}
