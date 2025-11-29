use std::fs;
use regex::Regex;
use rand::prelude::*;

mod groq_api_handler;
mod twitter_api_handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let twitterless = false; // use this for faster code testing, the free version of twitter api allows one tweet posted and read every 15 min, ain't nobody got time for that
    
    let hard_tweet: u64 = 1497155881317904400;
    let queries = [
        "woke agenda".to_string(),
        "women back kitchen".to_string(),
        "redpill".to_string()];
    let mut random_querie_index;
    let mut random_tweet_index;
    let mut rng = rand::rng();

    let groq_api_key_raw = fs::read_to_string("api_keys/groq_API_key.txt")?;
    let groq_api_key = format!("Bearer {groq_api_key_raw}");
    let groq_client = groq_api_handler::create_client();
    let twitter_client = twitter_api_handler::create_client()?;
    
    let mut potential_tweets;
    let mut tweet;
    let mut tweet_id;

    random_querie_index = rng.random_range(0..queries.len());

    if !twitterless {
        potential_tweets = twitter_api_handler::search_for_tweets(&twitter_client, queries[random_querie_index].clone())?;
        random_tweet_index = rng.random_range(0..potential_tweets.len());
        tweet = potential_tweets[random_tweet_index].1.clone();
        tweet_id = potential_tweets[random_tweet_index].0.clone();
    } else {
        tweet = "".to_string();
        tweet_id = 1497155881317904400;
    }

    println!("{tweet}");
    let filtered_tweet = filter_read_tweet(tweet)?;

    println!("{filtered_tweet}");

    let raw_response = groq_api_handler::request(filtered_tweet, groq_client, groq_api_key)?;
    let response = filter_ai_response(raw_response)?;
    
    println!("{response}");

    if !twitterless {
        twitter_api_handler::post_tweet(&twitter_client, response, true, Some(tweet_id))?;
    }

    Ok(())
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
