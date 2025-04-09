```markdown
# Crypto News Aggregator

This is a simple backend application built with **Rust** and **Actix-web** that fetches the latest cryptocurrency news from an external API and serves it via an HTTP server. The app uses the **NewsData.io** API to retrieve news articles about specific cryptocurrencies, such as Bitcoin (BTC).

## Features

- Fetch the latest news for specific cryptocurrencies (e.g., Bitcoin).
- Serve the news in JSON format via a RESTful API.
- Configured to use an API key stored in a `.env` file for secure access to external data.

## Requirements

- **Rust** (preferably the latest stable version)
- **NewsData.io API Key** (sign up at [NewsData.io](https://newsdata.io/) to get your API key)
- **dotenv** crate for environment variable management

## Installation

### Clone the repository:

```bash
git clone https://github.com/yourusername/crypto-news-aggregator.git
cd crypto-news-aggregator
```

### Set up dependencies:

Make sure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/). Then run the following command to install required dependencies:

```bash
cargo build
```

### Create a `.env` file:

Create a `.env` file in the root of your project directory with the following content:

```env
NEWS_DATA_API_KEY=your_api_key_here
```

Replace `your_api_key_here` with the actual API key you obtained from [NewsData.io](https://newsdata.io/).

### Run the server:

Start the server by running:

```bash
cargo run
```

The server will be running on `http://localhost:8080`.

### Example Request:

To get the latest news for Bitcoin (BTC), visit:

```
http://localhost:8080/news/BTC
```

This will return a list of news articles in JSON format about Bitcoin.

### Example Response:

```json
[
    {
        "title": "Bitcoin Price Hits New High",
        "description": "Bitcoin has reached a new all-time high price.",
        "publishedAt": "2025-04-09T10:00:00Z",
        "source": "Crypto News",
        "url": "https://cryptonews.com/bitcoin-price"
    },
    {
        "title": "Bitcoin Adoption Growing Worldwide",
        "description": "Bitcoin adoption continues to grow globally as more businesses accept it.",
        "publishedAt": "2025-04-09T09:30:00Z",
        "source": "Blockchain News",
        "url": "https://blockchainnews.com/bitcoin-adoption"
    }
]
```

## Project Structure

- **src/main.rs** - The main entry point of the application. It sets up the Actix-web server and routes for fetching cryptocurrency news.
- **src/services/newsdata.rs** - Contains the logic for interacting with the NewsData.io API.
- **.env** - Stores sensitive environment variables such as the API key.

## Troubleshooting

### 1. API key not found:
Make sure your `.env` file is correctly set up with the `NEWS_DATA_API_KEY` and located in the root of your project.

### 2. Failed to fetch news:
Ensure that the API key is valid and has proper permissions. You can check the status of NewsData.io API at [NewsData.io status page](https://newsdata.io/status).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For more information or any issues, feel free to contact me at [your_email@example.com].
```

---

This `README.md` covers:

- **Project Overview**
- **Features**
- **Installation Instructions**
- **API Usage**
- **Example Response**
- **Project Structure**
- **Troubleshooting**
- **License and Contact Information**

Feel free to adjust the content, such as your email or any other specific details related to your project.
