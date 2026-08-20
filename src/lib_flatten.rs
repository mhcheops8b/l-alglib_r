use std::collections::{HashSet, HashMap};
use std::time::{Instant, Duration};
use itertools::Itertools;

// Attribution: 
// most code suggested by Gemini.AI - 3.6 Flash (free)
// after rewrite request of src/lib.rs to use flatten 1d arrays
//

// 1D flat indexing helper
#[inline(always)]
fn idx(x: usize, y: usize, n: usize) -> usize {
    x * n + y
}

// Flattened functions
pub fn l_alg_ax1(limpl: &[usize], n: usize, unit: usize) -> bool {
    for x in 0..n {
        if limpl[idx(x, x, n)] != unit {
            return false;
        }
    }
    true
}

pub fn l_alg_ax2(limpl: &[usize], n: usize, unit: usize) -> bool {
    for x in 0..n {
        if limpl[idx(x, unit, n)] != unit {
            return false;
        }
    }
    true
}

pub fn l_alg_ax3(limpl: &[usize], n: usize, unit: usize) -> bool {
    for x in 0..n {
        if limpl[idx(unit, x, n)] != x {
            return false;
        }
    }
    true
}

pub fn l_alg_ax4(limpl: &[usize], n: usize, bprint: bool) -> bool {
    for x in 0..n {
        for y in 0..n {
            let v_xy = limpl[idx(x, y, n)];
            let v_yx = limpl[idx(y, x, n)];
            for z in 0..n {
                let left = limpl[idx(v_xy, limpl[idx(x, z, n)], n)];
                let right = limpl[idx(v_yx, limpl[idx(y, z, n)], n)];
                if left != right {
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

pub fn l_alg_ax5(limpl: &[usize], n: usize, unit: usize) -> bool {
    for x in 0..n {
        for y in 0..n {
            if x != y && limpl[idx(x, y, n)] == unit && limpl[idx(y, x, n)] == unit {
                return false;
            }
        }
    }
    true
}

pub fn l_alg_is_l_algebra(limpl: &[usize], n: usize, unit: usize, bprint: bool) -> bool {
    l_alg_ax1(limpl, n, unit) &&
    l_alg_ax2(limpl, n, unit) &&
    l_alg_ax3(limpl, n, unit) &&
    l_alg_ax4(limpl, n, bprint) &&
    l_alg_ax5(limpl, n, unit)
}

pub fn l_alg_has_kl_property_old(limpl: &[usize], n: usize, unit: usize) -> bool {
    for x in 0..n {
        for a in 0..n {
            if limpl[idx(x, limpl[idx(a, x, n)], n)] != unit {
                eprintln!("Problem: x = {}, a = {}", x, a);
                return false;
            }
        }
    }
    true
}

pub fn l_alg_has_kl_property(limpl: &[usize], n: usize, unit: usize) -> Result<bool, String> {
    for x in 0..n {
        for a in 0..n {
            if limpl[idx(x, limpl[idx(a, x, n)], n)] != unit {
                return Err(format!("KL - Problem: x = {}, a = {}", x, a));
            }
        }
    }
    Ok(true)
}

pub fn l_alg_is_commutative_l_algebra(limpl: &[usize], n: usize) -> Result<bool, String> {
    for x in 0..n {
        for y in 0..n {
            let left = limpl[idx(limpl[idx(x, y, n)], y, n)];
            let right = limpl[idx(limpl[idx(y, x, n)], x, n)];
            if left != right {
                return Err(format!("Comm - Problem: x = {}, y = {}", x, y));
            }
        }
    }
    Ok(true)
}

pub fn l_alg_is_cl_algebra(limpl: &[usize], n: usize) -> Result<bool, String> {
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let left = limpl[idx(x, limpl[idx(y, z, n)], n)];
                let right = limpl[idx(y, limpl[idx(x, z, n)], n)];
                if left != right {
                    return Err(format!("CL - Problem: x = {}, y = {}, z = {}", x, y, z));
                }
            }
        }
    }
    Ok(true)
}

pub fn l_alg_is_left_distributive(limpl: &[usize], n: usize) -> Result<bool, String> {
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let left = limpl[idx(x, limpl[idx(y, z, n)], n)];
                let right = limpl[idx(limpl[idx(x, y, n)], limpl[idx(x, z, n)], n)];
                if left != right {
                    return Err(format!("Left Dist. - Problem: x = {}, y = {}, z = {}", x, y, z));
                }
            }
        }
    }
    Ok(true)
}

pub fn l_alg_is_right_distributive(limpl: &[usize], n: usize) -> Result<bool, String> {
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let left = limpl[idx(limpl[idx(x, y, n)], z, n)];
                let right = limpl[idx(limpl[idx(x, z, n)], limpl[idx(y, z, n)], n)];
                if left != right {
                    return Err(format!("Right Dist. - Problem: x = {}, y = {}, z = {}", x, y, z));
                }
            }
        }
    }
    Ok(true)
}

pub fn l_alg_is_filter(limpl: &[usize], n: usize, unit: usize, subset: &HashSet<usize>, bprint: bool) -> bool {
    if !subset.contains(&unit) {
        return false;
    }

    for x in subset {
        for y in 0..n {
            if subset.contains(&limpl[idx(*x, y, n)]) && !subset.contains(&y) {
                if bprint {
                    eprintln!("Problem: x = {}, y = {}", *x, y);
                }
                return false;
            }
        }
    }
    true
}

pub fn l_alg_get_order(limpl: &[usize], n: usize) -> Vec<usize> {
    let mut res = vec![0usize; n * n];
    for i in 0..n {
        for j in 0..n {
            if limpl[idx(i, j, n)] == n - 1 {
                res[idx(i, j, n)] = 1;
            }
        }
    }
    res
}

pub fn l_alg_get_all_filters(limpl: &[usize], n: usize, unit: usize) {
    for i in 0usize..(1 << n) {
        if i & (1 << unit) == 0 {
            continue;
        }
        let mut filt_cand = HashSet::<usize>::new();
        for k in 0..n {
            if i & (1 << k) != 0 {
                filt_cand.insert(k);
            }
        }

        if l_alg_is_filter(limpl, n, unit, &filt_cand, false) {
            println!(" {:?}", filt_cand);
        }
    }
}

