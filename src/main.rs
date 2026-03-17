// use core::num;
use std::collections::HashMap;
use std::{collections::HashSet, io::BufRead};
use itertools::{Itertools};
// use l_alglib::l_alg_init_limpl;
use std::fs::File;
use bzip2::read::{BzDecoder};
use std::io::{BufReader};
use std::time::{Instant};

// #[allow(dead_code)]
// fn parse_vector(line: &String) -> Vec<Vec<usize>> {
//         let mut parsed_vector = Vec::<Vec<usize>>::new();
//         for t in line.split("[") {
//             if t=="" {
//                 continue;
//             }
//             let mut tt = String::from(t);
            
//             let gg = tt.find("], ");
//             if gg.is_some() {
//                 let pos = gg.unwrap();
//                 tt.remove(pos);
//                 tt.remove(pos);
//                 tt.remove(pos);
//             }
//             let gg = tt.find("]]");
//             if gg.is_some() {
//                 let pos = gg.unwrap();
//                 tt.remove(pos);
//                 tt.remove(pos);
//             }
//             let vv = tt.split(", ").map(|v| {//println!("v = {v:?}"); 
//             v.trim().parse::<usize>().unwrap()}).collect::<Vec<_>>();
//             // println!("{vv:?}");
//             parsed_vector.push(vv);

//             //tt.remove_matches("]]");
//             //println!("{tt:?}");
//         }
//         // println!("{parsed_vector:?}");

//         parsed_vector

// }


//#![feature(string_remove_matches)]

fn hashmap_perm_image(fun: &HashMap::<(usize,usize), usize>, perm: &Vec<usize>) -> HashMap::<(usize,usize), usize> {
    let mut res_hm = HashMap::<(usize,usize), usize>::new();
    

    for k in fun.keys() {
        let v = fun[k];
        res_hm.insert((perm[k.0],perm[k.1]), perm[v]);
    }
    res_hm
}

fn get_images(perms_set: &HashSet::<Vec<usize>>, fun: &HashMap::<(usize,usize), usize>) -> Vec<Vec<usize>>{
    let mut keys_sorted = fun.keys().collect::<Vec<_>>();
    keys_sorted.sort();

    let mut hs = HashSet::<Vec<usize>>::new();
    for perm in perms_set {
        let hh_img = hashmap_perm_image(fun, perm);
        // get vector
        let mut vv = Vec::<usize>::new();
        for k in &keys_sorted {
            vv.push(hh_img[k]);
        }
        hs.insert(vv);
     }

     let mut hs_v = hs.into_iter().collect::<Vec<_>>();
     hs_v.sort();

     hs_v
 }

 fn get_images2(perms_set: impl Iterator<Item=Vec<usize>>, fun: &HashMap::<(usize,usize), usize>) -> Vec<Vec<usize>> {

    let mut keys_sorted = fun.keys().collect::<Vec<_>>();
    keys_sorted.sort();

    let mut hs = HashSet::<Vec<usize>>::new();
    for perm in perms_set {
         let hh_img = hashmap_perm_image(fun, &perm);
         // get vector
         let mut vv = Vec::<usize>::new();
         for k in &keys_sorted {
             vv.push(hh_img[k]);
         }
         hs.insert(vv);
    }

    let mut hs_v = hs.into_iter().collect::<Vec<_>>();
    hs_v.sort();

     hs_v
 }


