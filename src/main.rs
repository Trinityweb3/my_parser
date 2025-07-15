use std::fs::OpenOptions;
use reqwest::{header::USER_AGENT, Result};
use std::io::Write;
use reqwest::blocking::Client;
const URL: &str = "https://www.floatrates.com/json-feeds.html";
const CUSTOM_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";
const PATH: &str = "parse_result.txt";

fn main() {
    let client = Client::new();
    let response = client.get(URL).header(USER_AGENT,  CUSTOM_USER_AGENT);
    let data = response.send().expect("Error").text().expect("Error");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(PATH)
        .expect("Error reading/opening file");

    
    file.write_all(data.as_bytes()).expect("Error writing to file");
}














/*
fn main() {
    let num = [1, 2, 3, 4, 4, 1, 1, 5, 6];
    // let chars = ['x','g', 'r', 'w', 'a', 'x', 'g', 'x'];
    let result = find_re(&num);
    println!("{:?}", result);
}

fn find_re<T>(list: &T) -> Vec<T> {
    let mut vector: Vec<T> = Vec::new();
    for i in 0..(list.len()) {
        for j in (i+1)..(list.len()) {
            if list[i] == list[j] {
                if !vector.contains(&list[i]) {
                    vector.push(list[i]);
                }
            }
        }
    }

    return vector;
} */