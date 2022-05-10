#[derive(Serialize, Deserialize)]
struct Movie {
    id: String,
    title: String,
    poster: String,
    overview: String,
    release_date: i64,
    genres: Vec<String>,
}

use meilisearch_sdk::{
    indexes::*,
    client::*,
    search::*,
    settings::*
};
use serde::{Serialize, Deserialize};
use std::{io::prelude::*, fs::File};
use futures::executor::block_on;

fn main() { block_on(async move {
    let client = Client::new("http://localhost:7700", "masterKey");

    // reading and parsing the file
    let mut file = File::open("movies.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let moives_docs: Vec<Movie> = serde_json::from_str(&contents).unwrap();

    // adding documents
    client.index("movies").add_documents(&moives_docs, None).await.unwrap();
})}
