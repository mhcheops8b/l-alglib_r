fn main() {
   main_1_3();
}

fn main_1_3() {
    // 1
    let num_pord = 1;
    let pord = vec![vec![1, 0, 0, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
        
    let fixed_vec: Vec<(usize, usize)> = vec![(0,1), (0,2), (0,3), (0,4), (0,5), (0,6), 
                                              (1,0), (1,2), (1,3), (1,4), (1,5), (1,6), 
                                              (2,0), (2,1), (2,3), (2,4), (2,5), (2,6)];
    let mut lalg_limpl = l_alglib::l_alg_alloc_limpl(pord.len());
    let mut positions = Vec::<(usize,usize)>::new();
    fn ff(pe: &[usize]) -> bool {
        std::cmp::min(std::cmp::min(pe[0], pe[1]), pe[2]) == 0 && 
        std::cmp::max(std::cmp::max(pe[0], pe[1]), pe[2]) == 2
    }
    // let ff = (|pe:Vec<usize>| (pe[0]==0 && pe[1]==1 || pe[0]==1 && pe[1]==0) && pe[5]==5);
    l_alglib::l_alg_init_from_ord(&mut lalg_limpl, &pord, pord.len()-1, &mut positions);

    if std::env::args().len() < 2 {
        println!("Usage: {} <init_vector>", std::env::args().next().unwrap());
        return;

    }
    let from_vec:Vec<_> = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse().unwrap()).collect();
    let mut iter_cnt =0usize;

    for i in 0..from_vec.len() {
        if l_alglib::l_alg_test_init_value(fixed_vec[i].0, fixed_vec[i].1, from_vec[i], &lalg_limpl) {
            lalg_limpl[fixed_vec[i].0][fixed_vec[i].1] = from_vec[i];
        }
        else {
            return;
        }
    }
    l_alglib::get_plan_fixed_rec(from_vec.len(), &mut iter_cnt, pord.len(), &pord, num_pord, &fixed_vec,&positions, ff, &mut lalg_limpl, &l_alglib::OutputType::List);
    eprintln!("Finished.");
    // let mut ts =Instant::now();
    // l_alglib::get_plan_continue_rec(&mut from_vec, &mut iter_cnt, &mut ts, 0, pord.len(), &pord, num_pord, &fixed_vec, &positions, ff, &mut lalg_limpl, &l_alglib::OutputType::Script);
}