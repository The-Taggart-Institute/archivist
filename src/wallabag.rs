use reqwest::{Client, Response, StatusCode};
use serde::Deserialize;
use serde_json::json;
use url::Url;
use crate::config::Config;

/// Struct for handling token responses
#[derive(Deserialize)]
struct OAuthResponse {
    access_token: String,
    // expires_in: usize,
    // refresh_token: String,
    // scope: Option<String>,
    // token_type: String,
}

/// Retrieve OAuth Token
async fn get_token(config: &Config) -> Result<String, String> {
    let mut url = String::from(config.wallabag_url.clone());
    url.push_str("oauth/v2/token");
    let client = Client::new();
    let params = json!(
        {
            "grant_type": "password",
            "client_id": config.client_id,
            "client_secret": config.client_secret,
            "username": config.wallabag_username,
            "password": config.wallabag_password
        }
    );

    let res = client.post(url).json(&params).send().await.unwrap();
    match res.status() {
        StatusCode::OK => {
            let json = res.json::<OAuthResponse>().await.unwrap();
            Ok(json.access_token)
        }
        _ => Err("No token found!".to_string()),
    }
}

/// Archive
pub async fn add_article(url: &str, tags: &str, config: &Config) -> Result<Response, String> {

    match Url::parse(url) {
        Ok(u) => {

            if u.scheme() == "http" || u.scheme() == "https" {
                match get_token(config).await {
                    Ok(token) => {
                        let mut add_url = String::from(config.wallabag_url.clone());
                        add_url.push_str("api/entries");
                        let client = reqwest::Client::new();
                        let params = json!({"url": url, "tags": tags});
                        let res = client
                            .post(add_url)
                            .bearer_auth(token)
                            .json(&params)
                            .send()
                            .await
                            .unwrap();
            
                        match res.status() {
                            StatusCode::OK => Ok(res),
                            _ => Err(res.text().await.unwrap()),
                        }
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err("Invalid scheme!".to_string())
            }
        },
        Err(e) => Err(format!("{e}"))
    }

}

#[tokio::test]
async fn can_create_post() {
    use crate::config::load_config;
    let config = load_config().unwrap();

    let res = add_article("https://taggart-tech.com/ai-llms/", "foo,bar,baz", &config).await;
    println!("{:?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn stop_javascript_url() {
    use crate::config::load_config;
    let config = load_config().unwrap();

    let res = add_article("javascript:alert('XSS')", "foo,bar,baz", &config).await;
    println!("{:?}", res);
    assert!(res.is_err())
}

#[tokio::test]
async fn stop_bogus_url() {
    use crate::config::load_config;
    let config = load_config().unwrap();

    let res = add_article("https://'><h1>Test</h1>", "foo,bar,baz", &config).await;
    println!("{:?}", res);
    assert!(res.is_err())
}

#[tokio::test]
async fn can_retrieve_token() {
    use crate::config::load_config;
    let config = load_config().unwrap();
    let token = get_token(&config).await;
    println!("{:?}", token);
    assert!(token.is_ok())
}


