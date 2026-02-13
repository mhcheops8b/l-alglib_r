use std::collections::{HashSet};

use itertools::{Itertools, Permutations};


fn l_alg_ax1(limpl: &Vec<Vec<usize>>, unit: usize) -> bool {
    let n = limpl.len();

    for x in 0..n {
        if limpl[x][x] != unit {
            return false;
        }
    }
    true
}

fn l_alg_ax2(limpl: &Vec<Vec<usize>>, unit: usize) -> bool {
    let n = limpl.len();

    for x in 0..n {
        if limpl[x][unit] != unit {
            return false;
        }
    }
    true
}

fn l_alg_ax3(limpl: &Vec<Vec<usize>>, unit: usize) -> bool {
    let n = limpl.len();

    for x in 0..n {
        if limpl[unit][x] != x {
            return false;
        }
    }
    true
}

fn l_alg_ax4(limpl: &Vec<Vec<usize>>, bprint: bool) -> bool {
    let n = limpl.len();

    for x in 0..n {
        for y in 0..n {
            for z in 0..n {        
                if limpl[limpl[x][y]][limpl[x][z]] != 
                   limpl[limpl[y][x]][limpl[y][z]] {
                    if bprint {
                        eprintln!("  Problem: x = {}, y = {}, z = {}", x, y, z);
                    }
                    return false;
                }
            }
        }
    }
    true
}

fn l_alg_ax5(limpl: &Vec<Vec<usize>>, unit: usize) -> bool {
    let n = limpl.len();

    for x in 0..n {
        for y in 0..n {
            if x != y {
                if limpl[x][y] == unit && limpl[y][x] == unit {
                    return false;
                }
            }
        }
    }
    true
}

fn l_alg_is_l_algebra(limpl: &Vec<Vec<usize>>, unit: usize, bprint: bool) -> bool {
    l_alg_ax1(limpl, unit) &&
    l_alg_ax2(limpl, unit) &&
    l_alg_ax3(limpl, unit) &&
    l_alg_ax4(limpl, bprint) &&
    l_alg_ax5(limpl, unit) 
}

fn l_alg_has_kl_property_old(limpl: &Vec<Vec<usize>>, unit: usize) -> bool {
    let n = limpl.len();

    for x in 0..n {
        for a in 0..n {
            if limpl[x][limpl[a][x]] != unit {
                eprintln!("Problem: x = {}, a = {}", x, a);
                return false;
            }
        }
    }
    true
}

fn l_alg_has_kl_property(limpl: &Vec<Vec<usize>>, unit: usize) -> Result<bool,String> {
    let n = limpl.len();

    for x in 0..n {
        for a in 0..n {
            if limpl[x][limpl[a][x]] != unit {
                let err = format!("KL - Problem: x = {}, a = {}", x, a);
                return Err(err);
            }
        }
    }
    Ok(true)
}


fn l_alg_is_commutative_l_algebra(limpl: &Vec<Vec<usize>>) -> Result<bool,String> {
    let n = limpl.len();

    for x in 0..n {
        for y in 0..n {
            if limpl[limpl[x][y]][y] != limpl[limpl[y][x]][x] {
                    let err = format!("Comm - Problem: x = {}, y = {}", x, y);
                    //eprintln!("Problem: x = {}, y = {}", x, y);
                return Err(err)
            }
        }
    }
    Ok(true)
}

fn l_alg_is_cl_algebra(limpl: &Vec<Vec<usize>>) -> Result<bool,String> {
    let n = limpl.len();

    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                if limpl[x][limpl[y][z]] != limpl[y][limpl[x][z]] {
                        let err = format!("CL - Problem: x = {}, y = {}, z = {}", x, y, z);
                        //eprintln!("Problem: x = {}, y = {}", x, y);
                    return Err(err)
                }
            }
        }
    }
    Ok(true)
}

