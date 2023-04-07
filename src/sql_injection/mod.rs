use reqwest::blocking::Client;
use reqwest::Error;

#[tokio::main]
pub async fn do_throttled_request(url: &str){
    let response = reqwest::get(url).await.unwrap();
    println!("Response: {}", response.text().await.unwrap());
}