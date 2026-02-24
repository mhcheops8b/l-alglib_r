use std::collections::{HashSet};
use itertools::{Itertools};

pub fn l_alg_ax1(limpl: &Vec<Vec<usize>>, unit: usize) -> bool {
    let n = limpl.len();

    for x in 0..n {
        if limpl[x][x] != unit {
            return false;
        }
    }
    true
}

pub fn l_alg_ax2(limpl: &Vec<Vec<usize>>, unit: usize) -> bool {
    let n = limpl.len();

    for x in 0..n {
        if limpl[x][unit] != unit {
            return false;
        }
    }
    true
}

pub fn l_alg_ax3(limpl: &Vec<Vec<usize>>, unit: usize) -> bool {
    let n = limpl.len();

    for x in 0..n {
        if limpl[unit][x] != x {
            return false;
        }
    }
    true
}

pub fn l_alg_ax4(limpl: &Vec<Vec<usize>>, bprint: bool) -> bool {
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

pub fn l_alg_ax5(limpl: &Vec<Vec<usize>>, unit: usize) -> bool {
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

pub fn l_alg_is_l_algebra(limpl: &Vec<Vec<usize>>, unit: usize, bprint: bool) -> bool {
    l_alg_ax1(limpl, unit) &&
    l_alg_ax2(limpl, unit) &&
    l_alg_ax3(limpl, unit) &&
    l_alg_ax4(limpl, bprint) &&
    l_alg_ax5(limpl, unit) 
}

pub fn l_alg_has_kl_property_old(limpl: &Vec<Vec<usize>>, unit: usize) -> bool {
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

pub fn l_alg_has_kl_property(limpl: &Vec<Vec<usize>>, unit: usize) -> Result<bool,String> {
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


pub fn l_alg_is_commutative_l_algebra(limpl: &Vec<Vec<usize>>) -> Result<bool,String> {
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

pub fn l_alg_is_cl_algebra(limpl: &Vec<Vec<usize>>) -> Result<bool,String> {
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

pub fn l_alg_is_left_distributive(limpl: &Vec<Vec<usize>>) -> Result<bool,String> {
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

pub fn l_alg_is_right_distributive(limpl: &Vec<Vec<usize>>) -> Result<bool,String> {
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

pub fn l_alg_is_filter(limpl: &Vec<Vec<usize>>, unit: usize, subset: &HashSet<usize>, bprint: bool) -> bool {
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

pub fn l_alg_get_all_filters(limpl: &Vec<Vec<usize>>, unit: usize) {
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

pub fn gen_all_lalgs_rec(index:usize, positions:&Vec<(usize,usize)>, limpl: &mut Vec<Vec<usize>>, unit:usize, res:&mut HashSet<Vec<Vec<usize>>>, num_tested: &mut usize) {
    let n = positions.len();
    //eprintln!("FHFH: {index} / {n}");
    *num_tested+=1;
    if *num_tested % 10_000_000 == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }
    if index >= n {
        //eprintln!("FHFH: {index} / {n}");
        //eprintln!("{limpl:?}");
        // *num_tested+=1;
        // if true || *num_tested % 1000 == 1 {
        //     eprintln!("Cur_progress: {limpl:?}");
        //     // eprintln!("{:?}", l_alg_get_repr(limpl, true));
        // }
        if l_alg_is_l_algebra(limpl, unit, false) {
            // *num_tested+=1;
            // if *num_tested % 1000 == 1 {
            //     eprintln!("Cur_progress: {limpl:?}");
            //     // eprintln!("{:?}", l_alg_get_repr(limpl, true));
            // }

            if l_alg_is_repr(limpl, true) {
                println!("{limpl:?}");
                // std::io::stdout().flush().unwrap();
                res.insert(limpl.clone());
            }
            // if res.len() % 1000 == 0 {
            //     eprintln!("{limpl:?}");
            // }
            
            // let ll = l_alg_get_repr(&limpl, true);
            // if !res.contains(&ll) {
            //     // eprintln!("{ll:?}");
            //     res.insert(ll);//limpl.clone());
            // }
        }

    }
    else {
        let x = positions[index].0;
        let y = positions[index].1;
        let m = limpl.len();
        // let o_v = limpl[x][y];
        for e in 0.. m {
            if e == unit {
                continue;
            }
            if limpl[y][x] == unit && limpl[y][e] != unit {
                continue;
            }
            let mut b_found = false;
            for t in 0..y {
                if limpl[t][y] == unit && limpl[limpl[x][t]][e] != unit {
                    b_found = true;
                    break;
                }
            }
            if b_found {
                continue;
            }
            limpl[x][y] = e;

            // test ax4 partial
            let mut b_problem = false;
            for i in 0..m {
                if b_problem {
                    break;
                }
                for j in 0..m {
                    if b_problem {
                        break;
                    }
                    if limpl[i][j] == m+1 {
                        continue;
                    }
                    if limpl[j][i] == m+1 {
                        continue;
                    }
                    for k in 0..m {
                        if limpl[i][k] == m+1 {
                            continue;
                        }
                        if limpl[j][k] == m+1 {
                            continue;
                        }
                        if limpl[limpl[i][j]][limpl[i][k]] != m+1 && limpl[limpl[j][i]][limpl[j][k]] != m+1 &&  limpl[limpl[i][j]][limpl[i][k]] != limpl[limpl[j][i]][limpl[j][k]] {
                            b_problem = true;
                            break;
                        }
                    }
                }
            }

            if b_problem {
                limpl[x][y] = m+1;
                continue;
            }
            
            gen_all_lalgs_rec(index+1, positions, limpl, unit, res, num_tested);
        }
        limpl[x][y] = m+1; //unfilled element
    }
}

pub fn l_alg_isomorphic_image(limpl: &Vec<Vec<usize>>, unit: usize, perm:&Vec<usize>) -> (Vec<Vec<usize>>, usize) {
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


pub fn l_alg_alloc_limpl(n:usize) -> Vec<Vec<usize>> {
    let mut res = Vec::<Vec<usize>>::new();

    for i in 0..n {
        res.push(Vec::<usize>::new());
        for _ in 0..n {
            res[i].push(0);
        }
    }

    res
}

pub fn l_alg_init_limpl(limpl: &mut Vec<Vec<usize>>) {
    let n = limpl.len();

    for i in 0..n {
        limpl[i][i] = 0;
        limpl[0][i] = i;
    }
} 


pub fn l_alg_init_from_ord(limpl: &mut Vec<Vec<usize>>, order: &Vec<Vec<usize>>, unit_elem:usize, unfilled_positions: &mut Vec<(usize,usize)>) {
    let n = limpl.len();
    for i in 0.. n {
        if i != unit_elem {
            for j in 0.. n {
                if order[i][j] == 1 {
                    limpl[i][j] = unit_elem;
                }
                else {
                    unfilled_positions.push((i,j));
                    limpl[i][j] = n+1;
                }
            }
        }
        else {
            for j in 0.. n {
                limpl[i][j] = j;
            }
        }
    }
}
pub fn qord_is_antisymmetric(qord: &Vec<Vec<usize>>) -> bool {
    let n = qord.len();

    for i in 0..n {
        for j in 0..n {
            if qord[i][j] == 1 && qord[j][i] == 1 && i != j {
                return false;
            }
        }
    }

    true
}

pub fn l_alg_cmp_is_strictly_less(limpl1: &Vec<Vec<usize>>, limpl2: &Vec<Vec<usize>>) -> bool {
    let n = limpl1.len();
    let mut i = 0usize;
    let mut j = 0usize;

    while i < n && limpl1[i][j] == limpl2[i][j] {
        if j < n-1 {
            j += 1;
        }
        else {
            i += 1;
            j =  0;
        }
    }

    if i == n {
        return false;
    }

    if limpl1[i][j] < limpl2[i][j] {
        return true;
    }
    
    false
}

pub fn l_alg_cmp_is_strictly_greater(limpl1: &Vec<Vec<usize>>, limpl2: &Vec<Vec<usize>>) -> bool {
    let n = limpl1.len();
    let mut i = 0usize;
    let mut j = 0usize;

    while i < n && limpl1[i][j] == limpl2[i][j] {
        if j < n-1 {
            j += 1;
        }
        else {
            i += 1;
            j =  0;
        }
    }

    if i == n {
        return false;
    }

    if limpl1[i][j] > limpl2[i][j] {
        return true;
    }
    
    false
}



pub fn l_alg_get_repr(limpl: &Vec<Vec<usize>>, b_minimal: bool) ->Vec<Vec<usize>> {
    let n = limpl.len();
    let lalg_unit = limpl[0][0];

    let mut base_perm_vec = Vec::<usize>::new();
    let mut iso_perm_vec= Vec::<usize>::new();
    //let mut limpl_repr = Vec::<Vec<usize>>::new();

    let mut limpl_repr = limpl.clone();
    for i in 0..n {
        if i != lalg_unit {
            base_perm_vec.push(i);
        }
        iso_perm_vec.push(i);
    }

    for perm in base_perm_vec.iter().permutations(n-1) {
        for j in 0..n-1 {
            iso_perm_vec[base_perm_vec[j]] = *perm[j];
        }
        
        // let mut b_preserve = true;
        // for idx1 in 0..n {
        //     for idx2 in (idx1+1)..n {
        //         if idx1 < idx2 {
        //         if limpl[idx1][idx2] == lalg_unit && iso_perm_vec[idx1] > iso_perm_vec[idx2]{
        //             b_preserve = false;
        //             break;
        //         }
        //         }
        //     }
        //     if !b_preserve {
        //         break;
        //     }
        // }
        // if !b_preserve {
        //     continue;
        // }

        if !l_alg_perm_preserve_ord(limpl, &iso_perm_vec) {
            continue;
        }

        // eprintln!("{perm:?}");
        // eprintln!("{iso_perm_vec:?}");
        let limpl_img = l_alg_isomorphic_image(limpl, lalg_unit, &iso_perm_vec).0;

        if b_minimal {
            if l_alg_cmp_is_strictly_less(&limpl_img, &limpl_repr) {
                limpl_repr = limpl_img;
            }
        }
        else {
            if l_alg_cmp_is_strictly_greater(&limpl_img, &limpl_repr) {
                limpl_repr = limpl_img;
            }
        }
    }

    limpl_repr
}

// canonical preserve e_i < e_j iff i < j
pub fn l_alg_perm_preserve_ord(limpl: &Vec<Vec<usize>>, iso_perm_vec: &Vec<usize>) -> bool {
        let n = limpl.len();
        let lalg_unit = limpl[0][0];

        for idx1 in 0..n {
            for idx2 in (idx1+1)..n {                
                if limpl[idx1][idx2] == lalg_unit && iso_perm_vec[idx1] > iso_perm_vec[idx2]{
                    return false
                }
            }
        }
        true
}

pub fn l_alg_is_repr(limpl: &Vec<Vec<usize>>, b_minimal: bool) -> bool {
    let n = limpl.len();
    let lalg_unit = limpl[0][0];

    let mut base_perm_vec = Vec::<usize>::new();
    let mut iso_perm_vec= Vec::<usize>::new();
    // let mut limpl_repr = Vec::<Vec<usize>>::new();

    // limpl_repr = limpl.clone();
    for i in 0..n {
        if i != lalg_unit {
            base_perm_vec.push(i);
        }
        iso_perm_vec.push(i);
    }

    for perm in base_perm_vec.iter().permutations(n-1) {
        for j in 0..n-1 {
            iso_perm_vec[base_perm_vec[j]] = *perm[j];
        }
        
    //    let mut b_preserve = true;
    //    for idx1 in 0..n {
    //        for idx2 in 0..n {
    //            if idx1 < idx2 {
    //             //    if limpl[iso_perm_vec[idx1]][iso_perm_vec[idx2]] == lalg_unit && iso_perm_vec[idx1] > iso_perm_vec[idx2]{
    //                 if limpl[idx1][idx2] == lalg_unit && iso_perm_vec[idx1] > iso_perm_vec[idx2]{
    //                    b_preserve = false;
    //                    break;
    //                }
    //            }
    //        }
    //        if !b_preserve {
    //            break;
    //        }
    //    }
    //    if !b_preserve {
    //        continue;
    //    }
        if !l_alg_perm_preserve_ord(limpl, &iso_perm_vec) {
            continue;
        }
        
        // eprintln!("{perm:?}");
        // eprintln!("{iso_perm_vec:?}");
        let limpl_img = l_alg_isomorphic_image(limpl, lalg_unit, &iso_perm_vec).0;

        if b_minimal {
            if l_alg_cmp_is_strictly_less(&limpl_img, &limpl) {
                return false;
            }
        }
        else {
            if l_alg_cmp_is_strictly_greater(&limpl_img, &limpl) {
                return false;
            }
        }
    }

    true
}