fn main() {
    // {
    //     let tt = (0..5).map(|i| 0..=1);
    //     // for t in tt {
    //     //      println!("{t:?}");
    //     // }

    //     let mut multi_prod = tt.multi_cartesian_product();

    //     for f in multi_prod {
    //         println!("{f:?}");
    //     }
    // }
    // return;
    
    // 1728
    let pord = vec![vec![1, 1, 1, 1, 1, 1, 1, 1], vec![0, 1, 0, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];

    // 408
    // let pord = vec![vec![1usize, 1, 0, 0, 1, 0, 0, 1], vec![0, 1, 0, 0, 1, 0, 0, 1], vec![0, 0, 1, 0, 1, 0, 0, 1], vec![0, 0, 0, 1, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
    // 338
    // let pord = vec![vec![1usize, 1, 0, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
    // 6
    // let pord = vec![vec![1, 0, 0, 0, 1, 0, 0, 1], vec![0, 1, 0, 0, 1, 0, 0, 1], vec![0, 0, 1, 0, 1, 0, 0, 1], vec![0, 0, 0, 1, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
    // 376: 1st level: (0,2), (0,3), (0,4), (0,6); 2nd level + (1,2), (1,3), (1,4), (1,6); 3rd level + (5,6)
    // let pord = vec![vec![1, 1, 0, 0, 0, 1, 0, 1], vec![0, 1, 0, 0, 0, 1, 0, 1], vec![0, 0, 1, 0, 0, 1, 0, 1], vec![0, 0, 0, 1, 0, 1, 0, 1], vec![0, 0, 0, 0, 1, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
    // 5
    // let pord = vec![vec![1, 0, 0, 0, 0, 1, 1, 1], vec![0, 1, 0, 0, 0, 1, 1, 1], vec![0, 0, 1, 0, 0, 1, 1, 1], vec![0, 0, 0, 1, 0, 1, 1, 1], vec![0, 0, 0, 0, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
    // 4
    // let pord = vec![vec![1, 0, 0, 0, 0, 1, 1, 1], vec![0, 1, 0, 0, 0, 1, 1, 1], vec![0, 0, 1, 0, 0, 1, 1, 1], vec![0, 0, 0, 1, 0, 1, 1, 1], vec![0, 0, 0, 0, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
    // 3: 1st: (0,1), (0,2), (0,3), (0,4), (0,6); 2nd: + (5,6)
    // let pord = vec![vec![1, 0, 0, 0, 0, 1, 0, 1], vec![0, 1, 0, 0, 0, 1, 0, 1], vec![0, 0, 1, 0, 0, 1, 0, 1], vec![0, 0, 0, 1, 0, 1, 0, 1], vec![0, 0, 0, 0, 1, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
    // 2: (0,1), (0,2), (0,3), (0,4), (0,5)
    // let pord = vec![vec![1, 0, 0, 0, 0, 0, 1, 1], vec![0, 1, 0, 0, 0, 0, 1, 1], vec![0, 0, 1, 0, 0, 0, 1, 1], vec![0, 0, 0, 1, 0, 0, 1, 1], vec![0, 0, 0, 0, 1, 0, 1, 1], vec![0, 0, 0, 0, 0, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
    // 1:
    // let pord = vec![vec![1, 0, 0, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 1]];
    
    let mut hh = HashMap::<(usize,usize), usize>::new();
    let tt = (0..5).map(|i| 0..7usize);
    let mut num_cls = 0usize;
    let ts = Instant::now();
    let fixed = vec![0,0,0,5,6];
    for var in tt.multi_cartesian_product() {
        // hh.insert((1,2), var[0]);
        // hh.insert((1,3), var[1]);
        // hh.insert((1,4), var[2]);
        // hh.insert((1,5), var[3]);
        // hh.insert((1,6), var[4]);
        hh.insert((1,2), fixed[0]);
        hh.insert((1,3), fixed[1]);
        hh.insert((1,4), fixed[2]);
        hh.insert((1,5), fixed[3]);
        hh.insert((1,6), fixed[4]);
        hh.insert((2,1), var[0]);
        hh.insert((2,3), var[1]);
        hh.insert((2,4), var[2]);
        hh.insert((2,5), var[3]);
        hh.insert((2,6), var[4]);

        // hh.insert((0,6), var[4]);
        // hh.insert((4,5), var[5]);
        // hh.insert((4,6), var[6]);

        let pp = (0usize..pord.len()).collect::<Vec<_>>();
        let jj = get_images2(pp.into_iter().permutations(pord.len())
            .filter(|pe| pe[1]==1 && pe[2] ==2 || pe[1]==2 && pe[2] == 1)
            .filter(|pe| l_alglib::pord_perm_preserve_ord(&pord, &pe)), &hh);
        // for perm in pp.into_iter().permutations(pord.len())
        //     .filter(|pe| pe[0]==0 && pe[1]==1 && pe[4]==4 && l_alglib::pord_perm_preserve_ord(&pord, &pe)) {
        //     println!("{perm:?}");
        // }
        // println!("DBG: {jj:?}");
        // println!("{:?}, {:?}", jj[0].clone().into_iter().skip(1).collect::<Vec<_>>(), var);
        //if jj[0].clone().into_iter().skip(1).collect::<Vec<_>>() == var {
        // eprintln!("{:?}, {:?}", jj[0], var);
        // let t = &jj[0][5..6];
        // eprintln!("{:?}", t);
        // break;
        if &jj[0][0..5] == fixed && &jj[0][5..10] == var {
            num_cls+=1;
            if false {
            let siz =  jj.len();       
            // println!("{}", siz);
            let mut b_first = true;
            let mut b_second = true;
            for vec_j in jj.iter() {
                if b_first {
                    print!("  \\item $\\mathbf{{");
                    for e in vec_j {
                        print!("{}", *e);
                    }
                    print!("}}$ ({}): ", siz);
                    b_first = false;
                }
                else {
                    if !b_second {
                        print!(", ");
                    }
                    else {
                        b_second = false;
                    }
                    
                    print!("${{");
                    for e in vec_j {
                        print!("{}", *e);
                    }
                    print!("}}$");
                }

            }
            //println!("{:?}", jj);
            println!("\n\n");
            }
            else {
                // for 1728
                // println!("./target/release/gen_from_ord.exe 1728 9,{},{},{},{},{},9,{},{},{},{},{} 1> rc8sym-1728/hh7_pord_1728-{}{}{}{}{}_{}{}{}{}{}.txt 2> rc8sym-1728/hh7_pord_1728-{}{}{}{}{}_{}{}{}{}{}.log", var[0], var[1], var[2], var[3], var[4], var[5], var[6], var[7], var[8], var[9], var[0], var[1], var[2], var[3], var[4], var[5], var[6], var[7], var[8], var[9], var[0], var[1], var[2], var[3], var[4], var[5], var[6], var[7], var[8], var[9]);
                println!("./target/release/gen_from_ord.exe 1728 9,{},{},{},{},{},9,{},{},{},{},{} 1> rc8sym-1728/hh8_pord_1728-{}{}{}{}{}_{}{}{}{}{}.txt 2> rc8sym-1728/hh8_pord_1728-{}{}{}{}{}_{}{}{}{}{}.log", fixed[0], fixed[1], fixed[2], fixed[3], fixed[4], var[0], var[1], var[2], var[3], var[4], fixed[0], fixed[1], fixed[2], fixed[3], fixed[4], var[0], var[1], var[2], var[3], var[4], fixed[0], fixed[1], fixed[2], fixed[3], fixed[4], var[0], var[1], var[2], var[3], var[4]);

                // println!("./target/release/gen_from_ord.exe 1728 9,0,0,1,2,3,9,{},{},{},{},{} 1> rc8sym-1728/hh7_pord_1728-00123_{}{}{}{}{}.txt 2> rc8sym-1728/hh7_pord_1728-00123_{}{}{}{}{}.log", var[0], var[1], var[2], var[3], var[4], var[0], var[1], var[2], var[3], var[4], var[0], var[1], var[2], var[3], var[4]);
            }
        }    
    
    }
    eprintln!("{num_cls}");
    eprintln!("Time elapsed: {:.2} s", ts.elapsed().as_secs_f32());
    return;
    let pord = vec![vec![1usize, 0, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 0, 1], vec![0, 0, 0, 1, 0, 0, 1], vec![0, 0, 0, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 1]];
    
    let tt = (0..5).map(|i| 0..=5usize);
    let mut num_cls = 0usize;
    for var in tt.multi_cartesian_product() {

        let pp = (0usize..7).collect::<Vec<_>>();

        let mut hh = HashMap::<(usize,usize), usize>::new();
        hh.insert((0,1), var[0]);
        hh.insert((0,2), var[1]);
        hh.insert((0,3), var[2]);
        hh.insert((0,4), var[3]);
        hh.insert((0,5), var[4]);
        
        
        let jj = get_images2(pp.into_iter().permutations(7).filter(|pe| pe[0]==0 && l_alglib::pord_perm_preserve_ord(&pord, &pe)), &hh);
        
        if jj[0] == var {
            num_cls+=1;
            let siz =  jj.len();       
            // println!("{}", siz);
            let mut b_first = true;
            let mut b_second = true;
            for vec_j in jj.iter() {
                if b_first {
                    print!("  \\item $\\mathbf{{{}{}{}{}{}}}$ ({}): ", vec_j[0], vec_j[1], vec_j[2], vec_j[3], vec_j[4], jj.len());
                    b_first = false;
                }
                else {
                    if b_second {
                        print!("${{{}{}{}{}{}}}$", vec_j[0], vec_j[1], vec_j[2], vec_j[3], vec_j[4]);
                        b_second = false;
                    }
                    else {
                        print!(", ${{{}{}{}{}{}}}$", vec_j[0], vec_j[1], vec_j[2], vec_j[3], vec_j[4]);
                    }
                }

            }
            //println!("{:?}", jj);
            println!("\n\n");
        }
        // for perm in pp.into_iter().permutations(7) {
        //     if l_alglib::pord_perm_preserve_ord(&pord, &perm) {
        //         println!("{:?}", perm);
        //     }
        // }
    }
    
    eprintln!("{num_cls}");
    return;
    let mut perms_set = HashSet::<Vec<usize>>::new();
    perms_set.insert(vec![0,1,2,3,4,5,6,7]);
    perms_set.insert(vec![0,1,2,3,5,4,6,7]);
    perms_set.insert(vec![0,1,2,4,3,5,6,7]);
    perms_set.insert(vec![0,1,2,4,5,3,6,7]);
    perms_set.insert(vec![0,1,2,5,3,4,6,7]);
    perms_set.insert(vec![0,1,2,5,4,3,6,7]);
    perms_set.insert(vec![0,2,1,3,4,5,6,7]);
    perms_set.insert(vec![0,2,1,3,5,4,6,7]);
    perms_set.insert(vec![0,2,1,4,3,5,6,7]);
    perms_set.insert(vec![0,2,1,4,5,3,6,7]);
    perms_set.insert(vec![0,2,1,5,3,4,6,7]);
    perms_set.insert(vec![0,2,1,5,4,3,6,7]);

    let mut hh = HashMap::<(usize,usize), usize>::new();
    hh.insert((0,3), 4);
    hh.insert((0,4), 6);
    hh.insert((0,5), 6);
    
    println!("{:?}", get_images(&perms_set, &hh));
    return;


    
    let positions = Vec::from([(0usize, 3usize), (0, 4), (0, 5), (1, 0), (1, 2), (1, 3), (1, 4), (1, 5), (2, 0), (2, 1), (2, 3), (2, 4), (2, 5), (3, 0), (3, 1), (3, 2), (3, 4), (3, 5), (4, 0), (4, 1), (4, 2), (4, 3), (4, 5), (5, 0), (5, 1), (5, 2), (5, 3), (5, 4), (6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5)]);

    for pos in positions {
        let mut tmp_vec = Vec::<(usize,usize)>::new();
        for per in &perms_set {
            tmp_vec.push((per[pos.0], per[pos.1]));
        }
        tmp_vec.sort();
        println!("{tmp_vec:?}");
    }

    return;
    let mut hh = HashMap::<(usize,usize), usize>::new();
    hh.insert((1,2), 0);
    hh.insert((1,3), 0);
    hh.insert((1,4), 6);
    hh.insert((1,5), 1);

    let pp = vec![2,3,4,5];

    

    let mut tt = HashSet::<Vec<usize>>::new();
    let mut full_perm = [0usize,1,2,3,4,5,6,7];
    for perm in pp.iter().permutations(4) {
        
        for i in 0..4 {
            full_perm[i+2] = *perm[i];
        }
        let mut is_ok = true;
        for t in hh.keys() {
            let k = (full_perm[t.0], full_perm[t.1]);
            if !hh.contains_key(&k) {
                is_ok = false;
                break;
            }
        }
        let mut hh2 = HashMap::<(usize,usize), usize>::new();
        if is_ok {
            for t in hh.keys() {
                let k = (full_perm[t.0], full_perm[t.1]);
                let v = hh[&(t.0,t.1)];
                hh2.insert(k, full_perm[v]);
            }

            let mut hh2_keys = hh2.keys().collect::<Vec<_>>();
            hh2_keys.sort();

            let mut v = Vec::<usize>::new();
            for k in hh2_keys {
                v.push(hh2[k]);
            }
            // println!("{:?}", v);
            tt.insert(v);
            //println!("PP: {:?}", full_perm);
            // eprintln!("{:?}", hh2);
            // tt.insert(hh2);
            // to_vec

        }
    }
    
    println!("{}", tt.len());
    let mut tt_vec = tt.iter().collect::<Vec<_>>();
    tt_vec.sort();
    for v in tt_vec {
        println!("{:?}", v);
    }
    // eprintln!("{:?}", hh);
    return;

    let mut hs = HashSet::<Vec<usize>>::new();
    // let mut vv= vec![0usize,0,0,0,2];
    // let mut vv2= vec![0usize,0,0,0,0];
    let mut vv= vec![0usize,0,0,5];
    let mut vv2= vec![0usize,0,0,0];

    // let n = vv.len();
    // let mut pp = Vec::<usize>::new();
    // for i in 2..=n {
    //     pp.push(i);
    // }
    let n = 4usize;

    //let pp = [0usize;n];
    // let pp = vec![2,3,4,5,6];
    let pp = vec![2,3,4,5];

    for perm in pp.iter().permutations(n) {
        for i in 0..n {
            if vv[i] != 0 && vv[i] != 1 {
                vv2[perm[i]-2] = *perm[vv[i]-2];
            }
            else {
                vv2[perm[i]-2] = vv[i];
            }
        }
        //println!("{vv2:?}");
        hs.insert(vv2.clone());
        // for v in &perm {
        //     if vv[*v -1] == 0 {
        //         vv2[*v - 1] = 0;
        //     }
        //     else {
        //         vv2[*v-1] = *perm[vv[*v-1]-1];
        //     }
        // }
        
        //println!("{:?}", perm);
    }

    let mut h = hs.iter().collect::<Vec<_>>();
    h.sort();
    println!("{}", h.len());
    for hh in h {
        println!("{hh:?}");
    }
    //println!("{h:?}");

    return;
    // let n = 7usize;
    // let lalg_unit = n-1;
    // let file = BufReader::new(File::open("ord7_with_top.txt").expect("Cannot open file"));
    // let pord_num = 300;
    // // let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
    // let mut cur_line_no = 0usize;
    // for line in file.lines() {
    //     let cur_line = line.unwrap();
    //     cur_line_no+=1;
        

    //     if cur_line_no == pord_num {
    //         let pord = serde_json::from_str::<Vec<Vec<usize>>>(&cur_line).unwrap();

    //         let mut base_perm_vec = Vec::<usize>::new();
    //         let mut iso_perm_vec= Vec::<usize>::new();
    


    //         for i in 0..n {
    //             if i != lalg_unit {
    //                 base_perm_vec.push(i);
    //             }
    //             iso_perm_vec.push(i);

    //         }   
    //         let mut cnt = 0usize;
    //         for perm in base_perm_vec.iter().permutations(n-1) {
    //             for j in 0..n-1 {
    //                 iso_perm_vec[base_perm_vec[j]] = *perm[j];
    //             }
    //             let mut b_preserve_ord = true;
    //             for i in 0..n {
    //                 for j in (i+1)..n {
    //                     if pord[i][j] == 1 && iso_perm_vec[i] > iso_perm_vec[j] {
    //                         b_preserve_ord = false;
    //                         break;
    //                     }
    //                 }
    //                 if !b_preserve_ord {
    //                     break;
    //                 }
    //             }
    //             if b_preserve_ord {
    //                 eprintln!("{iso_perm_vec:?}");
    //                 cnt+=1;
    //             }
    //         }
    //         eprintln!("{cnt}");

    //     }
        
    // }
    // return ;
    
    // use serde_json::{Result, Value};
    // let v_str = "[[0,1,2],[2,3,4],[4,5,6]]";

    // let v = serde_json::from_str::<Vec<Vec<usize>>>(v_str).unwrap();
    // eprintln!("{v:?}");
    // return;
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

    let file = BufReader::new(File::open("c:/users/mhycko/documents/rust/serde_test/all_qords5.pickle.bz2").expect("Cannot open file"));
    let mut bz_decoder = BzDecoder::new(file);
    let qords:Vec<Vec<Vec<usize>>> = serde_pickle::from_reader(&mut bz_decoder, Default::default()).unwrap();
    println!("{}", qords.len());
    let mut ord_count = 0usize;
    let n = 5;
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
            // if ord_count != 1 {
            //     continue;
            // }

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

            println!("{qord_n1:?}");
        }

    }
    eprintln!("{ord_count}");

    return;



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