pub fn l_alg_test_ax4_partial_xy(limpl: &[usize], m: usize, x: usize, y: usize, _b_print: bool) -> bool {
    if limpl[idx(y, x, m)] != m + 1 {
        for z in 0..m - 1 {
            if x != z && y != z
                && limpl[idx(x, z, m)] != m + 1
                && limpl[idx(y, z, m)] != m + 1
                && limpl[idx(limpl[idx(x, y, m)], limpl[idx(x, z, m)], m)] != m + 1
                && limpl[idx(limpl[idx(y, x, m)], limpl[idx(y, z, m)], m)] != m + 1
                && limpl[idx(limpl[idx(x, y, m)], limpl[idx(x, z, m)], m)]
                    != limpl[idx(limpl[idx(y, x, m)], limpl[idx(y, z, m)], m)]
            {
                return false;
            }
        }
    }
    for z in 0..m - 1 {
        if z == x || z == y || limpl[idx(x, z, m)] == m + 1 || limpl[idx(z, x, m)] == m + 1 || limpl[idx(z, y, m)] == m + 1 {
            continue;
        }

        if limpl[idx(limpl[idx(x, z, m)], limpl[idx(x, y, m)], m)] != m + 1
            && limpl[idx(limpl[idx(z, x, m)], limpl[idx(z, y, m)], m)] != m + 1
            && limpl[idx(limpl[idx(x, z, m)], limpl[idx(x, y, m)], m)]
                != limpl[idx(limpl[idx(z, x, m)], limpl[idx(z, y, m)], m)]
        {
            return false;
        }
    }
    for s in 0..m - 1 {
        for t in 0..m - 1 {
            if limpl[idx(s, t, m)] == x {
                for u in 0..m - 1 {
                    if limpl[idx(s, u, m)] == y {
                        if limpl[idx(t, s, m)] != m + 1
                            && limpl[idx(t, u, m)] != m + 1
                            && limpl[idx(limpl[idx(t, s, m)], limpl[idx(t, u, m)], m)] != m + 1
                            && limpl[idx(limpl[idx(t, s, m)], limpl[idx(t, u, m)], m)] != limpl[idx(x, y, m)]
                        {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

// unit = m-1
pub fn l_alg_test_ax4_partial(limpl: &[usize], m: usize, b_print: bool) -> bool {
    for i in 0..m - 1 {
        for j in 0..m - 1 {
            if i == j || limpl[idx(i, j, m)] == m + 1 || limpl[idx(j, i, m)] == m + 1 {
                continue;
            }
            for k in 0..m - 1 {
                if i == k || j == k || limpl[idx(i, k, m)] == m + 1 || limpl[idx(j, k, m)] == m + 1 {
                    continue;
                }
                let left = limpl[idx(limpl[idx(i, j, m)], limpl[idx(i, k, m)], m)];
                let right = limpl[idx(limpl[idx(j, i, m)], limpl[idx(j, k, m)], m)];
                if left != m + 1 && right != m + 1 && left != right {
                    if b_print {
                        eprintln!("Partial ax4 is not satisfied for x = {}, y = {}, z = {}", i, j, k);
                    }
                    return false;
                }
            }
        }
    }
    true
}

pub fn l_alg_test_ax4_partial_as_result(limpl: &[usize], m: usize) -> Result<bool, String> {
    for i in 0..m {
        for j in 0..m {
            if limpl[idx(i, j, m)] == m + 1 || limpl[idx(j, i, m)] == m + 1 {
                continue;
            }
            for k in 0..m {
                if limpl[idx(i, k, m)] == m + 1 || limpl[idx(j, k, m)] == m + 1 {
                    continue;
                }
                let left = limpl[idx(limpl[idx(i, j, m)], limpl[idx(i, k, m)], m)];
                let right = limpl[idx(limpl[idx(j, i, m)], limpl[idx(j, k, m)], m)];
                if left != m + 1 && right != m + 1 && left != right {
                    return Err(format!("Partial ax4 is not satisfied for x = {}, y = {}, z = {}", i, j, k));
                }
            }
        }
    }
    Ok(true)
}

pub fn l_alg_alloc_limpl(n: usize) -> Vec<usize> {
    vec![0; n * n]
}

pub fn l_alg_init_limpl(limpl: &mut [usize], n: usize) {
    for i in 0..n {
        limpl[idx(i, i, n)] = 0;
        limpl[idx(0, i, n)] = i;
    }
}

pub fn l_alg_init_from_ord(limpl: &mut [usize], n: usize, order: &[usize], unit_elem: usize) {
    for i in 0..n {
        if i != unit_elem {
            for j in 0..n {
                if order[idx(i, j, n)] == 1 {
                    limpl[idx(i, j, n)] = unit_elem;
                } else {
                    limpl[idx(i, j, n)] = n + 1;
                }
            }
        } else {
            for j in 0..n {
                limpl[idx(i, j, n)] = j;
            }
        }
    }
}

pub fn l_alg_isomorphic_image(limpl: &[usize], n: usize, unit: usize, perm: &[usize]) -> (Vec<usize>, usize) {
    let mut res = vec![0usize; n * n];
    let res_unit = perm[unit];

    for i in 0..n {
        for j in 0..n {
            res[idx(perm[i], perm[j], n)] = perm[limpl[idx(i, j, n)]];
        }
    }

    (res, res_unit)
}

pub fn qord_is_antisymmetric(qord: &[usize], n: usize) -> bool {
    for i in 0..n {
        for j in 0..n {
            if qord[idx(i, j, n)] == 1 && qord[idx(j, i, n)] == 1 && i != j {
                return false;
            }
        }
    }
    true
}

pub fn l_alg_cmp_is_strictly_less(limpl1: &[usize], limpl2: &[usize]) -> bool {
    for (val1, val2) in limpl1.iter().zip(limpl2.iter()) {
        if val1 != val2 {
            return val1 < val2;
        }
    }
    false
}

pub fn l_alg_cmp_is_strictly_greater(limpl1: &[usize], limpl2: &[usize]) -> bool {
    for (val1, val2) in limpl1.iter().zip(limpl2.iter()) {
        if val1 != val2 {
            return val1 > val2;
        }
    }
    false
}

pub fn pord_is_canonical(pord: &[usize], n: usize) -> bool {
    for idx1 in 0..n {
        for idx2 in 0..n {
            if idx1 != idx2 && pord[idx(idx1, idx2, n)] == 1 && idx1 > idx2 {
                return false;
            }
        }
    }
    true
}

pub fn pord_perm_canonical_preserve_ord(pord: &[usize], n: usize, iso_perm_vec: &[usize]) -> bool {
    for idx1 in 0..n {
        for idx2 in (idx1 + 1)..n {
            if pord[idx(idx1, idx2, n)] == 1 && iso_perm_vec[idx1] > iso_perm_vec[idx2] {
                return false;
            }
        }
    }
    true
}

pub fn pord_perm_preserve_ord(pord: &[usize], n: usize, iso_perm_vec: &[usize]) -> bool {
    for idx1 in 0..n {
        for idx2 in 0..n {
            if idx1 != idx2 && pord[idx(idx1, idx2, n)] == 1 && pord[idx(iso_perm_vec[idx1], iso_perm_vec[idx2], n)] == 0 {
                return false;
            }
        }
    }
    true
}

pub fn l_alg_perm_preserve_ord(limpl: &[usize], n: usize, iso_perm_vec: &[usize]) -> bool {
    let lalg_unit = limpl[idx(0, 0, n)];
    for idx1 in 0..n {
        for idx2 in (idx1 + 1)..n {
            if limpl[idx(idx1, idx2, n)] == lalg_unit && iso_perm_vec[idx1] > iso_perm_vec[idx2] {
                return false;
            }
        }
    }
    true
}

pub fn l_alg_perm_preserve_original_ord(limpl: &[usize], n: usize, iso_perm_vec: &[usize]) -> bool {
    let lalg_unit = limpl[idx(0, 0, n)];
    for idx1 in 0..n {
        if idx1 == lalg_unit { continue; }
        for idx2 in 0..n {
            if idx2 == lalg_unit || idx1 == idx2 { continue; }
            if limpl[idx(idx1, idx2, n)] == lalg_unit && limpl[idx(iso_perm_vec[idx1], iso_perm_vec[idx2], n)] != lalg_unit {
                return false;
            }
        }
    }
    true
}

pub fn l_alg_get_repr(limpl: &[usize], n: usize, b_minimal: bool, b_canonical: bool) -> Vec<usize> {
    let lalg_unit = limpl[idx(0, 0, n)];
    let mut base_perm_vec = Vec::<usize>::new();
    let mut iso_perm_vec = Vec::<usize>::new();

    let mut limpl_repr = limpl.to_owned();
    for i in 0..n {
        if i != lalg_unit {
            base_perm_vec.push(i);
        }
        iso_perm_vec.push(i);
    }

    for perm in base_perm_vec.iter().permutations(n - 1) {
        for j in 0..n - 1 {
            iso_perm_vec[base_perm_vec[j]] = *perm[j];
        }

        if b_canonical && !l_alg_perm_preserve_ord(limpl, n, &iso_perm_vec) {
            continue;
        }

        let limpl_img = l_alg_isomorphic_image(limpl, n, lalg_unit, &iso_perm_vec).0;

        if b_minimal {
            if l_alg_cmp_is_strictly_less(&limpl_img, &limpl_repr) {
                limpl_repr = limpl_img;
            }
        } else if l_alg_cmp_is_strictly_greater(&limpl_img, &limpl_repr) {
            limpl_repr = limpl_img;
        }
    }

    limpl_repr
}

pub fn l_alg_get_repr_with_orig_ord(
    limpl: &[usize], 
    n: usize, 
    b_minimal: bool, 
    b_canonical: bool
) -> Vec<usize> {
    let lalg_unit = limpl[idx(0, 0, n)];
    let mut base_perm_vec = Vec::<usize>::new();
    let mut iso_perm_vec = Vec::<usize>::new();

    let mut limpl_repr = limpl.to_owned();

    for i in 0..n {
        if i != lalg_unit {
            base_perm_vec.push(i);
        }
        iso_perm_vec.push(i);
    }

    for perm in base_perm_vec.iter().permutations(n - 1) {
        for j in 0..n - 1 {
            iso_perm_vec[base_perm_vec[j]] = *perm[j];
        }

        if b_canonical && !l_alg_perm_preserve_original_ord(limpl, n, &iso_perm_vec) {
            continue;
        }

        let limpl_img = l_alg_isomorphic_image(limpl, n, lalg_unit, &iso_perm_vec).0;

        if b_minimal {
            if l_alg_cmp_is_strictly_less(&limpl_img, &limpl_repr) {
                limpl_repr = limpl_img;
            }
        } else if l_alg_cmp_is_strictly_greater(&limpl_img, &limpl_repr) {
            limpl_repr = limpl_img;
        }
    }

    limpl_repr
}

pub fn l_alg_get_repr_with_target_ord(
    limpl: &[usize],
    target_ord: &[usize],
    n: usize,
    b_minimal: bool,
    b_canonical: bool,
) -> Vec<usize> {
    let lalg_unit = limpl[idx(0, 0, n)];
    let mut base_perm_vec = Vec::<usize>::new();
    let mut iso_perm_vec = Vec::<usize>::new();

    let mut limpl_repr = limpl.to_owned();

    for i in 0..n {
        if i != lalg_unit {
            base_perm_vec.push(i);
        }
        iso_perm_vec.push(i);
    }

    for perm in base_perm_vec.iter().permutations(n - 1) {
        for j in 0..n - 1 {
            iso_perm_vec[base_perm_vec[j]] = *perm[j];
        }

        if b_canonical && !pord_perm_preserve_ord(target_ord, n, &iso_perm_vec) {
            continue;
        }

        let limpl_img = l_alg_isomorphic_image(limpl, n, lalg_unit, &iso_perm_vec).0;

        if b_minimal {
            if l_alg_cmp_is_strictly_less(&limpl_img, &limpl_repr) {
                limpl_repr = limpl_img;
            }
        } else if l_alg_cmp_is_strictly_greater(&limpl_img, &limpl_repr) {
            limpl_repr = limpl_img;
        }
    }

    limpl_repr
}

pub fn l_alg_is_repr(limpl: &[usize], n: usize, b_minimal: bool) -> bool {
    let lalg_unit = limpl[idx(0, 0, n)];
    let mut base_perm_vec = Vec::<usize>::new();
    let mut iso_perm_vec = Vec::<usize>::new();

    for i in 0..n {
        if i != lalg_unit {
            base_perm_vec.push(i);
        }
        iso_perm_vec.push(i);
    }

    for perm in base_perm_vec.iter().permutations(n - 1) {
        for j in 0..n - 1 {
            iso_perm_vec[base_perm_vec[j]] = *perm[j];
        }

        if !l_alg_perm_preserve_ord(limpl, n, &iso_perm_vec) {
            continue;
        }

        let limpl_img = l_alg_isomorphic_image(limpl, n, lalg_unit, &iso_perm_vec).0;

        if b_minimal {
            if l_alg_cmp_is_strictly_less(&limpl_img, limpl) {
                return false;
            }
        } else if l_alg_cmp_is_strictly_greater(&limpl_img, limpl) {
            return false;
        }
    }

    true
}

pub fn l_alg_test_init_vector(
    pord: &[usize],
    init_vector: &[usize],
    n: usize,
    unit: usize,
    b_print: bool,
) -> bool {
    let mut limpl = l_alg_alloc_limpl(n);
    let mut positions = Vec::<(usize, usize)>::new();

    l_alg_init_from_ord(&mut limpl, n, pord, unit);
    l_alg_init_get_positions_new(pord, &mut positions, n);

    let min_len = std::cmp::min(positions.len(), init_vector.len());

    for i in 0..min_len {
        let (x, y) = positions[i];
        let e = init_vector[i];

        if e == n + 1 {
            continue;
        }

        if e == unit {
            if b_print {
                eprintln!("(Element at ({}, {}) cannot be equal to unit {}.)", x, y, unit);
            }
            return false;
        }

        // Verify partial order bounds and self-consistency conditions
        for t in 0..y {
            let impl_t_y = limpl[idx(t, y, n)];
            let impl_x_t = limpl[idx(x, t, n)];

            if impl_t_y == unit && impl_x_t != n + 1 && limpl[idx(impl_x_t, e, n)] != unit {
                if b_print {
                    eprintln!(
                        "(Element e={} at (x={}, y={}) needs to be larger than {} since t={} <= y => x->t <= x->y.)",
                        e, x, y, impl_x_t, t
                    );
                }
                return false;
            }
        }

        limpl[idx(x, y, n)] = e;

        if !l_alg_test_ax4_partial(&limpl, n, false) {
            if b_print {
                eprintln!("Partial ax4 is not satisfied for assignment ({}, {}) = {}", x, y, e);
            }
            return false;
        }
    }

    true
}

pub fn l_alg_test_init_vector_with_positions(
    pord: &[usize],
    init_vector: &[usize],
    positions: &[(usize, usize)],
    n: usize,
    unit: usize,
    b_print: bool,
) -> bool {
    let mut limpl = l_alg_alloc_limpl(n);

    l_alg_init_from_ord(&mut limpl, n, pord, unit);

    let min_len = std::cmp::min(positions.len(), init_vector.len());

    for i in 0..min_len {
        let (x, y) = positions[i];
        let e = init_vector[i];

        if e == n + 1 {
            continue;
        }

        if e == unit {
            if b_print {
                eprintln!("(Element at ({}, {}) cannot be equal to unit {}.)", x, y, unit);
            }
            return false;
        }

        // Check compatibility conditions using 1D indexing
        for t in 0..y {
            let impl_t_y = limpl[idx(t, y, n)];
            let impl_x_t = limpl[idx(x, t, n)];

            if impl_t_y == unit && impl_x_t != n + 1 && limpl[idx(impl_x_t, e, n)] != unit {
                if b_print {
                    eprintln!(
                        "(Element e={} at (x={}, y={}) needs to be larger than {} since t={} <= y => x->t <= x->y.)",
                        e, x, y, impl_x_t, t
                    );
                }
                return false;
            }
        }

        limpl[idx(x, y, n)] = e;

        if !l_alg_test_ax4_partial(&limpl, n, false) {
            if b_print {
                eprintln!("Partial ax4 is not satisfied for assignment ({}, {}) = {}", x, y, e);
            }
            return false;
        }
    }

    true
}

pub fn l_alg_test_init_value(
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    x: usize,
    y: usize,
    e: usize,
    b_print: bool,
) -> bool {
    if e == n + 1 {
        return true;
    }

    if e == unit {
        if b_print {
            eprintln!("(Element at ({}, {}) cannot be equal to unit {}.)", x, y, unit);
        }
        return false;
    }

    // Check compatibility conditions using 1D indexing
    for t in 0..y {
        let impl_t_y = limpl[idx(t, y, n)];
        let impl_x_t = limpl[idx(x, t, n)];

        if impl_t_y == unit && impl_x_t != n + 1 && limpl[idx(impl_x_t, e, n)] != unit {
            if b_print {
                eprintln!(
                    "(Element e={} at (x={}, y={}) needs to be larger than {} since t={} <= y => x->t <= x->y.)",
                    e, x, y, impl_x_t, t
                );
            }
            return false;
        }
    }

    let prev_val = limpl[idx(x, y, n)];
    limpl[idx(x, y, n)] = e;

    if !l_alg_test_ax4_partial(limpl, n, false) {
        if b_print {
            eprintln!("Partial ax4 is not satisfied for assignment ({}, {}) = {}", x, y, e);
        }
        limpl[idx(x, y, n)] = prev_val; // Revert change on failure
        return false;
    }

    true
}

pub fn gen_all_lalgs_rec(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
) {
    let num_positions = positions.len();
    *num_tested += 1;
    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }
    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }
        }
    } else {
        let (x, y) = positions[index];
        for e in 0..n {
            if e == unit {
                continue;
            }

            let mut b_found = false;
            for t in 0..y {
                
                if limpl[idx(t, y, n)] == unit && limpl[idx(x, t, n)] != n + 1 && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }
            if b_found {
                continue;
            }
            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            gen_all_lalgs_rec(index + 1, positions, limpl, n, unit, res, num_tested, num_models, status_output_limit);
        }
        limpl[idx(x, y, n)] = n + 1;
    }
}

pub fn gen_all_lalgs_rec_short_iter(
index: usize,
positions: &[(usize, usize)],
limpl: &mut [usize],
n: usize,
unit: usize,
res: &mut HashSet<Vec<usize>>,
num_tested: &mut usize,
num_models: &mut usize,
status_output_limit: usize,
) {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using flattened 1D array indexing
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            gen_all_lalgs_rec_short_iter(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
            );
        }

        // Reset state for backtracking
        limpl[idx(x, y, n)] = n + 1;
    }
}

const SHORT_STOP_ITER_COUNT:usize = 250_000_000;

pub fn l_alg_gen_all_short_iter(n: usize, status_output_limit: usize) -> HashSet<Vec<usize>> {
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    let mut limpl = vec![n + 1; n * n];
    let mut positions = Vec::<(usize, usize)>::new();
    let mut res = HashSet::<Vec<usize>>::new();

    let unit = 0;
    l_alg_init_limpl(&mut limpl, n);

    for x in 1..n {
        for y in 1..n {
            if x != y {
                positions.push((x, y));
            }
        }
    }

    let start_time = Instant::now();

    gen_all_lalgs_rec_short_iter(
        0,
        &positions,
        &mut limpl,
        n,
        unit,
        &mut res,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
    );

    let duration = start_time.elapsed();

    eprintln!(
        "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
        duration,
        num_tested,
        num_models,
        res.len()
    );

    res
}

pub fn gen_all_lalgs_rec_short_iter_(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
) {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using 1D indexing
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            gen_all_lalgs_rec_short_iter_(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
            );
        }

        // Reset state for backtracking
        limpl[idx(x, y, n)] = n + 1;
    }
}

