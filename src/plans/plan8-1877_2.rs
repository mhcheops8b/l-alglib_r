fn main() {
   main_1877_1();
    // main_1877_2();
}

#[allow(dead_code)]
fn main_1877_1() {
    // 
    let num_pord = 1877;
    // 
    let pord = vec![vec![1, 1, 1, 1, 1, 1, 1, 1], vec![0, 1, 1, 1, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(1,0), (2,1), (3,1), (4,1), (5,1), (6,1)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        
        pe[1]==1 && std::cmp::min(pe[2],pe[3]) == 2 && std::cmp::max(pe[2],pe[3]) == 3 && std::cmp::min(std::cmp::min(pe[4], pe[5]), pe[6])==4 && std::cmp::max(std::cmp::max(pe[4], pe[5]), pe[6])==6
        // pe[0] == 0 && std::cmp::min(pe[4],pe[5]) == 4 && std::cmp::max(pe[4],pe[5]) == 5
    }
    
    let mut from_vec = Vec::<usize>::new();
    if std::env::args().len() == 2 {
        from_vec = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse::<usize>().unwrap()).collect();
    }

    l_alglib::gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}

#[allow(dead_code)]
fn main_1877_2() {
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
    
    let mut from_vec = Vec::<usize>::new();
    if std::env::args().len() == 2 {
        from_vec = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse::<usize>().unwrap()).collect();
    }

    l_alglib::gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}
