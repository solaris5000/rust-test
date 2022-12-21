use std::io;
 
fn main() -> io::Result<()> {

    let mut f_var_a : f32 = 0.0;
    let mut f_var_b : f32 = 0.0;

    let mut f_result : f32 = 0.0;

    let mut choise : char = '+';


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