pub fn l_alg_gen_all_short_iter_(n: usize, status_output_limit: usize) -> HashSet<Vec<usize>> {
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    let mut limpl = vec![n + 1; n * n];
    let mut positions = Vec::<(usize, usize)>::new();
    let mut res = HashSet::<Vec<usize>>::new();

    let unit = 0;
    l_alg_init_limpl(&mut limpl, n);

    for x in 1..n {
        for y in 1..n {
            if x != y {
                positions.push((x, y));
            }
        }
    }

    let start_time = Instant::now();

    gen_all_lalgs_rec_short_iter_(
        0,
        &positions,
        &mut limpl,
        n,
        unit,
        &mut res,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
    );

    let duration = start_time.elapsed();

    eprintln!(
        "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
        duration,
        num_tested,
        num_models,
        res.len()
    );

    res
}

pub fn gen_lalgs_from_pord_rec(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
) {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            gen_lalgs_from_pord_rec(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
            );
        }

        limpl[idx(x, y, n)] = n + 1;
    }
}

pub fn l_alg_init_get_positions_old(order: &[usize], unfilled_positions: &mut Vec<(usize,usize)>, n: usize) {
    for i in 0..n-1 {
        for j in 0..n {
            if order[idx(i, j, n)] == 0 {
                unfilled_positions.push((i,j));
            }
        }
    }
}

