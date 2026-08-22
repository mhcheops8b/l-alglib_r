fn main() {
    // main7_1_1();
    // main7_1_2();
    // main_7_1_1_new2();
    // main7_2_1();
    // main_7_2_1_new2();
    // main7_3_1();
    // main7_4_1();
    // main7_5_1();
    // main7_66_1();
    main_7_66_1_new2();
    // main_7_67_1_new2();
    // main7_75_1();
    // main7_256_1();
    // main_7_256_1_new2();
    // main7_257_1();
    // main7_272_1();
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

#[allow(dead_code)]
fn main_7_1_1_new2() {    
    // rel_get_cover_rel(&vec![vec![1, 0, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]]);
    // return;
    l_alglib::gen_plans_main_new2(
        // 
        &vec![vec![1, 0, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]],
        //
        1,
        // 
        &vec![(0,1), (0,2), (0,3), (0,4), (0,5), (1,0), (1,2)] //
    )
}

#[allow(dead_code)]
fn main7_2_1() {
    // 2
    let num_pord = 2;
    // 
    let pord = vec![vec![1, 0, 0, 0, 0, 1, 1], vec![0, 1, 0, 0, 0, 1, 1], vec![0, 0, 1, 0, 0, 1, 1], vec![0, 0, 0, 1, 0, 1, 1], vec![0, 0, 0, 0, 1, 1, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(0,1), (0,2), (0,3), (0,4)];
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
fn main_7_2_1_new2() {    
    // rel_get_cover_rel(&vec![vec![1, 0, 0, 0, 0, 1, 1], vec![0, 1, 0, 0, 0, 1, 1], vec![0, 0, 1, 0, 0, 1, 1], vec![0, 0, 0, 1, 0, 1, 1], vec![0, 0, 0, 0, 1, 1, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]]);
    // return;
    l_alglib::gen_plans_main_new2(
        // 
        &vec![vec![1, 0, 0, 0, 0, 1, 1], vec![0, 1, 0, 0, 0, 1, 1], vec![0, 0, 1, 0, 0, 1, 1], vec![0, 0, 0, 1, 0, 1, 1], vec![0, 0, 0, 0, 1, 1, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]],
        //
        1,
        // 
        &vec![(0,1), (0,2), (0,3), (0,4), (1,0), (1,2)] //
    )
}

#[allow(dead_code)]
fn main7_3_1() {
    // 3
    let num_pord = 3;
    // 
    let pord = vec![vec![1, 0, 0, 0, 1, 0, 1], vec![0, 1, 0, 0, 1, 0, 1], vec![0, 0, 1, 0, 1, 0, 1], vec![0, 0, 0, 1, 1, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(0,1), (0,2), (0,3), (0,5), (4,5), (5,4)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        pe[0] == 0 && pe[4] == 4
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
fn main7_4_1() {
    // 4
    let num_pord = 4;
    // 
    let pord = vec![vec![1, 0, 0, 0, 1, 1, 1], vec![0, 1, 0, 0, 1, 1, 1], vec![0, 0, 1, 0, 1, 1, 1], vec![0, 0, 0, 1, 1, 1, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(0,1), (0,2), (0,3), (4,0), (4,5), (5,0), (5,4)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        pe[0] == 0 
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

    l_alglib::gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}

#[allow(dead_code)]
fn main7_5_1() {
    // 5
    let num_pord = 5;
    // 
    let pord = vec![vec![1, 0, 0, 0, 1, 1, 1], vec![0, 1, 0, 0, 1, 1, 1], vec![0, 0, 1, 0, 1, 1, 1], vec![0, 0, 0, 1, 1, 1, 1], vec![0, 0, 0, 0, 1, 1, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(0,1), (0,2), (0,3), (4,0), (5,0)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        pe[0] == 0 
        // pe[0] == 0 && std::cmp::min(pe[4],pe[5]) == 4 && std::cmp::max(pe[4],pe[5]) == 5
    }

    let mut from_vec = Vec::<usize>::new();
    if std::env::args().len() == 2 {
        from_vec = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse::<usize>().unwrap()).collect();
    }

    l_alglib::gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}

#[allow(dead_code)]
fn main7_66_1() {
    // 66
    let num_pord = 66;
    // 
    let pord = vec![vec![1, 1, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(0,2), (0,3), (0,4), (0,5), (1,0)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        
        pe[1] == 1
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

    l_alglib::gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}

#[allow(dead_code)]
fn main_7_66_1_new2() {    
    // rel_get_cover_rel(&vec![vec![1, 1, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]]);
    // return;
    l_alglib::gen_plans_main_new2(
        // 
        &vec![vec![1, 1, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]],
        //
        1,
        // 
        // &vec![(0,2), (0,3), (0,4), (0,5), (1,0), (1,2)] //
        &vec![(0,2), (0,3), (0,4), (0,5), (1,0), (1,2), (1,3), (1,4), (1,5),
                         (2,0), (2,1), (2,3), (2,4), (2,5), (3,0), (3,1), (3,2), (3,4),
                         (3,5), (4,0), (4,1), (4,2), (4,3), (4,5), (5,0), (5,1), (5,2), (5,3), (5,4)] //b
    )
}

#[allow(dead_code)]
fn main_7_67_1_new2() {    
    // l_alglib::rel_get_cover_rel(&vec![vec![1, 1, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 1, 1], vec![0, 0, 0, 1, 0, 1, 1], vec![0, 0, 0, 0, 1, 1, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]]);
    // return;
    l_alglib::gen_plans_main_new2(
        // 
        &vec![vec![1, 1, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 1, 1], vec![0, 0, 0, 1, 0, 1, 1], vec![0, 0, 0, 0, 1, 1, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]],
        //
        1,
        // 
        // &vec![(0,2), (0,3), (0,4), (0,5), (1,0), (1,2)] //
        &vec![(0,2), (0,3), (0,4), (0,5), (1,0), (1,2), (1,3), (1,4), (1,5),
                         (2,0), (2,1), (2,3), (2,4), (3,0), (3,1), (3,2), (3,4), (4,0),
                         (4,1), (4,2), (4,3), (5,0), (5,1), (5,2), (5,3), (5,4)] //b 180.4444s
    )
}

#[allow(dead_code)]
fn main7_75_1() {
    // 75
    let num_pord = 75;
    // 
    let pord = vec![vec![1, 1, 0, 0, 0, 1, 1], vec![0, 1, 0, 0, 0, 1, 1], vec![0, 0, 1, 0, 0, 1, 1], vec![0, 0, 0, 1, 0, 1, 1], vec![0, 0, 0, 0, 1, 1, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(0,2), (0,3), (0,4), (1,0), (1,2), (1,3), (1,4)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        
        pe[1] == 1
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

    l_alglib::gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
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

#[allow(dead_code)]
fn main_7_256_1_new2() {    
    // rel_get_cover_rel(&vec![vec![1, 1, 1, 1, 1, 1, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]]);
    // return;
    l_alglib::gen_plans_main_new2(
        // 
        &vec![vec![1, 1, 1, 1, 1, 1, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]],
        //
        1,
        // 
        &vec![(1,0), (1,2), (1,3), (1,4), (1,5), (2,0)]
    )
}

#[allow(dead_code)]
fn main7_257_1() {
    // 
    let num_pord = 257;
    // 
    let pord = vec![vec![1, 1, 1, 1, 1, 1, 1], vec![0, 1, 0, 0, 0, 1, 1], vec![0, 0, 1, 0, 0, 1, 1], vec![0, 0, 0, 1, 0, 1, 1], vec![0, 0, 0, 0, 1, 1, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(1,2), (1,3), (1,4), (5,1)];
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

#[allow(dead_code)]
fn main7_272_1() {
    // 
    let num_pord = 272;
    // 
    let pord = vec![vec![1, 1, 1, 1, 1, 1, 1], vec![0, 1, 1, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    //    
    let fixed_vec: Vec<(usize, usize)> = vec![(1,0), (1,3), (1,4), (1,5), (2,0), (2,1)];
    // 
    fn fix_pred(pe: &[usize]) -> bool {
        
        pe[2] == 2
        // pe[0] == 0 && std::cmp::min(pe[4],pe[5]) == 4 && std::cmp::max(pe[4],pe[5]) == 5
    }
    
    let mut from_vec = Vec::<usize>::new();
    if std::env::args().len() == 2 {
        from_vec = std::env::args().nth(1).unwrap().split(",").map(|v| v.trim().parse::<usize>().unwrap()).collect();
    }

    l_alglib::gen_plans(&pord, num_pord, &fixed_vec, fix_pred, &from_vec);
}