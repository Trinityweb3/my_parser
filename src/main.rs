use reqwest::Client;
use reqwest::header::USER_AGENT;
use std::fs::OpenOptions;
use std::io::Write;
use futures::join;

const URL1: &str = "https://www.floatrates.com/json-feeds.html";
const URL2: &str = "https://www.wikipedia.org/";
const PATH1: &str = "parsed_data1.html";
const PATH2: &str = "parsed_data2.html";
const CUSTOM_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";


#[tokio::main]
async fn main() {
    join!(write_to_file(PATH1, URL1), write_to_file(PATH2, URL2));
}
/// This function 
async fn get_response(url: &str, user_agent: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let res = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    let result = res
        .text()
        .await?;
    Ok(result)
}

async fn write_to_file(path: &str, url: &str) {
    match get_response(url, CUSTOM_USER_AGENT).await {
        Ok(body) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .expect("Error creating/writing a file\n:(");

            file.write_all(body.as_bytes()).expect("Error\n:(");
        },
        Err(_) => println!("ERROR\n:("),
    }
}
