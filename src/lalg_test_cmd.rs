use std::{collections::HashSet};
use std::time::Instant;
use std::fmt::Write;

// x -> x = 1
fn lalg_l1(lalg: &Vec<Vec<usize>>) -> bool {
    let n = lalg.len();

    for i in 0..n {
        if lalg[i][i] != n - 1 {
            return false;
        }
    }
    true
}

// x -> 1 = 1
fn lalg_l2(lalg: &Vec<Vec<usize>>) -> bool {
    let n = lalg.len();

    for i in 0..n {
        if lalg[i][n-1] != n - 1 {
            return false;
        }
    }
    true
}

// 1 -> x = x
fn lalg_l3(lalg: &Vec<Vec<usize>>) -> bool {
    let n = lalg.len();

    for i in 0..n {
        if lalg[n-1][i] != i {
            return false;
        }
    }
    true
}

// (x -> y) -> (x -> z) = (y -> x) -> (y -> z)
fn lalg_l4(lalg: &Vec<Vec<usize>>) -> bool {
    let n = lalg.len();

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if lalg[lalg[i][j]][lalg[i][k]] != lalg[lalg[j][i]][lalg[j][k]] {
                    return false;
                }
            }
        }
    }
    true
}

// (x -> y) = 1 && (y -> x) = 1 ==> x = y
fn lalg_l5(lalg: &Vec<Vec<usize>>) -> bool {
    let n = lalg.len();

    for i in 0..n {
        for j in 0..n {
            if i!=j && lalg[i][j]==n-1 && lalg[j][i]==n-1 {
                return false;
            }
        }
    }
    true
}

// x -> (y -> x) = y -> (x -> y)
fn lalg_self_similar(lalg: &Vec<Vec<usize>>)  -> Result<bool, String> {
    let n = lalg.len();
    let mut outerr_str = String::new();
    let mut b_first_err = true;
    let mut b_err = false;
    for i in 0..n {
        for j in 0..n {
            if lalg[i][lalg[j][i]] != lalg[j][lalg[j][i]] {
                b_err=true;
                if b_first_err {
                    b_first_err = false;
                }
                else {
                    write!(outerr_str, ", ").unwrap();
                    // outerr_str += ", ".to_string();
                }
                write!(outerr_str, "(x,y)=({},{})", i, j).unwrap();
            }
        }
    }
    if b_err {
        return Err(outerr_str);
    }

    Ok(true)
}

// (x -> y) -> y = (y -> x) -> x
fn lalg_commutative(lalg: &Vec<Vec<usize>>)  -> Result<bool, String> {
    let n = lalg.len();
    let mut outerr_str = String::new();
    let mut b_first_err = true;
    let mut b_err = false;
    for i in 0..n {
        for j in 0..n {
            if lalg[lalg[i][j]][j] != lalg[lalg[j][i]][i] {
                b_err=true;
                if b_first_err {
                    b_first_err = false;
                }
                else {
                    write!(outerr_str, ", ").unwrap();
                    // outerr_str += ", ".to_string();
                }
                write!(outerr_str, "(x,y)=({},{})", i, j).unwrap();
            }
        }
    }
    if b_err {
        return Err(outerr_str);
    }

    Ok(true)
}

// x -> (y -> z) = (x -> y) -> (x -> z)
fn lalg_self_distributive(lalg: &Vec<Vec<usize>>)  -> Result<bool, String> {
    let n = lalg.len();
    let mut outerr_str = String::new();
    let mut b_first_err = true;
    let mut b_err = false;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if lalg[i][lalg[j][k]] != lalg[lalg[i][j]][lalg[i][k]] {
                    b_err=true;
                    if b_first_err {
                        b_first_err = false;
                    }
                    else {
                        write!(outerr_str, ", ").unwrap();
                        // outerr_str += ", ".to_string();
                    }
                    write!(outerr_str, "(x,y,z)=({},{},{})", i, j, k).unwrap();
                }
            }
        }
    }
    if b_err {
        return Err(outerr_str);
    }

    Ok(true)
}

// x -> (y -> z) = y -> (x -> z)
fn lalg_cl_algebra(lalg: &Vec<Vec<usize>>)  -> Result<bool, String> {
    let n = lalg.len();
    let mut outerr_str = String::new();
    let mut b_first_err = true;
    let mut b_err = false;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if lalg[i][lalg[j][k]] != lalg[j][lalg[i][k]] {
                    b_err=true;
                    if b_first_err {
                        b_first_err = false;
                    }
                    else {
                        write!(outerr_str, ", ").unwrap();
                        // outerr_str += ", ".to_string();
                    }
                    write!(outerr_str, "(x,y,z)=({},{},{})", i, j, k).unwrap();
                }
            }
        }
    }
    if b_err {
        return Err(outerr_str);
    }

    Ok(true)
}

// x <= a -> x
fn lalg_kl_algebra(lalg: &Vec<Vec<usize>>)  -> Result<bool, String> {
    let n = lalg.len();
    let mut outerr_str = String::new();
    let mut b_first_err = true;
    let mut b_err = false;
    for i in 0..n {
        for j in 0..n {
            if lalg[i][lalg[j][i]] != n - 1 {
                b_err=true;
                if b_first_err {
                    b_first_err = false;
                }
                else {
                    write!(outerr_str, ", ").unwrap();
                    // outerr_str += ", ".to_string();
                }
                write!(outerr_str, "(x,a)=({},{})", i, j).unwrap();
            }
        }
    }
    if b_err {
        return Err(outerr_str);
    }

    Ok(true)
}

fn main() {
   
    let args_len = std::env::args().len();

    if args_len < 2 {
        println!("Usage: {} <l-algebra>", std::env::args().next().unwrap());
        return;
    }

    let lalg_init_string = std::env::args().nth(1).unwrap();
    
    eprintln!("Input L-algebra (str): {}", lalg_init_string);

    let lalg_res = serde_json::from_str::<Vec<Vec<usize>>>(&lalg_init_string);
    
    let lalg = match lalg_res {
        Ok(p) => p,
        Err(err) => {eprintln!("Error parsing json (pord): {err:?}"); return;}
    };
    
    let n = lalg.len();
    
    eprintln!("Input L-algebra: {:?}", lalg);
    println!("L1: {}", lalg_l1(&lalg));
    println!("L2: {}", lalg_l1(&lalg));
    println!("L3: {}", lalg_l1(&lalg));
    println!("L4: {}", lalg_l1(&lalg));
    println!("L5: {}", lalg_l1(&lalg));
    println!("---");
    println!("Self-similar: {:?}", lalg_self_similar(&lalg));
    println!("Commutative: {:?}", lalg_commutative(&lalg));
    println!("Self-distributive: {:?}", lalg_self_distributive(&lalg));
    println!("CL-algebra: {:?}", lalg_cl_algebra(&lalg));
    println!("KL-algebra: {:?}", lalg_kl_algebra(&lalg));
}
