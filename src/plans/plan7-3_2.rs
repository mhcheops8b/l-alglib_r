fn main() {
   main7_3_1();
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