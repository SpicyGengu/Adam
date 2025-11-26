use std::fs;
use regex::Regex;

mod groq_api_handler;
mod twitter_api_handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hard_tweet = 1993315004037382248;

    //let stdin = std::io::stdin();
    let groq_api_key_raw = fs::read_to_string("api_keys/groq_API_key.txt")?;
    let groq_api_key = format!("Bearer {groq_api_key_raw}");
    let groq_client = groq_api_handler::create_client();
    let twitter_client = twitter_api_handler::create_client()?;
    
    //let mut user_input = String::new();
    //stdin.read_line(&mut user_input)?;
    
    let tweet = twitter_api_handler::read_tweet(&twitter_client, hard_tweet)?;

    println!("{tweet}");
    let filtered_tweet = filter_read_tweet(tweet)?;

    println!("{filtered_tweet}");

    let raw_response = groq_api_handler::request(filtered_tweet, groq_client, groq_api_key)?;
    let response = filter_ai_response(raw_response)?;
    
    println!("{response}");

    twitter_api_handler::post_tweet(&twitter_client, response, true, Some(hard_tweet))?;

    Ok(())

    // Testing the post functionality
    /* 
    match twitter_api_handler::post_tweet(twitter_client, "Hello Twitter!".to_string()) {
        Ok(_) => {
            print!("succsess posting!!!");
        }
        Err(error) => {
            panic!("error: {error}");
        }
    }
    */
}

// The string filtering functions rely heavily on REGEX so pull a cheatsheet out for this one
fn filter_ai_response(api_response: String) -> Result<String, Box<dyn std::error::Error>> {
    let regex = Regex::new("\"content\":\"(?<middle>.*?)\"").unwrap();
    let Some(caps) = regex.captures(&api_response) else { 
        return Err("groq API response filtering failed".into()) 
    };
    Ok(caps[1].to_string())
}

// The string filtering functions rely heavily on REGEX so pull a cheatsheet out for this one
fn filter_read_tweet(tweet_text: String) -> Result<String, Box<dyn std::error::Error>> {
    let regex = Regex::new(r"(.*?)(?:https?:|$)").unwrap();
    let Some(caps) = regex.captures(&tweet_text) else { 
        return Err("No content found in tweet".into());
    };
    Ok(caps[1].trim().to_string())
}
