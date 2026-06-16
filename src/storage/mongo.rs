use mongodb::{Client, Collection};
use crate::models::event::{ActorSummary, Event, IntentResult};


pub async fn connect() -> Client {
   Client::with_uri_str("mongodb://mongo:27017/intentiq").await.unwrap()
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

pub async fn save_actorsummary(client: &Client, summaries: Vec<ActorSummary>) {
    let db = client.database("intentiq");
    let collection: Collection<ActorSummary> = db.collection("actorsummary");
    collection.delete_many(mongodb::bson::doc! {}).await.unwrap();
    collection.insert_many(summaries).await.unwrap();
}