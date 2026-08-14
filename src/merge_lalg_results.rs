use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {

    let res_file = std::io::stdin();
    let mut all_results = HashSet::<Vec<Vec<usize>>>::new();

    for res_line in res_file.lines() {
        let cur_result = serde_json::from_str::<Vec<Vec<usize>>>(&res_line.unwrap()).unwrap();
        if all_results.insert(cur_result.clone()) {
            println!("{:?}", cur_result);
        }    
    }
}