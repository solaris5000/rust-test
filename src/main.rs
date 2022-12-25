use std::io;

//function to get a next number
fn getNumber() -> f32 {
    0.0
}

//function to get a next char to parse it in operation
fn getChar() -> char {
    '+'
}

// well, that func printing 5 last operations, if there was 5 operations
fn printHistory(HistoryArray :&[(bool,f32, char, f32, f32)]) {
    print!("There is no history for now ;)\n")
}

//fucn to do all the math in this prog.
fn doMath (var_a :&f32, var_b :&f32, sym :&char) -> f32 {
    return 0.0;
}
 
fn main() -> io::Result<()> {

    let mut f_var_a : f32 = 0.0;
    let mut f_var_b : f32 = 0.0;
    let mut f_result : f32 = 0.0;
    let mut choise : char = '+';

    // This array will be used to store history of operations.
    let mut a_history_of_t: [(bool,f32, char, f32, f32); 5] = [
        (false, 0.0, '0', 0.0, 0.0),
        (false, 0.0, '0', 0.0, 0.0),
        (false, 0.0, '0', 0.0, 0.0),
        (false, 0.0, '0', 0.0, 0.0),
        (false, 0.0, '0', 0.0, 0.0)];

/* 
    loop{
        f_var_a = getNumber();
        choise = getChar();
        f_var_b = getNumber();
        f_result = doMath(&f_var_a, &f_var_b, &choise);
    }
*/
    let mut input = String::new();

    println!("Enter 1 var: ");
    io::stdin().read_line(&mut input)?;

    let trimmed = input.trim();
    match trimmed.parse::<f32>() {
        Ok(i) => {println!("Ok"); f_var_a = i},
        Err(..) => println!("this was not an valid number: {}", trimmed),
    };
    input.clear();

    println!("Enter 2 var: ");
    io::stdin().read_line(&mut input)?;
    

    let trimmed = input.trim();
    match trimmed.parse::<f32>() {
        Ok(i) => {println!("Ok"); f_var_b = i},
        Err(..) => println!("this was not an valid number: {}", trimmed),
    };
    input.clear();
    println!("Enter choise: ");
    io::stdin().read_line(&mut input)?;

    let trimmed = input.trim();
    match trimmed.parse::<char>() {
        Ok(i) => {println!("Ok"); choise = i},
        Err(..) => println!("this was not an valid number: {}", trimmed),
    };
    input.clear();
    if choise == '+' {
        f_result = f_var_a + f_var_b;
    } else if choise == '-' {
        f_result = f_var_a - f_var_b;
    } else if choise == '*' {
        f_result = f_var_a * f_var_b;
    } else
    if choise == '/' {
        f_result = f_var_a / f_var_b;
    } else
    {
        print!("Wrong operator - {}", choise);
        return Ok(());
    }
    
    print!("The result of calculation is {}", f_result);
    Ok(())
}
