use std::io;

fn main() {
   far_to_cel();
}

fn far_to_cel() {
   eprintln!("Please input temperature in Farenheit:");

   let mut temperature = String::new();

   //input temperature string
   io::stdin()
       .read_line(&mut temperature)
       .expect("Failed to read line");

   //parse temperature string
   let temperature: f32 = match temperature.trim().parse() {
      Ok(num) => num,
      Err(_) => return,
   };

   //convertion to Celsius
   let temperature = (temperature - 32.0) * 0.5556;

   eprintln!("The temperature in Celsius is: {temperature}")
}

fn gen_fib_nub() {

}

/*
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp (&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("CORRECT!");
                break;
            }
        }
    }
}
*/