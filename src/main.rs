use std::fs;
use regex::Regex;

mod groq_api_handler;
mod twitter_api_handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hard_tweet: u64 = 1994045362936664255;
    let queries = [
        "woke agenda".to_string(),
        "women back kitchen".to_string(),
        "redpill".to_string()];

    let twitterless = true; // use this for faster code testing, the free version of twitter api allows one tweet posted and read every 15 min, ain't nobody got time for that
    let groq_api_key_raw = fs::read_to_string("api_keys/groq_API_key.txt")?;
    let groq_api_key = format!("Bearer {groq_api_key_raw}");
    let groq_client = groq_api_handler::create_client();
    let twitter_client = twitter_api_handler::create_client()?;
    
    let potential_tweets = if !twitterless {
        Some(twitter_api_handler::search_for_tweets(&twitter_client, queries[0].clone())?)
    } else {
        None
    };

    let tweet = if twitterless {
        "I'm saying women should get back in the kitchen, and men should be the kind of men that allow them to do so. The current state of the world is due to demanding women and weak ass men that say yes to everything.".to_string()
    } else {
        potential_tweets.unwrap()[0].1.clone()
    };

    println!("{tweet}");
    let filtered_tweet = filter_read_tweet(tweet)?;

    println!("{filtered_tweet}");

    let raw_response = groq_api_handler::request(filtered_tweet, groq_client, groq_api_key)?;
    let response = filter_ai_response(raw_response)?;
    
    println!("{response}");

    if !twitterless {
        twitter_api_handler::post_tweet(&twitter_client, response, true, Some(hard_tweet))?;
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
