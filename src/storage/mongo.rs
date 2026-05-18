use mongodb::{Client, Collection};
use crate::models::event::{Event, IntentResult};


pub async fn connect() -> Client {
   Client::with_uri_str("mongodb://localhost:27017").await.unwrap()
}

pub async fn save_events(client: &Client, events: Vec<Event>) {
    let db = client.database("intentiq");
    let collection: Collection<Event> = db.collection("events");
    collection.delete_many(mongodb::bson::doc! {}).await.unwrap();
    collection.insert_many(events).await.unwrap();
}

pub async fn save_results(client: &Client, results: Vec<IntentResult>) {
    let db = client.database("intentiq");
    let collection: Collection<IntentResult> = db.collection("results");
    collection.delete_many(mongodb::bson::doc! {}).await.unwrap();
    collection.insert_many(results).await.unwrap();
}