use tests::tests_utils::get_random_quote;
use timed::timed;

#[tokio::main]
#[timed]
async fn main() {
    println!("Running main");
    get_random_quote().await
}