pub fn positions_process_pair(
    order: &[usize],
    already_processed: &mut HashSet<(usize, usize)>,
    unfilled_positions: &mut Vec<(usize, usize)>,
    x: usize,
    y: usize,
    n: usize,
) {
    if !already_processed.contains(&(x, y)) {
        already_processed.insert((x, y));
        if order[idx(x, y, n)] == 0 {
            unfilled_positions.push((x, y));
        }
    }
}

pub fn positions_process_triple(order: &[usize], already_processed: &mut HashSet<(usize,usize)>, unfilled_positions: &mut Vec<(usize,usize)>, i: usize, j:usize, k:usize, n:usize) {
    positions_process_pair(order, already_processed, unfilled_positions, i, j, n);
    positions_process_pair(order, already_processed, unfilled_positions, i, k, n);
    positions_process_pair(order, already_processed, unfilled_positions, j, i, n);
    positions_process_pair(order, already_processed, unfilled_positions, j, k, n);
    positions_process_pair(order, already_processed, unfilled_positions, k, i, n);
    positions_process_pair(order, already_processed, unfilled_positions, k, j, n);
}


pub fn l_alg_init_get_positions_new(
    order: &[usize],
    unfilled_positions: &mut Vec<(usize, usize)>,
    n: usize,
) {
    let mut already_processed = HashSet::<(usize, usize)>::new();

    for i in 1..n - 1 {
        for j in 0..i {
            positions_process_pair(order, &mut already_processed, unfilled_positions, j, i, n);
        }
        for j in 0..i {
            positions_process_pair(order, &mut already_processed, unfilled_positions, i, j, n);
        }
    }
}

pub fn l_alg_init_from_ord_(limpl: &mut [usize], n: usize, order: &[usize], unit_elem: usize) {
    for i in 0..n {
        if i != unit_elem {
            for j in 0..n {
                if order[idx(i, j, n)] == 1 {
                    limpl[idx(i, j, n)] = unit_elem;
                } else {
                    limpl[idx(i, j, n)] = n + 1;
                }
            }
        } else {
            for j in 0..n {
                limpl[idx(i, j, n)] = j;
            }
        }
    }
}

pub fn gen_all_lalgs_from_ord_rec(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
) {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using flattened 1D array indexing
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            gen_all_lalgs_from_ord_rec(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
            );
        }

        // Backtracking: reset element state
        limpl[idx(x, y, n)] = n + 1;
    }
}

// pub fn l_alg_gen_from_ord(
//     pord: &[usize],
//     n: usize,
//     unit_elem: usize,
//     status_output_limit: usize,
// ) -> HashSet<Vec<usize>> {
//     let mut num_tested = 0usize;
//     let mut num_models = 0usize;
//     let mut limpl = vec![n + 1; n * n];
//     let mut positions = Vec::<(usize, usize)>::new();
//     let mut res = HashSet::<Vec<usize>>::new();

//     // Initialize implication table from the input order
//     l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);
//     l_alg_init_get_positions_new(pord, &mut positions, n);

//     let start_time = Instant::now();

//     gen_all_lalgs_from_ord_rec(
//         0,
//         &positions,
//         &mut limpl,
//         n,
//         unit_elem,
//         &mut res,
//         &mut num_tested,
//         &mut num_models,
//         status_output_limit,
//     );

//     let duration = start_time.elapsed();

//     eprintln!(
//         "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
//         duration,
//         num_tested,
//         num_models,
//         res.len()
//     );

//     res
// }

// manual transformation
pub fn l_alg_gen_from_ord(pord: &[usize], n: usize, init_vector: &Vec<usize>, lalgs: &mut HashSet<Vec<usize>>, b_test: bool, b_print: bool, status_output_limit: usize) {

    if b_print {
        eprintln!("Order: {pord:?}");
    }

    let mut lalg_limpl = l_alg_alloc_limpl(n);
    let mut positions = Vec::<(usize,usize)>::new();
                
    l_alg_init_from_ord(&mut lalg_limpl, n, &pord, n-1);
    l_alg_init_get_positions_old(pord, &mut positions, n);

    // apply init_vector
    let mut b_first = true;
    for i in 0usize..std::cmp::min(positions.len(), init_vector.len()) {
        if b_print {
            if b_first {
                b_first = false;
            } 
            else {
                eprint!(", ");
            }
        }
        let (x, y) = positions[i];
        let e = init_vector[i];
        if b_print {
            eprint!("({},{}) = {} ", x, y, e);
        }
        if b_test {
            if e == n+1 {
                if b_print {
                    eprint!("(skipping)");
                }
                continue;
            }
            if e == n-1 {
                if b_print {
                    eprint!("(Element at ({}, {}) cannot be equal to unit ({}).)",x,y,n-1);
                }
                return;
            }

            // self-similar property: x -> (y -> x) = y -> (x -> y)    
            // if lalg_limpl[y][x] == n-1 && lalg_limpl[y][e] != n-1 {
            //     if b_print {
            //         eprint!("(Element at ({}, {}) needs to be greater than {} since {} <= {}.)",x,y,y,y,x);
            //     }
            //     return;
            // }

            for t in 0..y {
                if lalg_limpl[idx(t, y, n)] == n-1 
                   && lalg_limpl[idx(x,t,n)] != n+1 
                   && lalg_limpl[idx(lalg_limpl[idx(x,t,n)],e, n)] != n-1 {
                    if b_print {
                        eprint!("(Element e={} at (x={}, y={}) needs to larger than {} since t={} <= y => x->t <= x->y.)", e, x, y, lalg_limpl[idx(x,t,n)], t);
                    }
                    return;
                }
            }
        }

        lalg_limpl[idx(x,y,n)] = e;
        if b_test {
            if !l_alg_test_ax4_partial(&lalg_limpl, n,  true) {
                //eprintln!("Partial ax4 is not satisfied");
                return;
            }
        }
    }
    eprintln!();    
    for i in (0usize..std::cmp::min(positions.len(), init_vector.len())).rev() {
        if init_vector[i] != n+1 {
            positions.remove(i); 
        }
    }
    if b_print {
        eprintln!("Positions: {positions:?}");
        eprintln!("Init limpl: {lalg_limpl:?}");
    }
        // return;
    let time_start = Instant::now();
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    gen_all_lalgs_rec(0, &positions, &mut lalg_limpl, n, n-1, lalgs, &mut num_tested, &mut num_models, status_output_limit);

    eprintln!("Computation time: {:.4} s", time_start.elapsed().as_secs_f32());
    eprintln!("Number recursive calls: {}", num_tested);
    eprintln!("Number of all models: {}", num_models);
    eprintln!("Number of representative models {}", lalgs.len());
}

