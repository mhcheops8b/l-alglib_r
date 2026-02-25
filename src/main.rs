// use core::num;
use std::{collections::HashSet, io::BufRead};
use itertools::{Itertools};
// use l_alglib::l_alg_init_limpl;
use std::fs::File;
use bzip2::read::{BzDecoder};
use std::io::{BufReader};

#[allow(dead_code)]
fn parse_vector(line: &String) -> Vec<Vec<usize>> {
        let mut parsed_vector = Vec::<Vec<usize>>::new();
        for t in line.split("[") {
            if t=="" {
                continue;
            }
            let mut tt = String::from(t);
            
            let gg = tt.find("], ");
            if gg.is_some() {
                let pos = gg.unwrap();
                tt.remove(pos);
                tt.remove(pos);
                tt.remove(pos);
            }
            let gg = tt.find("]]");
            if gg.is_some() {
                let pos = gg.unwrap();
                tt.remove(pos);
                tt.remove(pos);
            }
            let vv = tt.split(", ").map(|v| {//println!("v = {v:?}"); 
            v.trim().parse::<usize>().unwrap()}).collect::<Vec<_>>();
            // println!("{vv:?}");
            parsed_vector.push(vv);

            //tt.remove_matches("]]");
            //println!("{tt:?}");
        }
        // println!("{parsed_vector:?}");

        parsed_vector

}

//#![feature(string_remove_matches)]
fn main() {
    // let file = BufReader::new(File::open("./hh6_1.txt").expect("Cannot open file"));

    // for line in file.lines() {
    //     let yy = parse_vector(&line.unwrap());
    //     //println!("{yy:?}");
    //     println!("{:?}", l_alglib::l_alg_get_repr(&yy, false));
    //     //println!("  -> {:?}", l_alglib::l_alg_get_repr(&yy, false));
    //     // let mut parsed_vector = Vec::<Vec<usize>>::new();
    //     // for t in line.unwrap().split("[") {
    //     //     if t=="" {
    //     //         continue;
    //     //     }
    //     //     let mut tt = String::from(t);
            
    //     //     let gg = tt.find("], ");
    //     //     if gg.is_some() {
    //     //         let pos = gg.unwrap();
    //     //         tt.remove(pos);
    //     //         tt.remove(pos);
    //     //         tt.remove(pos);
    //     //     }
    //     //     let gg = tt.find("]]");
    //     //     if gg.is_some() {
    //     //         let pos = gg.unwrap();
    //     //         tt.remove(pos);
    //     //         tt.remove(pos);
    //     //     }
    //     //     let vv = tt.split(", ").map(|v| {//println!("v = {v:?}"); 
    //     //     v.trim().parse::<usize>().unwrap()}).collect::<Vec<_>>();
    //     //     // println!("{vv:?}");
    //     //     parsed_vector.push(vv);

    //     //     //tt.remove_matches("]]");
    //     //     //println!("{tt:?}");
    //     // }
    //     // println!("{parsed_vector:?}");
    //     // break;
    // }

    // return;


    // let limp = vec![vec![5usize, 5, 0, 3, 0, 5], vec![0, 5, 2, 3, 4, 5], vec![0, 1, 5, 3, 0, 5], vec![0, 1, 2, 5, 4, 5], vec![0, 1, 0, 3, 5, 5], vec![0, 1, 2, 3, 4, 5]];

    // eprintln!("{:?}", limp);
    // eprintln!("{:?}", l_alglib::l_alg_get_repr(&limp, true));
    // eprintln!("{:?}", l_alglib::l_alg_get_repr(&limp, false));

    // eprintln!("--");
    // let limp2 = vec![vec![5usize, 0, 0, 3, 5, 5], vec![0, 5, 0, 3, 4, 5], vec![0, 0, 5, 3, 4, 5], vec![0, 1, 2, 5, 4, 5], vec![0, 1, 2, 3, 5, 5], vec![0, 1, 2, 3, 4, 5]];
    // eprintln!("{:?}", limp2);
    // eprintln!("{:?}", l_alglib::l_alg_get_repr(&limp2, true));
    // eprintln!("{:?}", l_alglib::l_alg_get_repr(&limp2, false));
    // eprintln!("--");
    // let limp2 = vec![vec![5usize, 5, 2, 0, 0, 5], vec![0, 5, 2, 3, 4, 5], vec![0, 1, 5, 3, 4, 5], vec![0, 1, 2, 5, 0, 5], vec![0, 1, 2, 0, 5, 5], vec![0, 1, 2, 3, 4, 5]];
    // eprintln!("{:?}", limp2);
    // eprintln!("{:?}", l_alglib::l_alg_get_repr(&limp2, true));
    // eprintln!("{:?}", l_alglib::l_alg_get_repr(&limp2, false));
    
    // return;

    let file = BufReader::new(File::open("c:/users/mhycko/documents/rust/serde_test/all_qords6.pickle.bz2").expect("Cannot open file"));
    let mut bz_decoder = BzDecoder::new(file);
    let qords:Vec<Vec<Vec<usize>>> = serde_pickle::from_reader(&mut bz_decoder, Default::default()).unwrap();
    println!("{}", qords.len());
    let mut ord_count = 0usize;
    let n = 6;
    let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    // let mut qords = Vec::<Vec<Vec<usize>>>::new();

    // let mut qord = l_alglib::l_alg_alloc_limpl(n+1);
    // for i in 0..=n {
    //     for j in 0..=n{
    //         if i<=j {
    //             qord[i][j] = 1;
    //         }
    //     }
    // }
    // qords.push(qord);

    for qord in qords {
        if l_alglib::qord_is_antisymmetric(&qord) {
            ord_count+=1;
            if ord_count != 1 {
                continue;
            }

            // eprintln!("HHEHE");
            // get order n+1 with n as maximal element
            let mut qord_n1 = l_alglib::l_alg_alloc_limpl(n+1);
            for i in 0.. n {
                for j in 0..n {
                    qord_n1[i][j] = qord[i][j];
                }
            }
            for i in 0..(n+1) {
                qord_n1[i][n] = 1;
            }

            eprintln!("{qord_n1:?}");
            
            

            let mut falg = l_alglib::l_alg_alloc_limpl(n+1);
            let mut positions = Vec::<(usize,usize)>::new();
            

            


            l_alglib::l_alg_init_from_ord(&mut falg, &qord_n1, n, &mut positions);

            //falg[0][1] = 4;
            //falg[0][2] = 4;
            //falg[0][3] = 0;
            //positions.remove(0);
            //positions.remove(0);
            //positions.remove(0);

            // eprintln!("{:?}", positions);

            // println!("{falg:?}");

            // return;
            eprintln!("{positions:?}");
            
            l_alglib::gen_all_lalgs_rec(0, &positions, &mut falg, n, &mut lalgs, &mut num_tested, &mut num_models);

            eprintln!("{}", lalgs.len());
        }

    }
    eprintln!("{ord_count}");

    for falg in lalgs {
        println!("{falg:?}");
    }

    // println!("{}", qords.len());
    // println!("{:?}", qords[0]);


}

