fn main() {
   main7_66_1();
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