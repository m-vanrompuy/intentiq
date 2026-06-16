use qdrant_client::Qdrant;
use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder, PointStruct, UpsertPointsBuilder};
use qdrant_client::Payload;
use uuid::Uuid;
use crate::models::event::ActorSummary;


const COLLECTION_NAME: &str = "actors";
const VECTOR_SIZE: u64 = 768; 

pub async fn connect() -> Qdrant {
    Qdrant::from_url("http://qdrant:6334").build().unwrap()
}

pub async fn create_collection_if_not_exists(client: &Qdrant) {
    
    let collections = client.list_collections().await.unwrap();
    
   
    let exists = collections.collections
        .iter()
        .any(|c| c.name == COLLECTION_NAME);
    
    
    if !exists {
        client.create_collection(
            CreateCollectionBuilder::new(COLLECTION_NAME)
                .vectors_config(VectorParamsBuilder::new(VECTOR_SIZE, Distance::Cosine))
        ).await.unwrap();
        
        println!("Qdrant collection '{}' aangemaakt", COLLECTION_NAME);
    }
}

pub async fn save_actor_vector(client: &Qdrant, summary: &ActorSummary) {
    // 1. Tekst samenvoegen
    let text = format!(
        "{} {} {}",
        summary.actor,
        summary.intents.join(" "),
        summary.evidence.join(" ")
    );

    // 2. Embedding ophalen van Ollama
    let embedding = get_embedding(&text).await;

    let actor_uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, summary.actor.as_bytes());
    let points = vec![
        PointStruct::new(
            actor_uuid.to_string(),
            embedding,
            Payload::new()
        )
    ];

    client.upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, points)).await.unwrap();
}

async fn get_embedding(text: &str) -> Vec<f32> {
    let client = reqwest::Client::new();
    
    let body = serde_json::json!({
        "model": "nomic-embed-text",
        "input": text
    });

    let response = client
        .post("http://ollama:11434/api/embed")
        .json(&body)
        .send()
        .await.unwrap();
    
    let json: serde_json::Value = response.json().await.unwrap();
    
    json["embeddings"][0]
        .as_array().unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap() as f32)
        .collect()
}