#[allow(dead_code)]
fn main7() {
    let n = 3;
    let lin_ord_3 = vec![vec![1,1,1],vec![0,1,1],vec![0,0,1]];
    let unit = 2;

    let mut falg = l_alglib::l_alg_alloc_limpl(n);
    let mut positions = Vec::<(usize,usize)>::new();

    l_alglib::l_alg_init_from_ord(&mut falg, &lin_ord_3, unit, &mut positions);

    eprintln!("{falg:?}");
    eprintln!("{positions:?}");
    let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    l_alglib::gen_all_lalgs_rec(0, &positions, &mut falg, unit, &mut lalgs, &mut num_tested, &mut  num_models);

    eprintln!("{lalgs:?}");
}


#[allow(dead_code)]
fn main6() {
    let n = 6usize;

    let mut lalg_impl = l_alglib::l_alg_alloc_limpl(n);
    let lalg_unit = 0usize;
    l_alglib::l_alg_init_limpl(&mut lalg_impl);

    //eprintln!("{lalg_impl:?}");

    // init positions

    let mut positions = Vec::<(usize,usize)>::new();

    for i in 1..n {
        for j in 1..n {
            if i != j {
                positions.push((i,j));
            }
        }
    }

    // generate algebras
    let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    l_alglib::gen_all_lalgs_rec(0, &positions, &mut lalg_impl, lalg_unit, &mut lalgs, &mut num_tested, &mut num_models);

    // eliminate isomorphic
    let mut lalgs_processed = HashSet::<Vec<Vec<usize>>>::new();
    let mut lalgs_unique = HashSet::<Vec<Vec<usize>>>::new();
    eprintln!(" lalgs size: {}", lalgs.len());
    for lalg in &lalgs {
        if !lalgs_processed.contains(lalg) {
            lalgs_unique.insert(lalg.clone());
            // println!("LL: {lalg:?}");
            for perm_ in (1..=(n-1)).permutations(n-1) {
                // eprintln!("{:?}", perm_);
                let mut iso_perm = Vec::<usize>::new();
                iso_perm.push(0);
                for i in 0..(n-1) {
                    iso_perm.push(perm_[i]);
                }
                // eprintln!("{:?}", iso_perm);

                lalgs_processed.insert(l_alglib::l_alg_isomorphic_image(lalg, lalg_unit, &iso_perm).0);
            }
            // eprintln!("{}:{lalgs_processed:?}", lalgs_processed.len());
            // eprintln!("--");
        }
    }

    eprintln!(" lalgs_unique size: {}", lalgs_unique.len());
    for lalg in &lalgs_unique {
        let mut already_processed = HashSet::<Vec<Vec<usize>>>::new();
        for perm in (1..=(n-1)).permutations(n-1) {
            let mut iso_perm = Vec::<usize>::new();
            iso_perm.push(0);
            for i in 0..(n-1) {
                iso_perm.push(perm[i]);
            }

            
            

            let iso_image = l_alglib::l_alg_isomorphic_image(lalg, lalg_unit, &iso_perm).0;
            if !already_processed.contains(&iso_image) {
                println!("{iso_image:?}");
                already_processed.insert(iso_image);
            }
        }

        // if l_alg_is_right_distributive(lalg).is_ok() {
        //println!("{lalg:?}");
        println!(" Commutative: {}", l_alglib::l_alg_is_commutative_l_algebra(lalg).is_ok());
        println!("  CL-algebra: {}", l_alglib::l_alg_is_cl_algebra(lalg).is_ok());
        println!("  KL-algebra: {}", l_alglib::l_alg_has_kl_property(lalg, 0).is_ok());
        println!("  Left Dist.: {}", l_alglib::l_alg_is_left_distributive(lalg).is_ok());
        println!(" Filters:");
        //println!(" Right Dist.: {}", l_alg_is_right_distributive(lalg).is_ok());
        l_alglib::l_alg_get_all_filters(lalg, lalg_unit);
        // }
    }
}




