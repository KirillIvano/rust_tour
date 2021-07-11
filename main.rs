use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {    
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read");

        let guess: u32 = guess.trim().parse().expect("ftf");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("sm"),
            Ordering::Greater => println!("gr"),
            Ordering::Equal => {
                println!("eq");
                break;
            }
        };

        println!("your guess: {}", guess);
    }
}
