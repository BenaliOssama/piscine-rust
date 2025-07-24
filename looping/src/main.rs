use std::io;

fn main() {
    let target = String::from("The letter e");
    let mut number : u16 = 0 ;
    loop {
        number += 1 ;
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let mut input = String::new();
        io::stdin().read_line(&mut input);

        if target == input.trim() {
            println!("Number of trials: {}", number);
            break;
        }
    }
}
