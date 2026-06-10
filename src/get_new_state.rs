fn main() {
   
    let args_len = std::env::args().len();

    if args_len < 2 {
        println!("Usage: {} <lagl_state>", std::env::args().next().unwrap());
        return;
    }

    let lalg_state_init_string = std::env::args().nth(1).unwrap();
    
    // eprintln!("Input Order (str): {}", pord_init_string);

    let lalg_state_res = serde_json::from_str::<Vec<Vec<usize>>>(&lalg_state_init_string);
    
    let lalg_state = match lalg_state_res {
        Ok(p) => p,
        Err(err) => {eprintln!("Error parsing json (lalg_state): {err:?}"); return;}
    };

    let pord = l_alglib::l_alg_get_order(&lalg_state);

    let mut positions = Vec::<(usize,usize)>::new();
    l_alglib::l_alg_init_get_positions_new(&pord, &mut positions); 
    
    let mut b_first = true;
    for pos in positions {
        if b_first {
            b_first = false
        }
        else {
            print!(",");
        }
        print!("{}",lalg_state[pos.0][pos.1]);
    }
    println!();
}
