#![allow(dead_code)]
static N: usize = 8;


fn check_valid(last: usize, queens: &Vec<usize>) -> bool { //where last is index of last placed queen
    for x in 0..queens.len() {
        if x != last && queens[x] != 0 {
            let mut dx = (last - x) as i32;
            dx = dx.abs();
            let mut dy = (queens[last] - queens[x]) as i32;
            dy = dy.abs();
            if dx == dy {
                return false;
            }
        }
    }
    true
}

fn valid(queens: &Vec<usize>) -> bool {
    if queens.len() as i32 == 0 {
        return false;
    }
    for i in 0..queens.len() {
        
        if i == 0 {
            return false;
        }
    }
    true
}


fn print_board(queens: &Vec<usize>) {
    println!("");
    for r in 0..queens.len() {
        let mut out = vec!['_'; queens.len()];
        //let mut out = String::from_utf8(vec!['_'; len(queens)]);
        let index = queens[r];
        
        if index > 0 {
            out[index] = 'Q';
        }

        println!("{:?}", out);
    }
    println!("");
}
        
fn solve<'a, 'b>(variables: &'a Vec<usize>, values: &'a Vec<usize>, board: &'b Vec<usize>) -> Option<&'b Vec<usize>> {
    if valid(board) {
        return Some(board);
    }
    
    let mut new_vars = variables.clone();
    let mut choose_var: Option<usize> = None;
    /*if len(new_vars) > 0:
        choose_var = new_vars.pop() - 1

    random.shuffle(new_vars)


    if choose_var is not None:
        for val in values:
            new_board = board[:]
            new_board[choose_var] = val
            new_vals = values[:]
            new_vals.remove(val)
            random.shuffle(new_vals)
            
            if check_valid(choose_var, new_board):
                result = solve(new_vars, new_vals, new_board)
                if result is not None:
                    return result
    return None*/
    None
}
    
fn main() {
    /*global N
    if len(sys.argv) > 1:
        try:
            N = int(sys.argv[1])
        except:
            print("Please enter a valid number.")
            return 1

    queens = [0] * N
    variables = [i for i in range(1, N + 1)]
    values = [i for i in range(1, N + 1)]
    tic = time.time()
    out = solve(variables, values, queens)
    toc = time.time()
    print(toc - tic)
    */
    let mut queens = vec![0; N];
    let mut variables = vec![0; N];
    let mut values = vec![0; N];
    for i in 0..N {
        variables[i] = i + 1;
        values[i] = i + 1;
        println!("{}", i + 1);
    }
    print_board(&mut queens);
    //let mut result = solve(&mut variables, &mut values, &mut queens);
    
    
    //print(out)
    //print_board(out)
    
}
