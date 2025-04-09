use actix_web::{web, App, HttpServer, HttpResponse};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use dotenv::dotenv;  // Import dotenv for environment variable loading

#[derive(Serialize)]
struct NewsItem {
    title: String,
    description: Option<String>,
    url: String,
}

async fn get_news(crypto: web::Path<String>) -> HttpResponse {
    dotenv().ok();  // Load environment variables from .env file

    // Retrieve the API key from environment variables
    let api_key = env::var("NEWS_DATA_API_KEY").expect("API key not found in .env file");

    // Build the URL using the crypto parameter (like BTC)
    let url = format!(
        "https://newsdata.io/api/1/news?apikey={}&q={}&language=en",
        api_key, crypto
    );

    // Send GET request to the API
    match reqwest::get(&url).await {
        Ok(response) => {
            match response.json::<Value>().await {
                Ok(data) => {
                    // Log the raw API response for debugging
                    println!("Raw API Response: {:?}", data);

                    let articles = data["results"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|article| {
                            let title = article["title"].as_str().unwrap_or("No Title").to_string();
                            let description = article["description"]
                                .as_str()
                                .map(|d| d.to_string())
                                .unwrap_or_else(|| "No Description".to_string());
                            let url = article["link"].as_str().unwrap_or("").to_string();

                            NewsItem {
                                title,
                                description: Some(description),
                                url,
                            }
                        })
                        .collect::<Vec<NewsItem>>();

                    // Return the data in a JSON response
                    HttpResponse::Ok().json(articles)
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to parse news response."),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch news."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/news/{crypto}", web::get().to(get_news))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
