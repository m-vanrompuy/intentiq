use crate::models::event::ActorSummary;
use std::env;



pub async fn analyze_with_llm(summary: &ActorSummary) -> Option<ActorSummary> {
    let evidence = summary
    .evidence
    .iter()
    .take(20)
    .cloned()
    .collect::<Vec<_>>()
    .join("\n- ");

    let api_key = env::var("OPENAI_API_KEY").ok()?;

    let chat_model =
    env::var("OPENAI_CHAT_MODEL")
        .unwrap_or("gpt-4o-mini".to_string());

    let prompt = format!(
        "Je bent een cybersecurity analyst. Analyseer de volgende actor en bepaal de aanvalsintentie.
        

Actor: {}
Gevonden intents door rules: {}
Evidence:
- {}

Geef je antwoord in dit exacte JSON formaat, houd reasoning onder 20 woorden.:
{{
  \"intents\": [\"reconnaissance\"|\"credential_access\"|\"privilege_escalation\"|\"persistence\"|\"data_exfiltration\"|\"benign\"],
  \"total_confidence\": 0.0-1.0,
  \"reasoning\": \"korte uitleg\"
}}

Geef ALLEEN het JSON object terug, geen andere tekst.",
        summary.actor,
        summary.intents.join(", "),
        evidence
    );


    let body = serde_json::json!({
        "model": chat_model,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "response_format": {
            "type": "json_object"
        }
    });

    let client = reqwest::Client::new(); 

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .ok()?; 

    let json: serde_json::Value = response.json().await.ok()?; 

    // println!(
    //     "OPENAI RESPONSE:\n{}",
    //     serde_json::to_string_pretty(&json).unwrap()
    // );

    let content = json["choices"][0]["message"]["content"]
    .as_str()?; 

    // println!("RAW MODEL OUTPUT:");
    // println!("{}", content);    
    let parsed: serde_json::Value = serde_json::from_str(content).ok()?;

    let intents = parsed["intents"]
        .as_array()?                                          // JSON array → Vec<Value>
        .iter()                                               // loop over de elementen
        .filter_map(|v| v.as_str().map(|s| s.to_string()))   // elke Value → String, ongeldige waarden skippen
        .collect();                                           // Vec<String>

    let total_confidence = parsed["total_confidence"].as_f64()?;

    let reasoning = parsed["reasoning"].as_str()?;

    Some(ActorSummary {
        actor: summary.actor.clone(),   
        intents,                         
        total_confidence,               
        evidence: vec![format!("LLM analyse: {}", reasoning)],  
    })

}