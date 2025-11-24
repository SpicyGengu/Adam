use std::error::Error;
use serde::Deserialize;
use twitter_v2::TwitterApi;
use twitter_v2::authorization::Oauth1aToken;
use twitter_v2::prelude::IntoNumericId;


#[derive(Debug, Deserialize)]
struct TwitterAuth {
    api_key: String,
    api_secret: String,
    access_token: String,
    access_token_secret: String,
}

pub fn create_client() -> Result<TwitterApi<Oauth1aToken>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("api_keys/twitter_auth.csv")?;
    
    for result in rdr.deserialize() {
        let auth_data: TwitterAuth = result?;
        let auth = Oauth1aToken::new(
            &auth_data.api_key,
            &auth_data.api_secret,
            &auth_data.access_token,
            &auth_data.access_token_secret,
        );
        return Ok(TwitterApi::new(auth));
    }
    
    Err("No authentication data found in CSV".into())
}

#[tokio::main]
pub async fn post_tweet(client: &TwitterApi<Oauth1aToken>, tweet_contents: String, reply: bool, reply_id: Option<impl IntoNumericId>) -> Result<(), Box<dyn Error>> {
    match reply {
        true => {
            client
                .post_tweet()
                .text(tweet_contents)
                .in_reply_to_tweet_id(reply_id.unwrap())
                .send()
                .await?;
        }
        false => {
            client
                .post_tweet()
                .text(tweet_contents)
                .send()
                .await?;
        }
    }

    // Debuging if needed
    /*
    let tweet = client
        .post_tweet()
        .text(tweet_contents)
        .send()
        .await?;
    match &tweet.data {
        Some(tweet_data) => {
            println!("Posted tweet: {}", tweet_data.id);
            println!("Tweet text: {}", tweet_data.text);
        }
        None => {
            return Err("Tweet was posted but no data returned from Twitter API".into());
        }
    }
    */

    Ok(())
}

#[tokio::main]
pub async fn read_tweet(client: &TwitterApi<Oauth1aToken>, tweet_id: impl IntoNumericId) -> Result<String, Box<dyn Error>> {
    let tweet = client
        .get_tweet(tweet_id)
        .send()
        .await?;
    match &tweet.data {
        Some(tweet_data) => Ok(tweet_data.text.to_string()),
        None => Err("Tweet not found or no data returned".into()),
    }
}







// example
/*
use twitter_v2::TwitterApi;
use twitter_v2::authorization::Oauth1aToken;

// This ONE client can do both reading AND writing
fn setup_twitter_client() -> TwitterApi<Oauth1aToken> {
    let auth = Oauth1aToken::new(
        "your_api_key",
        "your_api_secret", 
        "your_access_token",
        "your_access_token_secret",
    );
    TwitterApi::new(auth)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let twitter = setup_twitter_client();
    
    // WRITE - Post a tweet
    let tweet = twitter
        .post_tweet()
        .text("Hello from Rust! 🦀")
        .send()
        .await?;
    println!("Posted tweet: {}", tweet.data.id);
    
    // READ - Search for tweets
    let search_results = twitter
        .get_tweets_search_recent()
        .query("rust programming")
        .max_results(5)
        .send()
        .await?;
    
    if let Some(tweets) = search_results.data {
        for tweet in tweets {
            println!("Found tweet: {}", tweet.text);
        }
    }
    
    Ok(())
}
*/
