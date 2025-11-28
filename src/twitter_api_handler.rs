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

#[tokio::main]
pub async fn search_for_tweets(client: &TwitterApi<Oauth1aToken>, query: String) -> Result<Vec<(u64, String)>, Box<dyn Error>> {
    let tweet = client
        .get_tweets_search_recent(query)
        .max_results(10)
        .send()
        .await?;
    match &tweet.data {
        Some(tweet_list) => Ok(
            tweet_list.iter()
                .map(|tweet| (tweet.id.as_u64(), tweet.text.to_string()))
                .collect()
        ),
        None => Err("Tweets not found or no data returned".into()),
    }
}