pub fn l_alg_gen_from_ord_new(
    pord: &[usize],
    init_vector: &[usize],
    lalgs: &mut HashSet<Vec<usize>>,
    b_test: bool,
    b_print: bool,
    status_output_limit: usize,
    n: usize,
) {
    if b_print {
        eprintln!("Order: {pord:?}");
    }

    let mut lalg_limpl = l_alg_alloc_limpl(n);
    let mut positions = Vec::<(usize, usize)>::new();

    l_alg_init_from_ord(&mut lalg_limpl, n, pord, n - 1);
    l_alg_init_get_positions_new(pord, &mut positions, n);

    // Apply init_vector
    let mut b_first = true;
    let min_len = std::cmp::min(positions.len(), init_vector.len());

    for i in 0..min_len {
        if b_print {
            if b_first {
                b_first = false;
            } else {
                eprint!(", ");
            }
        }

        let (x, y) = positions[i];
        let e = init_vector[i];

        if b_print {
            eprint!("({},{}) = {} ", x, y, e);
        }

        if b_test {
            if e == n + 1 {
                if b_print {
                    eprint!("(skipping)");
                }
                continue;
            }

            if e == n - 1 {
                if b_print {
                    eprintln!("(Element at ({}, {}) cannot be equal to unit ({}).)", x, y, n - 1);
                }
                return;
            }

            for t in 0..y {
                let impl_t_y = lalg_limpl[idx(t, y, n)];
                let impl_x_t = lalg_limpl[idx(x, t, n)];

                if impl_t_y == n - 1 && impl_x_t != n + 1 && lalg_limpl[idx(impl_x_t, e, n)] != n - 1 {
                    if b_print {
                        eprintln!(
                            "(Element e={} at (x={}, y={}) needs to larger than {} since t={} <= y => x->t <= x->y.)",
                            e, x, y, impl_x_t, t
                        );
                    }
                    return;
                }
            }
        }

        lalg_limpl[idx(x, y, n)] = e;

        if b_test {
            if !l_alg_test_ax4_partial(&lalg_limpl, n, true) {
                return;
            }
        }
    }

    eprintln!();

    for i in (0..min_len).rev() {
        if init_vector[i] != n + 1 {
            positions.remove(i);
        }
    }

    if b_print {
        eprintln!("Positions: {positions:?}");
        eprintln!("Init limpl: {lalg_limpl:?}");
    }

    let time_start = Instant::now();
    let mut num_tested = 0usize;
    let mut num_models = 0usize;

    gen_all_lalgs_rec(
        0,
        &positions,
        &mut lalg_limpl,
        n,
        n - 1,
        lalgs,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
    );

    eprintln!("Computation time: {:.4} s", time_start.elapsed().as_secs_f32());
    eprintln!("Number recursive calls: {}", num_tested);
    eprintln!("Number of all models: {}", num_models);
    eprintln!("Number of representative models {}", lalgs.len());
}    

pub fn gen_lalgs_from_pord_rec_with_positions(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
) {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using flat 1D array indexing
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            gen_lalgs_from_pord_rec_with_positions(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
            );
        }

        // Backtrack: reset state
        limpl[idx(x, y, n)] = n + 1;
    }
}

pub fn l_alg_gen_from_ord_new_with_positions(
    pord: &[usize],
    positions: &[(usize, usize)],
    n: usize,
    unit_elem: usize,
    status_output_limit: usize,
) -> HashSet<Vec<usize>> {
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    let mut limpl = vec![n + 1; n * n];
    let mut res = HashSet::<Vec<usize>>::new();

    // Initialize implication array from the partial order slice
    l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);

    let start_time = Instant::now();

    gen_lalgs_from_pord_rec_with_positions(
        0,
        positions,
        &mut limpl,
        n,
        unit_elem,
        &mut res,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
    );

    let duration = start_time.elapsed();

    eprintln!(
        "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
        duration,
        num_tested,
        num_models,
        res.len()
    );

    res
}

pub fn gen_lalgs_from_pord_short_iter_rec(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
) {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using flat 1D array indexing
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            gen_lalgs_from_pord_short_iter_rec(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
            );
        }

        // Backtrack: reset state
        limpl[idx(x, y, n)] = n + 1;
    }
}

pub fn l_alg_gen_from_ord_short_iter(
    pord: &[usize],
    n: usize,
    unit_elem: usize,
    status_output_limit: usize,
) -> HashSet<Vec<usize>> {
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    let mut limpl = vec![n + 1; n * n];
    let mut positions = Vec::<(usize, usize)>::new();
    let mut res = HashSet::<Vec<usize>>::new();

    // Initialize implication array from partial order slice and collect positions
    l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);
    l_alg_init_get_positions_new(pord, &mut positions, n);

    let start_time = Instant::now();

    gen_lalgs_from_pord_short_iter_rec(
        0,
        &positions,
        &mut limpl,
        n,
        unit_elem,
        &mut res,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
    );

    let duration = start_time.elapsed();

    eprintln!(
        "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
        duration,
        num_tested,
        num_models,
        res.len()
    );

    res
}

pub fn gen_lalgs_from_pord_short_iter_rec_limit_old(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
    model_limit: usize,
) -> bool {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }

            if model_limit > 0 && res.len() >= model_limit {
                return true; // Stop early once limit is hit
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using flattened 1D array indexing
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            if gen_lalgs_from_pord_short_iter_rec_limit_old(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
                model_limit,
            ) {
                return true;
            }
        }

        // Reset state for backtracking
        limpl[idx(x, y, n)] = n + 1;
    }

    false
}

pub fn l_alg_gen_from_ord_short_iter_limit_old(
    pord: &[usize],
    n: usize,
    unit_elem: usize,
    status_output_limit: usize,
    model_limit: usize,
) -> HashSet<Vec<usize>> {
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    let mut limpl = vec![n + 1; n * n];
    let mut positions = Vec::<(usize, usize)>::new();
    let mut res = HashSet::<Vec<usize>>::new();

    // Initialize implication table using old position extraction logic
    l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);
    l_alg_init_get_positions_old(pord, &mut positions, n);

    let start_time = Instant::now();

    gen_lalgs_from_pord_short_iter_rec_limit_old(
        0,
        &positions,
        &mut limpl,
        n,
        unit_elem,
        &mut res,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
        model_limit,
    );

    let duration = start_time.elapsed();

    eprintln!(
        "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
        duration,
        num_tested,
        num_models,
        res.len()
    );

    res
}

pub fn gen_lalgs_from_pord_short_iter_rec_limit_new(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
    model_limit: usize,
) -> bool {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }

            if model_limit > 0 && res.len() >= model_limit {
                return true; // Stop early once model limit is reached
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using flat 1D indexing
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            if gen_lalgs_from_pord_short_iter_rec_limit_new(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
                model_limit,
            ) {
                return true;
            }
        }

        // Reset state for backtracking
        limpl[idx(x, y, n)] = n + 1;
    }

    false
}

pub fn l_alg_gen_from_ord_short_iter_limit_new(
    pord: &[usize],
    n: usize,
    unit_elem: usize,
    status_output_limit: usize,
    model_limit: usize,
) -> HashSet<Vec<usize>> {
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    let mut limpl = vec![n + 1; n * n];
    let mut positions = Vec::<(usize, usize)>::new();
    let mut res = HashSet::<Vec<usize>>::new();

    // Uses the updated position extraction logic for optimized search ordering
    l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);
    l_alg_init_get_positions_new(pord, &mut positions, n);

    let start_time = Instant::now();

    gen_lalgs_from_pord_short_iter_rec_limit_new(
        0,
        &positions,
        &mut limpl,
        n,
        unit_elem,
        &mut res,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
        model_limit,
    );

    let duration = start_time.elapsed();

    eprintln!(
        "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
        duration,
        num_tested,
        num_models,
        res.len()
    );

    res
}

