fn main() {
   main_1762_1();
    // main_1_2();
}

#[allow(dead_code)]
fn main_1762_1() {
    // 
    let num_pord = 1762;
    // 
    let pord = vec![vec![1, 1, 1, 1, 1, 1, 1, 1], vec![0, 1, 0, 1, 1, 0, 0, 1], vec![0, 0, 1, 1, 1, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(1,0), (1,2), (1,5), (1,6), (2,0), (2,1), (2,5), (2,6)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        
        std::cmp::min(pe[1],pe[2]) == 1 && std::cmp::max(pe[1],pe[2]) == 2 &&
        std::cmp::min(pe[3],pe[4]) == 3 && std::cmp::max(pe[3],pe[4]) == 4 &&
        std::cmp::min(pe[5],pe[6]) == 5 && std::cmp::max(pe[5],pe[6]) == 6 
        // pe[0] == 0 && std::cmp::min(pe[4],pe[5]) == 4 && std::cmp::max(pe[4],pe[5]) == 5
    }
    
    let mut from_vec = Vec::<usize>::new();
    if std::env::args().len() == 2 {
        from_vec = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse::<usize>().unwrap()).collect();
    }

    l_alglib::gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}