fn main() {
    // main7_1_1();
    main7_1_2();
}

#[allow(dead_code)]
fn main7_1_1() {
    // 1
    let num_pord = 1;
    // 
    let pord = vec![vec![1, 0, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(0,1), (0,2), (0,3), (0,4), (0,5)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        pe[0] == 0
    }
    
    // if std::env::args().len() < 2 {
    //     println!("Usage: {} <init_vector>", std::env::args().next().unwrap());
    //     return;

    // }
    let mut from_vec = Vec::<usize>::new();
    if std::env::args().len() == 2 {
        from_vec = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse::<usize>().unwrap()).collect();
    }

    l_alglib::gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}

#[allow(dead_code)]
fn main7_1_2() {
    // 1
    let num_pord = 1;
    // 
    let pord = vec![vec![1, 0, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(0,1), (0,2), (0,3), (0,4), (0,5), (1,0), (1,2), (1,3), (1,4), (1,5)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        pe[0] == 0
    }
    
    // if std::env::args().len() < 2 {
    //     println!("Usage: {} <init_vector>", std::env::args().next().unwrap());
    //     return;

    // }
    let mut from_vec = Vec::<usize>::new();
    if std::env::args().len() == 2 {
        from_vec = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse::<usize>().unwrap()).collect();
    }

    l_alglib::gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}