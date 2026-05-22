use std::time::{Instant};
use itertools::{Itertools};

fn main() {
   main_1_1();
    // main_1_2();
}

fn gen_plans(pord: &Vec<Vec<usize>>, num_pord: usize, fixed_vec: &Vec<(usize,usize)>, fixed_predicate: fn(&[usize])->bool, init_vector: &Vec<usize>) {
    let mut lalg_limpl = l_alglib::l_alg_alloc_limpl(pord.len());
    let mut positions = Vec::<(usize,usize)>::new();

    l_alglib::l_alg_init_from_ord(&mut lalg_limpl, &pord, pord.len()-1);
    l_alglib::l_alg_init_get_positions_old(&pord, &mut positions);
    
    for i in 0..init_vector.len() {
        if l_alglib::l_alg_test_init_value(fixed_vec[i].0, fixed_vec[i].1, init_vector[i], &lalg_limpl) {
            lalg_limpl[fixed_vec[i].0][fixed_vec[i].1] = init_vector[i];
        }
        else {
            return;
        }
    }
    
    let mut num_iter =0usize;
    l_alglib::get_plan_fixed_rec(init_vector.len(), &mut num_iter, pord.len(), &pord, num_pord, fixed_vec,&positions, fixed_predicate, &mut lalg_limpl, &l_alglib::OutputType::List);
    // print_vec(&mut std::io::stderr(), &get_iter(fixed_vec.len(), &fixed_vec, &lalg_limpl));
    eprintln!("Finished.");
}


fn main_1_1() {
    // 
    let num_pord = 1762;
    // 
    let pord = vec![vec![1, 1, 1, 1, 1, 1, 1, 1], vec![0, 1, 0, 1, 1, 0, 0, 1], vec![0, 0, 1, 1, 1, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
                                        
    // let pp = (0usize..pord.len()).collect::<Vec<_>>();
    // let jj:Vec<_> = pp.into_iter().permutations(pord.len())
    //         .filter(|pe| l_alglib::pord_perm_preserve_ord(&pord, &pe))
    //         .filter(|pe| pe[1]==1 && pe[2]==2 && pe[3]==3 && pe[4]==4)
    //         .collect();

    // eprintln!("{jj:?}");
    // return;
    
    
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(1,0), (1,2), (1,5), (1,6), (2,0), (2,1), (2,5), (2,6)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        
        std::cmp::min(pe[1],pe[2]) == 1 && std::cmp::max(pe[1],pe[2]) == 2 &&
        std::cmp::min(pe[3],pe[4]) == 3 && std::cmp::max(pe[3],pe[4]) == 4 &&
        std::cmp::min(pe[5],pe[6]) == 5 && std::cmp::max(pe[5],pe[6]) == 6 
        // pe[0] == 0 && std::cmp::min(pe[4],pe[5]) == 4 && std::cmp::max(pe[4],pe[5]) == 5
    }
    
    // if std::env::args().len() < 2 {
    //     println!("Usage: {} <init_vector>", std::env::args().next().unwrap());
    //     return;

    // }
    let mut from_vec = Vec::<usize>::new();
    if std::env::args().len() == 2 {
        from_vec = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse::<usize>().unwrap()).collect();
    }

    gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}

fn main_1_2() {
    // 1
    let num_pord = 1;
    // 
    let pord = vec![vec![1, 0, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(0,1), (0,2), (0,3), (0,4), (0,5), (1,0), (1,2), (1,3), (1,4), (1,5)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        std::cmp::min(pe[0], pe[1]) == 0 && std::cmp::max(pe[0], pe[1]) == 1
    }
    
    // if std::env::args().len() < 2 {
    //     println!("Usage: {} <init_vector>", std::env::args().next().unwrap());
    //     return;

    // }
    let mut from_vec = Vec::<usize>::new();
    if std::env::args().len() == 2 {
        from_vec = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse::<usize>().unwrap()).collect();
    }

    gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}

// fn main_1_2() {
//     // 1
//     let num_pord = 1;
//     // 
//     let pord = vec![vec![1, 0, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
        
//     let fixed_vec: Vec<(usize, usize)> = vec![(0,1), (0,2), (0,3), (0,4), (0,5)];
//         let mut lalg_limpl = l_alglib::l_alg_alloc_limpl(pord.len());
//     let mut positions = Vec::<(usize,usize)>::new();
//     // fn ff(pe: &Vec<usize>) -> bool {
//     fn ff(pe: &[usize]) -> bool {
//     // fn ff(pe: &[usize]) -> bool {
//         std::cmp::min(pe[0], pe[1])== 0 && std::cmp::max(pe[0], pe[1]) == 1
//     }
//     // let ff = (|pe:Vec<usize>| (pe[0]==0 && pe[1]==1 || pe[0]==1 && pe[1]==0) && pe[5]==5);
//     l_alglib::l_alg_init_from_ord(&mut lalg_limpl, &pord, pord.len()-1);
//     l_alglib::l_alg_init_get_positions_old(&pord, &mut positions);
    
//     if std::env::args().len() < 2 {
//         println!("Usage: {} <init_vector>", std::env::args().next().unwrap());
//         return;

//     }
//     let mut from_vec: Vec<_> = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse().unwrap()).collect();
//     if true {

//         for i in 0..from_vec.len() {
//             if l_alglib::l_alg_test_init_value(fixed_vec[i].0, fixed_vec[i].1, from_vec[i], &lalg_limpl) {
//                 lalg_limpl[fixed_vec[i].0][fixed_vec[i].1] = from_vec[i];
//             }
//             else {
//                 return;
//             }
//         }
    
//         let mut num_iter =0usize;
//         l_alglib::get_plan_fixed_rec(from_vec.len(), &mut num_iter, pord.len(), &pord, num_pord, &fixed_vec,&positions, ff, &mut lalg_limpl, &l_alglib::OutputType::List);
//         // print_vec(&mut std::io::stderr(), &get_iter(fixed_vec.len(), &fixed_vec, &lalg_limpl));
//         eprintln!("Finished.");
//     }
//     else {
//         let mut num_iter =0usize;
//         let mut ts =Instant::now();
//         l_alglib::get_plan_continue_rec(&mut from_vec, &mut num_iter, &mut ts, 0, pord.len(), &pord, num_pord, &fixed_vec,&positions, ff, &mut lalg_limpl, &l_alglib::OutputType::List);
//     }
// }