fn l_alg_is_left_distributive(limpl: &Vec<Vec<usize>>) -> Result<bool,String> {
    let n = limpl.len();

    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                if limpl[x][limpl[y][z]] != limpl[limpl[x][y]][limpl[x][z]] {
                        let err = format!("Left Dist. - Problem: x = {}, y = {}, z = {}", x, y, z);
                        //eprintln!("Problem: x = {}, y = {}", x, y);
                    return Err(err)
                }
            }
        }
    }
    Ok(true)
}

fn l_alg_is_right_distributive(limpl: &Vec<Vec<usize>>) -> Result<bool,String> {
    let n = limpl.len();

    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                if limpl[limpl[x][y]][z] != limpl[limpl[x][z]][limpl[y][z]] {
                        let err = format!("Right Dist. - Problem: x = {}, y = {}, z = {}", x, y, z);
                        //eprintln!("Problem: x = {}, y = {}", x, y);
                    return Err(err)
                }
            }
        }
    }
    Ok(true)
}

fn l_alg_is_filter(limpl: &Vec<Vec<usize>>, unit: usize, subset: &HashSet<usize>, bprint: bool) -> bool {
    if !subset.contains(&unit) {
        return false;
    }

    let n = limpl.len();
    for x in subset {
        for y in 0..n {
            if subset.contains(&limpl[*x][y]) && !subset.contains(&y) {
                if bprint {
                    eprintln!("Problem: x = {}, y = {}", *x, y);
                }
                return false;
            }
        }
    }
    true
}

fn l_alg_get_all_filters(limpl: &Vec<Vec<usize>>, unit: usize) {
    let n = limpl.len();

    for i in 0usize..(1<<n) {
        if i & 1<<unit == 0 {
            continue;
        }
        let mut filt_cand = HashSet::<usize>::new();
        for k in 0..n {
            if i & (1<<k) != 0 {
                filt_cand.insert(k);
            }
        }

        if l_alg_is_filter(limpl, unit, &filt_cand, false) {
            println!(" {:?}", filt_cand);
        }
    }
}

fn gen_all_lalgs_rec(index:usize, positions:&Vec<(usize,usize)>, limpl: &mut Vec<Vec<usize>>, unit:usize, res:&mut HashSet<Vec<Vec<usize>>>) {
    let n = positions.len();
    // eprintln!("FHFH: {index} / {n}");

    if index >= n {
        if l_alg_is_l_algebra(limpl, unit, false) {
            res.insert(limpl.clone());
        }

    }
    else {
        let x = positions[index].0;
        let y = positions[index].1;
        let m = limpl.len();
        // let o_v = limpl[x][y];
        for e in 0.. m {
            limpl[x][y] = e;

            gen_all_lalgs_rec(index+1, positions, limpl, unit, res);
        }
    }
}

fn l_alg_isomorphic_image(limpl: &Vec<Vec<usize>>, unit: usize, perm:&Vec<usize>) -> (Vec<Vec<usize>>, usize) {
    let n = limpl.len();
    let mut res = Vec::<Vec<usize>>::new();
    
    for i in 0.. n {
        res.push(Vec::<usize>::new());
        for _ in 0..n {
            res[i].push(0);
        }
    }
    
    let res_unit = perm[unit];

    for i in 0..n {
        for j in 0..n {
            res[perm[i]][perm[j]] = perm[limpl[i][j]];
        }
    }
    // eprintln!("res: {res:?}");

    (res, res_unit)
}

