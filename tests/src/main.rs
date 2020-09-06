use serde::Deserialize;
use timed::timed;

mod tests;

#[derive(Deserialize, Debug)]
struct Quote {
    text: Option<String>,
    author: Option<String>,
}

#[tokio::main]
async fn main() {
    get_random_quote().await
}

#[timed]
async fn get_random_quote() {
    let url = "https://type.fit/api/quotes";
    println!("Calling {}", url);

    let quotes: Vec<Quote> = reqwest::get(url)
        .await
        .unwrap()
        .json::<Vec<Quote>>()
        .await
        .unwrap();

    println!(
        "Quote of the day: \n\"{:?}\" - {:?}",
        quotes[0].text, quotes[0].author
    );
}

#[tokio::test]
async fn test_async() {
    get_random_quote().await
}