pub fn gen_lalgs_from_pord_short_time_rec(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
) {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using 1D flat layout
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            gen_lalgs_from_pord_short_time_rec(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
            );
        }

        // Backtracking: reset unassigned state
        limpl[idx(x, y, n)] = n + 1;
    }
}

pub fn l_alg_gen_from_ord_short_time(
    pord: &[usize],
    n: usize,
    unit_elem: usize,
    status_output_limit: usize,
) -> HashSet<Vec<usize>> {
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    let mut limpl = vec![n + 1; n * n];
    let mut positions = Vec::<(usize, usize)>::new();
    let mut res = HashSet::<Vec<usize>>::new();

    // Initialize implication matrix and target position sequence
    l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);
    l_alg_init_get_positions_new(pord, &mut positions, n);

    let start_time = Instant::now();

    gen_lalgs_from_pord_short_time_rec(
        0,
        &positions,
        &mut limpl,
        n,
        unit_elem,
        &mut res,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
    );

    let duration = start_time.elapsed();

    eprintln!(
        "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
        duration,
        num_tested,
        num_models,
        res.len()
    );

    res
}

pub fn gen_lalgs_from_pord_short_time_with_limit_rec_old(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
    model_limit: usize,
) -> bool {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }

            if model_limit > 0 && res.len() >= model_limit {
                return true; // Reached target model limit; signal early termination
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using flattened 1D array indexing
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            if gen_lalgs_from_pord_short_time_with_limit_rec_old(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
                model_limit,
            ) {
                return true; // Propagate exit signal up the stack
            }
        }

        // Reset state for backtracking
        limpl[idx(x, y, n)] = n + 1;
    }

    false
}

pub fn l_alg_gen_from_ord_short_time_with_limit_old(
    pord: &[usize],
    n: usize,
    unit_elem: usize,
    status_output_limit: usize,
    model_limit: usize,
) -> HashSet<Vec<usize>> {
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    let mut limpl = vec![n + 1; n * n];
    let mut positions = Vec::<(usize, usize)>::new();
    let mut res = HashSet::<Vec<usize>>::new();

    // Use old position generator variant
    l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);
    l_alg_init_get_positions_old(pord, &mut positions, n);

    let start_time = Instant::now();

    gen_lalgs_from_pord_short_time_with_limit_rec_old(
        0,
        &positions,
        &mut limpl,
        n,
        unit_elem,
        &mut res,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
        model_limit,
    );

    let duration = start_time.elapsed();

    eprintln!(
        "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
        duration,
        num_tested,
        num_models,
        res.len()
    );

    res
}

pub fn gen_lalgs_from_pord_short_time_with_limit_rec_new(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
    model_limit: usize,
) -> bool {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }

            if model_limit > 0 && res.len() >= model_limit {
                return true; // Reached target model limit; signal early termination
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using flattened 1D array indexing
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            if gen_lalgs_from_pord_short_time_with_limit_rec_new(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
                model_limit,
            ) {
                return true; // Propagate exit signal up the stack
            }
        }

        // Reset state for backtracking
        limpl[idx(x, y, n)] = n + 1;
    }

    false
}

pub fn l_alg_gen_from_ord_short_time_with_limit_new(
    pord: &[usize],
    n: usize,
    unit_elem: usize,
    status_output_limit: usize,
    model_limit: usize,
) -> HashSet<Vec<usize>> {
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    let mut limpl = vec![n + 1; n * n];
    let mut positions = Vec::<(usize, usize)>::new();
    let mut res = HashSet::<Vec<usize>>::new();

    // Use updated position generator variant for optimal search-space ordering
    l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);
    l_alg_init_get_positions_new(pord, &mut positions, n);

    let start_time = Instant::now();

    gen_lalgs_from_pord_short_time_with_limit_rec_new(
        0,
        &positions,
        &mut limpl,
        n,
        unit_elem,
        &mut res,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
        model_limit,
    );

    let duration = start_time.elapsed();

    eprintln!(
        "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
        duration,
        num_tested,
        num_models,
        res.len()
    );

    res
}

pub fn hashmap_perm_image(fun: &HashMap::<(usize,usize), usize>, perm: &Vec<usize>) -> HashMap::<(usize,usize), usize> {
    let mut res_hm = HashMap::<(usize,usize), usize>::new();
    

    for k in fun.keys() {
        let v = fun[k];
        res_hm.insert((perm[k.0],perm[k.1]), perm[v]);
    }
    res_hm
}

pub fn perm_hashset_get_images(
    base_limpl: &[usize],
    perm_set: &HashSet<Vec<usize>>,
    iso_perm_vec: &mut [usize],
    n: usize,
    unit_elem: usize,
) -> HashSet<Vec<usize>> {
    let mut res = HashSet::<Vec<usize>>::new();

    for perm in perm_set {
        // Map elements using the permutation vector
        iso_perm_vec.copy_from_slice(perm);

        let (limpl_img, _) = l_alg_isomorphic_image(base_limpl, n, unit_elem, iso_perm_vec);
        res.insert(limpl_img);
    }

    res
}

pub fn perm_iter_get_images(
    base_limpl: &[usize],
    perm_iter: itertools::Permutations<std::slice::Iter<'_, usize>>,
    base_perm_vec: &[usize],
    iso_perm_vec: &mut [usize],
    n: usize,
    unit_elem: usize,
) -> HashSet<Vec<usize>> {
    let mut res = HashSet::<Vec<usize>>::new();
    let num_base_elements = base_perm_vec.len();

    for perm in perm_iter {
        for j in 0..num_base_elements {
            iso_perm_vec[base_perm_vec[j]] = *perm[j];
        }

        let (limpl_img, _) = l_alg_isomorphic_image(base_limpl, n, unit_elem, iso_perm_vec);
        res.insert(limpl_img);
    }

    res
}

pub fn perm_iter_get_images_new(
    base_limpl: &[usize],
    perm_iter: itertools::Permutations<std::slice::Iter<'_, usize>>,
    base_perm_vec: &[usize],
    iso_perm_vec: &mut [usize],
    n: usize,
    unit_elem: usize,
) -> HashSet<Vec<usize>> {
    let mut res = HashSet::<Vec<usize>>::new();
    let num_base_elements = base_perm_vec.len();

    for perm in perm_iter {
        for j in 0..num_base_elements {
            iso_perm_vec[base_perm_vec[j]] = *perm[j];
        }

        // Optimized order preservation check on flat slice layout
        if !l_alg_perm_preserve_original_ord(base_limpl, n, iso_perm_vec) {
            continue;
        }

        let (limpl_img, _) = l_alg_isomorphic_image(base_limpl, n, unit_elem, iso_perm_vec);
        res.insert(limpl_img);
    }

    res
}

pub fn perm_iter_get_images_new2(
    base_limpl: &[usize],
    target_ord: &[usize],
    perm_iter: itertools::Permutations<std::slice::Iter<'_, usize>>,
    base_perm_vec: &[usize],
    iso_perm_vec: &mut [usize],
    n: usize,
    unit_elem: usize,
) -> HashSet<Vec<usize>> {
    let mut res = HashSet::<Vec<usize>>::new();
    let num_base_elements = base_perm_vec.len();

    for perm in perm_iter {
        for j in 0..num_base_elements {
            iso_perm_vec[base_perm_vec[j]] = *perm[j];
        }

        // Filter out permutations that do not preserve the explicit target partial order
        if !pord_perm_preserve_ord(target_ord, n, iso_perm_vec) {
            continue;
        }

        let (limpl_img, _) = l_alg_isomorphic_image(base_limpl, n, unit_elem, iso_perm_vec);
        res.insert(limpl_img);
    }

    res
}

const SHORT_STOP_TIME_DURATION:Duration = Duration::new(60,0);

pub fn gen_all_lalgs_rec_short_time(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
) {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using flattened 1D array indexing
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            gen_all_lalgs_rec_short_time(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
            );
        }

        // Reset state for backtracking
        limpl[idx(x, y, n)] = n + 1;
    }
}

