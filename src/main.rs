#[derive(Serialize, Deserialize)]
struct Movie {
    id: String,
    #[serde(flatten)]
    value: serde_json::Value,
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
    let movies = client.index("index");

    // reading and parsing the file
    let mut file = File::open("movies.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let moives_docs: Vec<Movie> = serde_json::from_str(&contents).unwrap();

    // adding documents
    client.index("movies").add_documents(&moives_docs, None).await.unwrap();

    // build a query ann execute it later
    let query: Query = Query::new(&movies)
    .with_query("batman")
    .build();

    // let results: SearchResults<Movie> = client.index("movies").execute_query(&query).await.unwrap();
    let mut synonyms = std::collections::HashMap::new();
    synonyms.insert(String::from("winnie"), vec![String::from("piglet")]);
    synonyms.insert(String::from("piglet"), vec![String::from("winnie")]);

    client.index("movies").set_synonyms(&synonyms).await.unwrap();
    // println!("{:?}", results);
})}
