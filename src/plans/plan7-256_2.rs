fn main() {
   main7_256_1();
}

#[allow(dead_code)]
fn main7_256_1() {
    // 
    let num_pord = 256;
    // 
    let pord = vec![vec![1, 1, 1, 1, 1, 1, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(1,0), (1,2), (1,3), (1,4), (1,5)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        
        pe[1] == 1
        // pe[0] == 0 && std::cmp::min(pe[4],pe[5]) == 4 && std::cmp::max(pe[4],pe[5]) == 5
    }
    
    let mut from_vec = Vec::<usize>::new();
    if std::env::args().len() == 2 {
        from_vec = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse::<usize>().unwrap()).collect();
    }

    l_alglib::gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}