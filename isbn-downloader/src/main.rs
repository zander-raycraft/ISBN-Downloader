use reqwest::Client;
use serde::Deserialize;
use std::io;
use tokio;


#[derive(Deserialize)]
struct Book {
    // making the textbook silly thing
    title: String,
    download: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Ask user for ISBN
    let mut isbn = String::new();
    println!("Enter the ISBN number of the textbook:");
    io::stdin().read_line(&mut isbn)?;
    let isbn = isbn.trim();

    // Search Libgen
    let url = format!("http://libgen.rs/json.php?ids=1&fields=Title,Download&mode=last&search={}", isbn);
    let response: Vec<Book> = client.get(&url).send().await?.json().await?;

    // Download PDF if available
    if !response.is_empty() {
        let book = &response[0];
        println!("Found: {}", book.title);
        println!("Downloading...");
        let pdf_url = &book.download;
        let response = client.get(pdf_url).send().await?;
        let bytes = response.bytes().await?;
        let filename = format!("{}.pdf", book.title);
        std::fs::write(&filename, bytes)?;
        println!("Downloaded: {}", filename);
    } else {
        println!("No book found with the given ISBN.");
    }

    Ok(())
}