use timed::timed;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Quote {
    text: Option<String>,
    author: Option<String>,
}

#[timed]
pub async fn get_random_quote() {
    let url = "https://type.fit/api/quotes";
    println!("Calling {}", url);

    let quotes: Vec<Quote> = reqwest::get(url)
        .await
        .unwrap()
        .json::<Vec<Quote>>()
        .await
        .unwrap();

    println!(
        "Quote of the day: \n{} - {}",
        quotes[0].text.as_ref().unwrap(),
        quotes[0].author.as_ref().unwrap()
    );
}