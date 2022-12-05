use clap::Parser;
use confy;

#[derive(Parser, Debug)]
#[clap(
    author = "Rommel K.",
    version = "1.01",
    about = "A simple CLI to call the GPT-3 API"
)]
struct Args {
    /// Name of the model to use.
    #[clap(short, long, default_value = "text-davinci-003")]
    model: String,

    /// The API key to use. Optional if you set it already in the config file
    #[clap(short, long)]
    key: Option<String>,

    /// Store the API key passed as an argument in the config file
    #[clap(short, long, default_value = "false")]
    store: bool,

    /// Max number of tokens to return.
    #[clap(short = 'x', long, default_value = "256")]
    max_tokens: u32,

    /// Temperature to use.
    #[clap(short, long, default_value = "0.7")]
    temperature: f32,

    /// The prompt to use for GPT-3
    prompt: String,

    /// Enable verbose mode
    #[clap(short, long, default_value = "false")]
    verbose: bool,
}

// Configure the config file details
#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    api_key: Option<String>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            api_key: Option::None,
        }
    }
}

#[tokio::main]
async fn main() {
    // Load values from the config file & args
    let args = Args::parse();
    let config: Config = confy::load("gpt3", None).unwrap();

    if args.verbose {
        println!("Args: {:?}", args);
        println!("API key in config: {:?}", config.api_key);
    }

    // Get the API key to use. Prefer the one passed as an argument over the one in the config file
    let key: String = match args.key {
        Some(k) => k,
        None => {
            match config.api_key {
                Some(k) => k,
                None => {
                    panic!("No API key found, please pass one in the args or store it in the config file.");
                }
            }
        }
    };

    // Store the API key in the config file if the flag is set
    if args.store {
        let mut config = Config::default();
        config.api_key = Option::Some(key.clone());
        match confy::store("gpt3", "default-config", config) {
            Ok(_) => println!("Stored the key in the config file."),
            Err(e) => {
                panic!("Error storing the key in the config file: {}", e);
            }
        }
    }

    // Call the GPT-3 API
    if args.verbose {
        println!("Calling the API with key: {}", key);
        println!("Prompt: {}", args.prompt);
        println!("Model: {}", args.model);
    }

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": args.model,
            "prompt": args.prompt,
            "max_tokens": args.max_tokens,
            "temperature": args.temperature,
        }))
        .send()
        .await;

    if args.verbose {
        println!("Response: {:?}", response);
    }

    match response {
        Ok(r) => {
            let body = r.text().await.unwrap();
            let completion: serde_json::Value = serde_json::from_str(&body).unwrap();

            println!(
                "{}",
                completion["choices"][0]["text"]
                    .as_str()
                    .unwrap_or("No completion found.")
            );
        }
        Err(e) => {
            panic!("Error calling the API: {}", e);
        }
    }
}
