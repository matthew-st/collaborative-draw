use reqwest::Client;
use crate::structs::captcha::*;
use serde_urlencoded::ser::to_string;
pub async fn validate(token: String) -> bool {
    let body = Request {
        secret: std::env::var("CAPTCHA_SECRET").unwrap(),
        response: token,
    };
    let response = Client::new()
        .post("https://www.google.com/recaptcha/api/siteverify")
        .body(to_string(body).unwrap())
        .send().await.unwrap().json::<Response>().await.unwrap();

    if response.score < 0.5 {
        return false;
    }
    true
}