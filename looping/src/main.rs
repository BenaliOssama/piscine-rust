use std::io;

fn main() {
    let target = String::from("The letter e");
    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let mut input = String::new();
        io::stdin().read_line(&mut input);

        if target == input.trim() {
            break;
        }
    }
}