fn main4() {
    let mut ex3_limpl = vec![vec![0,1,2,3],vec![0,0,3,2],vec![0,3,0,1],vec![0,2,1,0]];
    let ex3_unit = 0usize;
    let positions = Vec::from([(1usize,2usize),(1,3),(2,1),(2,3),(3,1),(3,2)]); 
    //let positions = Vec::from([(1usize,2usize),(1,3),(2,1),(2,3),(3,1),(3,2)]); 

    let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();

    
    
    gen_all_lalgs_rec(0, &positions, &mut ex3_limpl, ex3_unit, &mut lalgs);

    // println!("{lalgs:?}");
    // return;

    // eliminate isomorphic
    let mut lalgs_processed = HashSet::<Vec<Vec<usize>>>::new();
    let mut lalgs_unique = HashSet::<Vec<Vec<usize>>>::new();
    eprintln!(" lalgs size: {}", lalgs.len());
    for lalg in &lalgs {
        if !lalgs_processed.contains(lalg) {
            lalgs_unique.insert(lalg.clone());
            // println!("LL: {lalg:?}");
            for perm_ in (1..=3).permutations(3) {
                // eprintln!("{:?}", perm_);
                let iso_perm = vec![0,perm_[0],perm_[1],perm_[2]];
                // eprintln!("{:?}", iso_perm);

                lalgs_processed.insert(l_alg_isomorphic_image(lalg, ex3_unit, &iso_perm).0);
            }
            // eprintln!("{}:{lalgs_processed:?}", lalgs_processed.len());
            // eprintln!("--");
        }
    }


    eprintln!(" lalgs_unique size: {}", lalgs_unique.len());
    for lalg in &lalgs_unique {
        let mut already_processed = HashSet::<Vec<Vec<usize>>>::new();
        for perm in (1..=3).permutations(3) {
            let iso_perm = vec![0,perm[0],perm[1],perm[2]];

            let iso_image = l_alg_isomorphic_image(lalg, 0, &iso_perm).0;
            if !already_processed.contains(&iso_image) {
                println!("{iso_image:?}");
                already_processed.insert(iso_image);
            }
        }

        // if l_alg_is_right_distributive(lalg).is_ok() {
        //println!("{lalg:?}");
        println!(" Commutative: {}", l_alg_is_commutative_l_algebra(lalg).is_ok());
        println!("  CL-algebra: {}", l_alg_is_cl_algebra(lalg).is_ok());
        println!("  KL-algebra: {}", l_alg_has_kl_property(lalg, 0).is_ok());
        println!("  Left Dist.: {}", l_alg_is_left_distributive(lalg).is_ok());
        println!(" Filters:");
        //println!(" Right Dist.: {}", l_alg_is_right_distributive(lalg).is_ok());
        l_alg_get_all_filters(lalg, ex3_unit);
        // }
    }
}