pub fn l_alg_gen_all_short_time(n: usize, status_output_limit: usize) -> HashSet<Vec<usize>> {
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    let mut limpl = vec![n + 1; n * n];
    let mut positions = Vec::<(usize, usize)>::new();
    let mut res = HashSet::<Vec<usize>>::new();

    let unit = 0;
    l_alg_init_limpl(&mut limpl, n);

    for x in 1..n {
        for y in 1..n {
            if x != y {
                positions.push((x, y));
            }
        }
    }

    let start_time = Instant::now();

    gen_all_lalgs_rec_short_time(
        0,
        &positions,
        &mut limpl,
        n,
        unit,
        &mut res,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
    );

    let duration = start_time.elapsed();

    eprintln!(
        "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
        duration,
        num_tested,
        num_models,
        res.len()
    );

    res
}

pub fn gen_all_lalgs_rec_short_time_with_limit(
    index: usize,
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    n: usize,
    unit: usize,
    res: &mut HashSet<Vec<usize>>,
    num_tested: &mut usize,
    num_models: &mut usize,
    status_output_limit: usize,
    model_limit: usize,
) -> bool {
    let num_positions = positions.len();
    *num_tested += 1;

    if *num_tested % status_output_limit == 1 {
        eprintln!("Cur_progress: {limpl:?}");
    }

    if index >= num_positions {
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            *num_models += 1;
            let l_alg_repr = l_alg_get_repr(limpl, n, true, true);
            if res.insert(l_alg_repr.clone()) {
                println!("{:?}", l_alg_repr);
            }

            if model_limit > 0 && res.len() >= model_limit {
                return true; // Reached the target model limit, signal early stop
            }
        }
    } else {
        let (x, y) = positions[index];

        for e in 0..n {
            if e == unit {
                continue;
            }

            // Pruning check using flattened 1D array indexing
            let mut b_found = false;
            for t in 0..y {
                if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                    b_found = true;
                    break;
                }
            }

            if b_found {
                continue;
            }

            limpl[idx(x, y, n)] = e;

            if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
                limpl[idx(x, y, n)] = n + 1;
                continue;
            }

            if gen_all_lalgs_rec_short_time_with_limit(
                index + 1,
                positions,
                limpl,
                n,
                unit,
                res,
                num_tested,
                num_models,
                status_output_limit,
                model_limit,
            ) {
                return true; // Propagate the early exit signal up the recursion stack
            }
        }

        // Reset state for backtracking
        limpl[idx(x, y, n)] = n + 1;
    }

    false
}

pub fn l_alg_gen_all_short_time_with_limit(
    n: usize,
    status_output_limit: usize,
    model_limit: usize,
) -> HashSet<Vec<usize>> {
    let mut num_tested = 0usize;
    let mut num_models = 0usize;
    let mut limpl = vec![n + 1; n * n];
    let mut positions = Vec::<(usize, usize)>::new();
    let mut res = HashSet::<Vec<usize>>::new();

    let unit = 0;
    l_alg_init_limpl(&mut limpl, n);

    for x in 1..n {
        for y in 1..n {
            if x != y {
                positions.push((x, y));
            }
        }
    }

    let start_time = Instant::now();

    gen_all_lalgs_rec_short_time_with_limit(
        0,
        &positions,
        &mut limpl,
        n,
        unit,
        &mut res,
        &mut num_tested,
        &mut num_models,
        status_output_limit,
        model_limit,
    );

    let duration = start_time.elapsed();

    eprintln!(
        "Time elapsed: {:?}, num tested: {}, num models: {}, num repr models: {}",
        duration,
        num_tested,
        num_models,
        res.len()
    );

    res
}

// TODO

pub enum OutputType {
    Script,
    List,
}

pub fn get_plan_fixed_rec(
    limpl: &mut [usize],
    fixed: &mut [bool],
    positions: &[(usize, usize)],
    index: usize,
    n: usize,
    unit: usize,
    plan: &mut Vec<usize>,
) -> bool {
    let num_positions = positions.len();

    if index >= num_positions {
        return true;
    }

    let (x, y) = positions[index];

    // If already fixed, record value and continue
    if fixed[idx(x, y, n)] {
        plan.push(limpl[idx(x, y, n)]);
        return get_plan_fixed_rec(limpl, fixed, positions, index + 1, n, unit, plan);
    }

    // Attempt candidates for unassigned cell
    for e in 0..n {
        if e == unit {
            continue;
        }

        // Compatibility pruning check using 1D indexing
        let mut b_found = false;
        for t in 0..y {
            if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                b_found = true;
                break;
            }
        }

        if b_found {
            continue;
        }

        limpl[idx(x, y, n)] = e;
        fixed[idx(x, y, n)] = true;

        if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
            limpl[idx(x, y, n)] = n + 1;
            fixed[idx(x, y, n)] = false;
            continue;
        }

        plan.push(e);

        if get_plan_fixed_rec(limpl, fixed, positions, index + 1, n, unit, plan) {
            return true;
        }

        // Backtrack
        plan.pop();
        limpl[idx(x, y, n)] = n + 1;
        fixed[idx(x, y, n)] = false;
    }

    false
}

pub fn get_plan_fixed(
    pord: &[usize],
    positions: &[(usize, usize)],
    n: usize,
    unit_elem: usize,
) -> Option<Vec<usize>> {
    let mut limpl = vec![n + 1; n * n];
    let mut fixed = vec![false; n * n];
    let mut plan = Vec::new();

    // Initialize initial state from order
    l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);

    // Mark pre-filled cells in the 1D fixed mask
    for i in 0..n {
        for j in 0..n {
            if limpl[idx(i, j, n)] != n + 1 {
                fixed[idx(i, j, n)] = true;
            }
        }
    }

    if get_plan_fixed_rec(
        &mut limpl,
        &mut fixed,
        positions,
        0,
        n,
        unit_elem,
        &mut plan,
    ) {
        Some(plan)
    } else {
        None
    }
}

pub fn get_plan_fixed_rec_new(
    limpl: &mut [usize],
    fixed: &mut [bool],
    positions: &[(usize, usize)],
    index: usize,
    n: usize,
    unit: usize,
    plan: &mut Vec<usize>,
) -> bool {
    let num_positions = positions.len();

    if index >= num_positions {
        return true;
    }

    let (x, y) = positions[index];

    // If cell is already assigned / fixed, record it in the plan and proceed
    if fixed[idx(x, y, n)] {
        plan.push(limpl[idx(x, y, n)]);
        return get_plan_fixed_rec_new(limpl, fixed, positions, index + 1, n, unit, plan);
    }

    // Attempt candidates for unassigned cell
    for e in 0..n {
        if e == unit {
            continue;
        }

        // Compatibility pruning check using 1D indexing
        let mut b_found = false;
        for t in 0..y {
            if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                b_found = true;
                break;
            }
        }

        if b_found {
            continue;
        }

        limpl[idx(x, y, n)] = e;
        fixed[idx(x, y, n)] = true;

        if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
            limpl[idx(x, y, n)] = n + 1;
            fixed[idx(x, y, n)] = false;
            continue;
        }

        plan.push(e);

        if get_plan_fixed_rec_new(limpl, fixed, positions, index + 1, n, unit, plan) {
            return true;
        }

        // Backtrack
        plan.pop();
        limpl[idx(x, y, n)] = n + 1;
        fixed[idx(x, y, n)] = false;
    }

    false
}

pub fn get_plan_fixed_new(
    pord: &[usize],
    positions: &[(usize, usize)],
    n: usize,
    unit_elem: usize,
) -> Option<Vec<usize>> {
    let mut limpl = vec![n + 1; n * n];
    let mut fixed = vec![false; n * n];
    let mut plan = Vec::new();

    // Initialize initial state from partial order using new position ordering strategy
    l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);

    // Mark pre-filled cells in the 1D fixed mask
    for i in 0..n {
        for j in 0..n {
            if limpl[idx(i, j, n)] != n + 1 {
                fixed[idx(i, j, n)] = true;
            }
        }
    }

    if get_plan_fixed_rec_new(
        &mut limpl,
        &mut fixed,
        positions,
        0,
        n,
        unit_elem,
        &mut plan,
    ) {
        Some(plan)
    } else {
        None
    }
}

