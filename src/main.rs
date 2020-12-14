//use crate::io::sys::process::process_common::File;
//use crate::io::sys::ext::process::process::Path;
use rand::Rng;
use std::io;

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

   /* let mut client = reqwest::Client::new();
    let mut image_file = client
        .get("https://images.pexels.com/photos/2124773/pexels-photo-2124773.jpeg")
        .send();

*/
/*
    let path = Path::new("img_test.jpeg");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
    match std::io::copy(&mut image_file, &mut file) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
*/
    Ok(())
}

fn test() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut guess = String::new();
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if guess != "" {
        hello()
    }

    println!("You guessed: {}", guess);
}

fn hello() {
    println!("Hello, world!");
}