#[allow(dead_code)]
fn main33() {
    let mut limpl4 = vec![vec![0,1,2,3,4],vec![0,0,0,0,0],vec![0,0,0,0,0],vec![0,0,0,0,0],vec![0,0,0,0,0]];
    let unit4 = 0usize;
    let positions4 = Vec::from([(1usize,2usize),(1,3),(1,4),(2,1),(2,3),(2,4),(3,1),(3,2),(3,4),(4,1),(4,2),(4,3)]); 
    //let positions = Vec::from([(1usize,2usize),(1,3),(2,1),(2,3),(3,1),(3,2)]); 

    let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    
    
    l_alglib::gen_all_lalgs_rec(0, &positions4, &mut limpl4, unit4, &mut lalgs, &mut num_tested, &mut num_models);

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
            for perm_ in (1..=4).permutations(4) {
                // eprintln!("{:?}", perm_);
                let iso_perm = vec![0,perm_[0],perm_[1],perm_[2],perm_[3]];
                // eprintln!("{:?}", iso_perm);

                lalgs_processed.insert(l_alglib::l_alg_isomorphic_image(lalg, unit4, &iso_perm).0);
            }
            // eprintln!("{}:{lalgs_processed:?}", lalgs_processed.len());
            // eprintln!("--");
        }
    }


    eprintln!(" lalgs_unique size: {}", lalgs_unique.len());
    for lalg in &lalgs_unique {
        let mut already_processed = HashSet::<Vec<Vec<usize>>>::new();
        for perm in (1..=4).permutations(4) {
            let iso_perm = vec![0,perm[0],perm[1],perm[2],perm[3]];

            let iso_image = l_alglib::l_alg_isomorphic_image(lalg, 0, &iso_perm).0;
            if !already_processed.contains(&iso_image) {
                println!("{iso_image:?}");
                already_processed.insert(iso_image);
            }
        }

        // if l_alg_is_right_distributive(lalg).is_ok() {
        //println!("{lalg:?}");
        println!(" Commutative: {}", l_alglib::l_alg_is_commutative_l_algebra(lalg).is_ok());
        println!("  CL-algebra: {}", l_alglib::l_alg_is_cl_algebra(lalg).is_ok());
        println!("  KL-algebra: {}", l_alglib::l_alg_has_kl_property(lalg, 0).is_ok());
        println!("  Left Dist.: {}", l_alglib::l_alg_is_left_distributive(lalg).is_ok());
        println!(" Filters:");
        //println!(" Right Dist.: {}", l_alg_is_right_distributive(lalg).is_ok());
        l_alglib::l_alg_get_all_filters(lalg, unit4);
        // }
    }
}