pub fn get_plan_fixed_rec_new2(
    index: usize,
    num_iter: &mut usize,
    n: usize,
    pord: &[usize],
    num_pord: usize,
    fixed_vec: &[(usize, usize)],
    positions: &[(usize, usize)],
    limpl: &mut [usize],
    output_type: &OutputType,
) -> bool {
    let num_positions = positions.len();
    let unit = n - 1;

    if index >= num_positions {
        *num_iter += 1;
        if l_alg_is_l_algebra(limpl, n, unit, false) {
            match output_type {
                OutputType::List => {
                    let plan: Vec<usize> = fixed_vec
                        .iter()
                        .map(|&(r, c)| limpl[idx(r, c, n)])
                        .collect();
                    println!("{:?}", plan);
                }
                OutputType::Script => {}
            }
            return true;
        }
        return false;
    }

    let (x, y) = positions[index];

    // If cell was pre-assigned via init_vector / fixed_vec, advance to next position
    if limpl[idx(x, y, n)] != n + 1 {
        return get_plan_fixed_rec_new2(
            index + 1,
            num_iter,
            n,
            pord,
            num_pord,
            fixed_vec,
            positions,
            limpl,
            output_type,
        );
    }

    // Branch on all candidate elements
    for e in 0..n {
        if e == unit {
            continue;
        }

        // 1D compatibility pruning check
        let mut b_found = false;
        for t in 0..y {
            if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                b_found = true;
                break;
            }
        }

        if b_found {
            continue;
        }

        limpl[idx(x, y, n)] = e;

        // Axiom 4 check on 1D matrix
        if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
            limpl[idx(x, y, n)] = n + 1;
            continue;
        }

        if get_plan_fixed_rec_new2(
            index + 1,
            num_iter,
            n,
            pord,
            num_pord,
            fixed_vec,
            positions,
            limpl,
            output_type,
        ) {
            return true;
        }

        // Backtrack: restore unassigned state
        limpl[idx(x, y, n)] = n + 1;
    }

    false
}

// pub fn get_plan_fixed_new2(
//     pord: &[usize],
//     positions: &[(usize, usize)],
//     n: usize,
//     unit_elem: usize,
// ) -> Option<Vec<usize>> {
//     let mut limpl = vec![n + 1; n * n];
//     let mut fixed = vec![false; n * n];
//     let mut plan = Vec::new();

//     // Initialize initial state from partial order using secondary position ordering strategy
//     l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);

//     // Mark pre-filled cells in the 1D fixed mask
//     for i in 0..n {
//         for j in 0..n {
//             if limpl[idx(i, j, n)] != n + 1 {
//                 fixed[idx(i, j, n)] = true;
//             }
//         }
//     }

//     if get_plan_fixed_rec_new2(
//         &mut limpl,
//         &mut fixed,
//         positions,
//         0,
//         n,
//         unit_elem,
//         &mut plan,
//     ) {
//         Some(plan)
//     } else {
//         None
//     }
// }

pub fn get_plan_continue_rec(
    limpl: &mut [usize],
    fixed: &mut [bool],
    positions: &[(usize, usize)],
    index: usize,
    n: usize,
    unit: usize,
    plan: &mut Vec<usize>,
) -> bool {
    let num_positions = positions.len();

    if index >= num_positions {
        return true;
    }

    let (x, y) = positions[index];

    // If cell is already fixed, record value and continue to next position
    if fixed[idx(x, y, n)] {
        plan.push(limpl[idx(x, y, n)]);
        return get_plan_continue_rec(limpl, fixed, positions, index + 1, n, unit, plan);
    }

    // Determine start candidate value: if plan has a value recorded for this slot, resume from it
    let start_e = if index < plan.len() {
        plan[index]
    } else {
        0
    };

    for e in start_e..n {
        if e == unit {
            continue;
        }

        // Compatibility pruning check using 1D indexing
        let mut b_found = false;
        for t in 0..y {
            if limpl[idx(t, y, n)] == unit && limpl[idx(limpl[idx(x, t, n)], e, n)] != unit {
                b_found = true;
                break;
            }
        }

        if b_found {
            continue;
        }

        limpl[idx(x, y, n)] = e;
        fixed[idx(x, y, n)] = true;

        if !l_alg_test_ax4_partial_xy(limpl, n, x, y, false) {
            limpl[idx(x, y, n)] = n + 1;
            fixed[idx(x, y, n)] = false;
            continue;
        }

        // Record or update element at current position in plan
        if index < plan.len() {
            plan[index] = e;
        } else {
            plan.push(e);
        }

        if get_plan_continue_rec(limpl, fixed, positions, index + 1, n, unit, plan) {
            return true;
        }

        // Backtrack
        limpl[idx(x, y, n)] = n + 1;
        fixed[idx(x, y, n)] = false;
    }

    // Truncate plan back to current index level on failure
    plan.truncate(index);
    false
}

pub fn get_plan_continue(
    pord: &[usize],
    positions: &[(usize, usize)],
    n: usize,
    unit_elem: usize,
    existing_plan: &mut Vec<usize>,
) -> Option<Vec<usize>> {
    let mut limpl = vec![n + 1; n * n];
    let mut fixed = vec![false; n * n];

    // Initialize state from partial order slice
    l_alg_init_from_ord(&mut limpl, n, pord, unit_elem);

    // Mark pre-filled cells in the 1D fixed mask
    for i in 0..n {
        for j in 0..n {
            if limpl[idx(i, j, n)] != n + 1 {
                fixed[idx(i, j, n)] = true;
            }
        }
    }

    if get_plan_continue_rec(
        &mut limpl,
        &mut fixed,
        positions,
        0,
        n,
        unit_elem,
        existing_plan,
    ) {
        Some(existing_plan.clone())
    } else {
        None
    }
}

// Wrong translation
// pub fn transform_init_vector(
//     limpl: &[usize],
//     perm: &[usize],
//     n: usize,
//     unit_elem: usize,
// ) -> Vec<usize> {
//     let mut transformed = vec![n + 1; n * n];

//     for i in 0..n {
//         for j in 0..n {
//             let val = limpl[idx(i, j, n)];
//             if val < n {
//                 // Apply permutation mapping to elements and table indices
//                 transformed[idx(perm[i], perm[j], n)] = perm[val];
//             } else if val == unit_elem {
//                 transformed[idx(perm[i], perm[j], n)] = unit_elem;
//             }
//         }
//     }

//     transformed
// }

// pub fn gen_plans_new2(
//     pord: &[usize],
//     num_pord: usize,
//     fixed_vec: &[(usize, usize)],
//     init_vector: &[usize],
//     n: usize,
// ) {
//     let unit_elem = n - 1;
//     let mut lalg_limpl = vec![n + 1; n * n];
//     let mut positions = Vec::<(usize, usize)>::new();

//     // Initialize 1D matrix and target position sequence
//     l_alg_init_from_ord(&mut lalg_limpl, n, pord, unit_elem);
//     l_alg_init_get_positions_old(pord, &mut positions, n);

//     // Validate and apply initial values
//     for i in 0..init_vector.len() {
//         let (row, col) = fixed_vec[i];
//         let val = init_vector[i];

//         if l_alg_test_init_value(&mut lalg_limpl, n, n - 1, row, col, val, false) {
//             lalg_limpl[idx(row, col, n)] = val;
//         } else {
//             return;
//         }
//     }

//     // limpl: &mut [usize],
//     // fixed: &mut [bool],
//     // positions: &[(usize, usize)],
//     // index: usize,
//     // n: usize,
//     // unit: usize,
//     // plan: &mut Vec<usize>,


//     let mut num_iter = 0usize;
//     get_plan_fixed_rec_new2(
//         &mut lalg_limpl,
//         init_vector.len(),
//         &mut num_iter,
//         n,
//         pord,
//         num_pord,
//         fixed_vec,
//         &positions,

//         &OutputType::List,
//     );

//     eprintln!("Finished.");
// }

// pub fn gen_plans_main_new2(pord: &[usize], n: usize, num_pord: usize, fixed_vec: &[(usize, usize)]) {
//     let ts = Instant::now();
//     let mut from_vec = Vec::<usize>::new();

//     if std::env::args().len() == 2 {
//         from_vec = std::env::args()
//             .nth(1)
//             .unwrap()
//             .split(',')
//             .map(|v| v.trim().parse::<usize>().unwrap())
//             .collect();
//     }

//     gen_plans_new2(pord, num_pord, fixed_vec, &from_vec, n);
//     eprintln!("Time elapsed: {:.4}s", ts.elapsed().as_secs_f32());
// }

