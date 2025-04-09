use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use actix_web::{web};

#[derive(Deserialize, Serialize)]
pub struct NewsResponse {
    pub status: String,
    pub results: Vec<NewsItem>,
}

#[derive(Deserialize, Serialize)]
pub struct NewsItem {  
    pub title: String,
    // Если поле `source` отсутствует, можно его убрать
    // или заменить на другое подходящее поле
    pub pub_date: String,  
    pub description: String,
    pub link: String,
}


pub async fn get_news(crypto: web::Path<String>) -> web::Json<Vec<NewsItem>> {
    let client = Client::new();
    let api_key = env::var("CRYPTO_NEWS_API_KEY").expect("API key not set");

    let url = format!(
        "https://newsdata.io/api/1/news?apikey={}&q={}&language=en&category=business",
        api_key, crypto
    );

    let res = client.get(&url).send().await;

    match res {
        Ok(response) => {
            let body = response.text().await.unwrap();
            println!("{}", body);  // Для отладки
            match serde_json::from_str::<NewsResponse>(&body) {
                Ok(news_response) => web::Json(news_response.results),
                Err(e) => {
                    eprintln!("Error deserializing response: {:?}", e);
                    web::Json(vec![])  // Возвращаем пустой список в случае ошибки
                }
            }
        },
        Err(e) => {
            eprintln!("Error fetching data from API: {:?}", e);
            web::Json(vec![])  // Возвращаем пустой список в случае ошибки
        }
    }
}