#[allow(dead_code)]
fn main4() {
    let mut ex3_limpl = vec![vec![0,1,2,3],vec![0,0,3,2],vec![0,3,0,1],vec![0,2,1,0]];
    let ex3_unit = 0usize;
    let positions = Vec::from([(1usize,2usize),(1,3),(2,1),(2,3),(3,1),(3,2)]); 
    //let positions = Vec::from([(1usize,2usize),(1,3),(2,1),(2,3),(3,1),(3,2)]); 

    let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    
    
    l_alglib::gen_all_lalgs_rec(0, &positions, &mut ex3_limpl, ex3_unit, &mut lalgs, &mut num_tested, &mut num_models);

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

                lalgs_processed.insert(l_alglib::l_alg_isomorphic_image(lalg, ex3_unit, &iso_perm).0);
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

            let iso_image = l_alglib::l_alg_isomorphic_image(lalg, 0, &iso_perm).0;
            if !already_processed.contains(&iso_image) {
                println!("{iso_image:?}");
                already_processed.insert(iso_image);
            }
        }

        // if l_alg_is_right_distributive(lalg).is_ok() {
        //println!("{lalg:?}");
        println!(" Commutative: {}", l_alglib::l_alg_is_commutative_l_algebra(lalg).is_ok());
        println!("  CL-algebra: {}", l_alglib::l_alg_is_cl_algebra(lalg).is_ok());
        println!("  KL-algebra: {}", l_alglib::l_alg_has_kl_property(lalg, 0).is_ok());
        println!("  Left Dist.: {}", l_alglib::l_alg_is_left_distributive(lalg).is_ok());
        println!(" Filters:");
        //println!(" Right Dist.: {}", l_alg_is_right_distributive(lalg).is_ok());
        l_alglib::l_alg_get_all_filters(lalg, ex3_unit);
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
                    if l_alglib::l_alg_is_l_algebra(&ex3_limpl, ex3_unit, false) {
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
        l_alglib::l_alg_get_all_filters(la, ex3_unit);

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

#[allow(dead_code)]
fn main1() {
    let ex1_limpl = vec![vec![0,1,2,3],vec![0,0,0,1],vec![0,1,0,3],vec![0,1,0,0]];
    let ex1_unit = 0;

    println!("Example 1");
    println!("  L-algebra: {}", l_alglib::l_alg_is_l_algebra(&ex1_limpl, ex1_unit, true));
    let ex1_subset1 = HashSet::from([0usize]);
    let ex1_subset2 = HashSet::from([0usize, 2]);
    let ex1_subset3 = HashSet::from([0usize, 2, 3]);

    println!(" {:?}: {}", ex1_subset1, l_alglib::l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset1, true));
    println!(" {:?}: {}", ex1_subset2, l_alglib::l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset2, true));
    println!(" {:?}: {}", ex1_subset3, l_alglib::l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset3, true));
    let ex1_subset4 = HashSet::from([0usize, 1]);
    println!(" {:?}: {}", ex1_subset4, l_alglib::l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset4, true));
    let ex1_subset4 = HashSet::from([0usize, 1,2]);
    println!(" {:?}: {}", ex1_subset4, l_alglib::l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset4, true));
    let ex1_subset4 = HashSet::from([0usize, 1,3]);
    println!(" {:?}: {}", ex1_subset4, l_alglib::l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset4, true));
    let ex1_subset4 = HashSet::from([0usize, 3]);
    println!(" {:?}: {}", ex1_subset4, l_alglib::l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset4, true));
    let ex1_subset4 = HashSet::from([0usize, 1,2,3]);
    println!(" {:?}: {}", ex1_subset4, l_alglib::l_alg_is_filter(&ex1_limpl, ex1_unit, &ex1_subset4, true));

    let ex3_limpl = vec![vec![0,1,2,3],vec![0,0,3,2],vec![0,3,0,1],vec![0,2,1,0]];
    let ex3_unit = 0;
    println!("Example 3");
    println!("  L-algebra: {}", l_alglib::l_alg_is_l_algebra(&ex3_limpl, ex3_unit, true));
    let ex3_subset1 = HashSet::from([0usize, 1]);
    println!(" {:?}: {}", ex3_subset1, l_alglib::l_alg_is_filter(&ex3_limpl, ex3_unit, &ex3_subset1, true));

    let ex4_limpl = vec![vec![4,4,4,4,4],vec![0,4,4,4,4],vec![0,3,4,3,4],vec![0,2,2,4,4],vec![0,1,2,3,4]];
    let ex4_unit = 4usize;
    println!("Example 4");
    println!("  L-algebra: {}", l_alglib::l_alg_is_l_algebra(&ex4_limpl, ex4_unit, true));

    let ex5_limpl = vec![vec![3,3,3,3],vec![2,3,3,3],vec![1,1,3,3],vec![0,1,2,3]];
    let ex5_unit = 3;
    println!("Example 5");
    println!("  L-algebra: {}",l_alglib::l_alg_is_l_algebra(&ex5_limpl, ex5_unit, true));

    let ex5p_limpl = vec![vec![3,3,3,3],vec![2,3,2,3],vec![1,1,3,3],vec![0,1,2,3]];
    let ex5p_unit = 3;
    println!("Example 5'");
    println!("  L-algebra: {}", l_alglib::l_alg_is_l_algebra(&ex5p_limpl, ex5p_unit, true));
    println!(" KL-algebra: {}", l_alglib::l_alg_has_kl_property(&ex5p_limpl, ex5p_unit).is_ok());

    let ex5p2_limpl = vec![vec![3,3,3,3],vec![2,3,3,3],vec![1,2,3,3],vec![0,1,2,3]];
    let ex5p2_unit = 3;
    println!("Example 5''");
    println!("  L-algebra: {}", l_alglib::l_alg_is_l_algebra(&ex5p2_limpl, ex5p2_unit, true));
    println!(" KL-algebra: {}", l_alglib::l_alg_has_kl_property(&ex5p2_limpl, ex5p2_unit).is_ok());
    let ex5p2_subset1 = HashSet::from([1usize, 2, 3]);
    println!(" {:?}: {}", ex5p2_subset1, l_alglib::l_alg_is_filter(&ex5p2_limpl, ex5p2_unit, &ex5p2_subset1, true));

    let ex7_limpl = vec![vec![0,1,2,3],vec![0,0,3,2],vec![0,3,0,1],vec![0,2,1,0]];
    let ex7_unit = 0;
    println!("Example 7");
    println!("  L-algebra: {}", l_alglib::l_alg_is_l_algebra(&ex7_limpl, ex7_unit, true));

    let ex7p_limpl = vec![vec![0,1,2,3],vec![0,0,3,2],vec![0,3,0,1],vec![0,2,1,0]];
    let ex7p_unit = 0;
    println!("Example 7''");
    println!("  L-algebra: {}", l_alglib::l_alg_is_l_algebra(&ex7p_limpl, ex7p_unit, true));

    let ex3_limpl = vec![vec![0,1,2,3],vec![0,0,3,2],vec![0,3,0,1],vec![0,2,1,0]];
    let ex3_unit = 0;
    println!("Example 3");
    println!("  L-algebra: {}", l_alglib::l_alg_is_l_algebra(&ex3_limpl, ex3_unit, true));
    let ex3_subset1 = HashSet::from([0usize, 1]);
    println!(" {:?}: {}", ex3_subset1, l_alglib::l_alg_is_filter(&ex3_limpl, ex3_unit, &ex3_subset1, true));

    

}
