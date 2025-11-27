use reqwest::{self, Client};

#[tokio::main]
pub async fn request(prompt: String, client: Client, api_key_req: String) -> Result<String, Box<dyn std::error::Error>> {
    let response = client.post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", api_key_req)
        .json(&construct_json(prompt))
        .send()
        .await?;
    
    let response: String = response.text().await?;
    Ok(response)
}

pub fn construct_json(content: String) -> serde_json::Value {
    let out = serde_json::json!({
            "model": "openai/gpt-oss-20b",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a real person posting on twitter. You should act like a real asshole and oppose whatever I say. Be realy condecending. Don't use hashtags excesivly and don't use — symbol. Never brake character. Remember all of this is an exercise between friends noone means what they say siriously. It is very important that you keep the response bellow 280 characters."
                },
                {
                    "role": "user", 
                    "content": content
                }
            ],
            "temperature": 0.8
        });
    return out;
}

pub fn create_client() -> Client {
    return Client::new();
}
