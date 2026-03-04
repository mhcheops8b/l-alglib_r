use std::io::{BufReader};
use bzip2::read::{BzDecoder};
// use std::collections::{HashSet};
use std::fs::File;


fn main() {
    let args_len = std::env::args().len();

    if args_len < 2 {
        println!("Usage: {} <n>", std::env::args().next().unwrap());
        return;
    }

    let mut nn = 0usize;
    match std::env::args().nth(1).unwrap().trim().parse() {
        Ok(val) => {nn = val},
        Err(_e) => println!("First argument must be a number.")
    }
    if nn < 1 || nn > 9 {
        println!("<n> must be 1-9.");
        return;
    }

    let file_path = format!("c:/users/mhyck/source/repos/mhcheops8b/serde_test/all_qords{}.pickle.bz2", nn);

    let file = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut bz_decoder = BzDecoder::new(file);
    let qords:Vec<Vec<Vec<usize>>> = serde_pickle::from_reader(&mut bz_decoder, Default::default()).unwrap();
    eprintln!("{}", qords.len());
    let mut ord_count = 0usize;
    // let mut lalgs = HashSet::<Vec<Vec<usize>>>::new();
    //let mut num_tested = 0usize;
    //let mut num_models = 0usize;
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

            // eprintln!("HHEHE");
            // get order n+1 with n as maximal element
            let mut qord_n1 = l_alglib::l_alg_alloc_limpl(nn+1);
            for i in 0.. nn {
                for j in 0..nn {
                    qord_n1[i][j] = qord[i][j];
                }
            }
            for i in 0..(nn+1) {
                qord_n1[i][nn] = 1;
            }

            println!("{qord_n1:?}");
        }

    }
    eprintln!("{ord_count}");
}