#[allow(dead_code)]
fn main3() {
    let mut ex3_limpl = vec![vec![0,1,2,3],vec![0,0,3,2],vec![0,3,0,1],vec![0,2,1,0]];
    let ex3_unit = 0usize;
    let positions = Vec::from([(1usize,2usize),(1,3),(2,1),(2,3),(3,1),(3,2)]);
    let pos_len = positions.len();
    // // one change
    // for p1 in 0..pos_len {
    //     let x1 = positions[p1].0;
    //     let y1 = positions[p1].1;
    //     let v1 = ex3_limpl[x1][y1];
    //     for j in 0..4 {
    //         ex3_limpl[x1][y1] = j;
    //         if l_alg_is_l_algebra(&ex3_limpl, ex3_unit, false) {
    //             println!("{:?}", ex3_limpl);
    //         }
    //     }
    //     ex3_limpl[x1][y1] = v1;
    // }
    
    let mut algs = HashSet::<Vec<Vec<usize>>>::new();
    // two changes
    for p1 in 0..pos_len {
        let x1 = positions[p1].0;
        let y1 = positions[p1].1;
        let v1 = ex3_limpl[x1][y1];
        for e1 in 0..4 {
            ex3_limpl[x1][y1] = e1;
            for p2 in p1+1..pos_len {
                let x2 = positions[p2].0;
                let y2 = positions[p2].1;
                let v2 = ex3_limpl[x2][y2];
                for e2 in 0.. 4 {
                    ex3_limpl[x2][y2] = e2;
                    if l_alg_is_l_algebra(&ex3_limpl, ex3_unit, false) {
                        algs.insert(ex3_limpl.clone());
                        // println!("{:?}", ex3_limpl);
                        // l_alg_get_all_filters(&ex3_limpl, ex3_unit);
                    }
                }
                ex3_limpl[x2][y2] = v2;
            }
        }
        ex3_limpl[x1][y1] = v1;
    }
    

    // // three changes
    // for p1 in 0..pos_len {
    //     let x1 = positions[p1].0;
    //     let y1 = positions[p1].1;
    //     let v1 = ex3_limpl[x1][y1];
    //     for e1 in 0..4 {
    //         if true || e1 != v1 {
    //             ex3_limpl[x1][y1] = e1;
    //             for p2 in p1+1..pos_len {
    //                 let x2 = positions[p2].0;
    //                 let y2 = positions[p2].1;
    //                 let v2 = ex3_limpl[x2][y2];
    //                 for e2 in 0.. 4 {
    //                     if true || e2 != v2 {
    //                         ex3_limpl[x2][y2] = e2;
    //                         for p3 in p2+1..pos_len {
    //                             let x3 = positions[p3].0;
    //                             let y3 = positions[p3].1;
    //                             let v3 = ex3_limpl[x3][y3];
    //                             for e3 in 0.. 4 {
    //                                 if true || e3 != v3 {
    //                                     ex3_limpl[x2][y2] = e3;
    //                                     if l_alg_is_l_algebra(&ex3_limpl, ex3_unit, false) {
    //                                         algs.insert(ex3_limpl.clone());
    //                                     }
    //                                 }
    //                             }
    //                             ex3_limpl[x3][y3] = v3;
    //                         }
    //                     }
    //                 }
    //                 ex3_limpl[x2][y2] = v2;
    //             }
    //         }
    //     }
    //     ex3_limpl[x1][y1] = v1;
    // }

    for la in &algs {

        println!("{:?}", la);
        l_alg_get_all_filters(la, ex3_unit);

    }



    // for i in &positions {
    //     let v1 = ex3_limpl[i.0][i.1];
    //     for j in 0..4 {
    //         ex3_limpl[i.0][i.1] = j;
    //         for p2 in &positions {
    //             if p2 != i {
    //                 let v2 = ex3_limpl[p2.0][p2.1];
    //                 for e2 in 0..4 {
    //                     ex3_limpl[p2.0][p2.1] = e2;

                    

            
    //                     if l_alg_is_l_algebra(&ex3_limpl, ex3_unit, false) {
    //                         println!("{:?}", ex3_limpl);
    //                     }
    //                 }
    //                 ex3_limpl[p2.0][p2.1] = v2;    
    //             }
    //         }
    //     }
    //     ex3_limpl[i.0][i.1] = v1;
    // }

}

