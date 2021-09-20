use std::fs;
use std::path::Path;

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    let client = reqwest::Client::new();
    let image_file = client
        .get("https://images.pexels.com/photos/2124773/pexels-photo-2124773.jpeg")
        .send()
        .await;

    let path = Path::new("img_test.jpeg");
    let display = path.display();

    let b = image_file.unwrap().bytes().await.unwrap();
    match fs::write("./test.jpeg", &b) {
        Err(_) => panic!("couldn't write to {}", display),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    Ok(())
}
