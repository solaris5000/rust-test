use std::io;

//function to get a next number
fn getNumber() -> f32 {

    let mut input = String::new();

    println!("Enter a number: ");
    io::stdin().read_line(&mut input);

    let trimmed = input.trim();
    match trimmed.parse::<f32>() {
        Ok(i) => {input.clear(); return i},
        Err(..) => {println!("this was not an valid number: {}", trimmed); input.clear(); return 0.0},
    };
}

//function to get a next char to parse it in operation
fn getChar() -> char {
    let mut input = String::new();

    println!("Enter an operator (- + * /): ");
    io::stdin().read_line(&mut input);

    let trimmed = input.trim();
    match trimmed.parse::<char>() {
        Ok(i) => {input.clear(); return i},
        Err(..) => {println!("this was not an valid operator, set to +: {}", trimmed); input.clear(); return '+'},
    };
}

// well, that func printing 5 last operations, if there was 5 operations
fn printHistory(HistoryArray :&[(bool,f32, char, f32, f32)]) {
    print!("1. {} {} {} = {}", &HistoryArray[0].1, &HistoryArray[0].2, &HistoryArray[0].3, &HistoryArray[0].4)
}

fn writeHistory(HistoryArray :&mut [(bool,f32, char, f32, f32)], var_a :&f32, var_b :&f32, sym :&char, f_res :&f32) {
    HistoryArray[0] = (true, *var_a, *sym, *var_b, *f_res);
}

//fucn to do all the math in this prog.
fn doMath (var_a :&f32, var_b :&f32, sym :&char) -> f32 {

    match sym {
        '+' => {return var_a + var_b;},
        '-' => {return var_a - var_b;},
        '/' => {if (var_b == &0.0) { println!("Division by zero."); return 0.0;} else {return var_a / var_b;}},
        '*' => {return var_a * var_b;},
        _ => {println!("Something gone wrong."); return 0.0;},
    }
    
    return 0.0;
}
 
fn main() -> io::Result<()> {

    let mut f_var_a : f32 = 0.0;
    let mut f_var_b : f32 = 0.0;
    let mut f_result : f32 = 0.0;
    let mut choise : char = '+';

    // This array will be used to store history of operations.
    // (used, 1 var, operation, 2 var, result)
    let mut a_history_of_t: [(bool,f32, char, f32, f32); 5] = [
        (false, 0.0, '0', 0.0, 0.0),
        (false, 0.0, '0', 0.0, 0.0),
        (false, 0.0, '0', 0.0, 0.0),
        (false, 0.0, '0', 0.0, 0.0),
        (false, 0.0, '0', 0.0, 0.0)];
 
    loop{
        f_var_a = getNumber();
        choise = getChar();
        f_var_b = getNumber();
        f_result = doMath(&f_var_a, &f_var_b, &choise);
        writeHistory(&mut a_history_of_t, &f_var_a, &f_var_b, &choise, &f_result);
        
        printHistory(&mut a_history_of_t);

        break;
    }
    
    //print!("The result of calculation is {}", f_result);
    Ok(())
}
