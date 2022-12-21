fn main() {
    let some_string : &str = "This is a some test string for \n";
    print!("{}a user soalris5000\n", some_string);

    let mut loop_x : i32 = 0;
    loop {
        if loop_x > 10 {break;}
        print!("{}\n",loop_x);
        loop_x +=1;
    }
}
