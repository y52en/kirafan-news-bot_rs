use std::env::var;

pub async fn tweet(tweet_str: &str) {
    let api_key = var("twitter_newsbot_api_key_rs").unwrap();
    let api_secret_key = var("twitter_newsbot_api_secret_rs").unwrap();
    let access_token_secret = var("twitter_newsbot_access_token_secret_rs").unwrap();
    let access_token = var("twitter_newsbot_access_token_rs").unwrap();

    let builder = kuon::TwitterAPI::builder()
        .access_token(access_token)
        .access_token_secret(access_token_secret)
        .api_key(api_key)
        .api_secret_key(api_secret_key);
    let api = builder.build().await.unwrap();
    api.tweet().status(tweet_str).send().await.unwrap();
}
