use std::collections::HashSet;
use std::io::{stdin, BufRead, BufReader};
use std::fs::File;

// fn main() {
//     let input=String::from("-");
//     let mut res_file = BufReader::new(&std::io::Read);
//     if input == "-" {
//         res_file = BufReader::new(stdin().lock() as dyn std::io::Read);
//     }
//     else {
//         res_file = BufReader::new(File::open(input).expect("Cannot open `ord_filename` file") as dyn std::io::Read);
//     }
//     return;
//     // let res_file = std::io::stdin();
//     

//     for res_line in res_file.lines() {
//         let cur_result = serde_json::from_str::<Vec<Vec<usize>>>(&res_line.unwrap()).unwrap();
//         if all_results.insert(cur_result.clone()) {
//             println!("{:?}", cur_result);
//         }    
//     }
// }

fn main() {
    let args_len = std::env::args().len();

    if args_len < 2 {
        println!("Usage: {} <input filename | - (for stdin)>", std::env::args().next().unwrap());
        return;
    }

    let filename = std::env::args().nth(1).unwrap();

    if filename == "-" {
        merge_fun(BufReader::new(stdin().lock()));
    }
    else {
        merge_fun(BufReader::new(File::open(filename).expect("Cannot open `ord_filename` file")));
    }    
}

fn merge_fun<T:std::io::Read>(res_file: BufReader<T>) {
    let mut all_results = HashSet::<Vec<Vec<usize>>>::new();
    for res_line in res_file.lines() {
        let cur_result = serde_json::from_str::<Vec<Vec<usize>>>(&res_line.unwrap()).unwrap();
        if all_results.insert(cur_result.clone()) {
            println!("{:?}", cur_result);
        }    
    }
}