// #[allow(dead_code)]
fn main() {
    let ex1_limpl = vec![vec![0,1,2,3],vec![0,0,0,1],vec![0,1,0,3],vec![0,1,0,0]];
    let ex1_unit = 0;

    println!("Example 1");
    println!("  L-algebra: {}", l_alg_is_l_algebra(&ex1_limpl, ex1_unit, true));
    let ex1_subset1 = HashSet::from([0usize]);
    let ex1_subset2 = HashSet::from([0usize, 2]);
    let ex1_subset3 = HashSet::from([0usize, 2, 3]);

    println!(" {:?}: {}", ex1_subset1, l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset1, true));
    println!(" {:?}: {}", ex1_subset2, l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset2, true));
    println!(" {:?}: {}", ex1_subset3, l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset3, true));
    let ex1_subset4 = HashSet::from([0usize, 1]);
    println!(" {:?}: {}", ex1_subset4, l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset4, true));
    let ex1_subset4 = HashSet::from([0usize, 1,2]);
    println!(" {:?}: {}", ex1_subset4, l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset4, true));
    let ex1_subset4 = HashSet::from([0usize, 1,3]);
    println!(" {:?}: {}", ex1_subset4, l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset4, true));
    let ex1_subset4 = HashSet::from([0usize, 3]);
    println!(" {:?}: {}", ex1_subset4, l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset4, true));
    let ex1_subset4 = HashSet::from([0usize, 1,2,3]);
    println!(" {:?}: {}", ex1_subset4, l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset4, true));

    let ex3_limpl = vec![vec![0,1,2,3],vec![0,0,3,2],vec![0,3,0,1],vec![0,2,1,0]];
    let ex3_unit = 0;
    println!("Example 3");
    println!("  L-algebra: {}", l_alg_is_l_algebra(&ex3_limpl, ex3_unit, true));
    let ex3_subset1 = HashSet::from([0usize, 1]);
    println!(" {:?}: {}", ex3_subset1, l_alg_is_filter(&ex3_limpl, ex3_unit, &ex3_subset1, true));

    let ex4_limpl = vec![vec![4,4,4,4,4],vec![0,4,4,4,4],vec![0,3,4,3,4],vec![0,2,2,4,4],vec![0,1,2,3,4]];
    let ex4_unit = 4usize;
    println!("Example 4");
    println!("  L-algebra: {}", l_alg_is_l_algebra(&ex4_limpl, ex4_unit, true));

    let ex5_limpl = vec![vec![3,3,3,3],vec![2,3,3,3],vec![1,1,3,3],vec![0,1,2,3]];
    let ex5_unit = 3;
    println!("Example 5");
    println!("  L-algebra: {}", l_alg_is_l_algebra(&ex5_limpl, ex5_unit, true));

    let ex5p_limpl = vec![vec![3,3,3,3],vec![2,3,2,3],vec![1,1,3,3],vec![0,1,2,3]];
    let ex5p_unit = 3;
    println!("Example 5'");
    println!("  L-algebra: {}", l_alg_is_l_algebra(&ex5p_limpl, ex5p_unit, true));
    println!(" KL-algebra: {}", l_alg_has_kl_property(&ex5p_limpl, ex5p_unit).is_ok());

    let ex5p2_limpl = vec![vec![3,3,3,3],vec![2,3,3,3],vec![1,2,3,3],vec![0,1,2,3]];
    let ex5p2_unit = 3;
    println!("Example 5''");
    println!("  L-algebra: {}", l_alg_is_l_algebra(&ex5p2_limpl, ex5p2_unit, true));
    println!(" KL-algebra: {}", l_alg_has_kl_property(&ex5p2_limpl, ex5p2_unit).is_ok());
    let ex5p2_subset1 = HashSet::from([1usize, 2, 3]);
    println!(" {:?}: {}", ex5p2_subset1, l_alg_is_filter(&ex5p2_limpl, ex5p2_unit, &ex5p2_subset1, true));

    let ex7_limpl = vec![vec![0,1,2,3],vec![0,0,3,2],vec![0,3,0,1],vec![0,2,1,0]];
    let ex7_unit = 0;
    println!("Example 7");
    println!("  L-algebra: {}", l_alg_is_l_algebra(&ex7_limpl, ex7_unit, true));

    let ex7p_limpl = vec![vec![0,1,2,3],vec![0,0,3,2],vec![0,3,0,1],vec![0,2,1,0]];
    let ex7p_unit = 0;
    println!("Example 7''");
    println!("  L-algebra: {}", l_alg_is_l_algebra(&ex7p_limpl, ex7p_unit, true));

    let ex3_limpl = vec![vec![0,1,2,3],vec![0,0,3,2],vec![0,3,0,1],vec![0,2,1,0]];
    let ex3_unit = 0;
    println!("Example 3");
    println!("  L-algebra: {}", l_alg_is_l_algebra(&ex3_limpl, ex3_unit, true));
    let ex3_subset1 = HashSet::from([0usize, 1]);
    println!(" {:?}: {}", ex3_subset1, l_alg_is_filter(&ex3_limpl, ex3_unit, &ex3_subset1, true));

    

}
