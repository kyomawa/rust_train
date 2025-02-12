use mongodb::{ Client, options::ClientOptions, Database };
use std::{ env, error::Error };

pub async fn init_db() -> Result<Database, Box<dyn Error>> {
    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL variable in the .env is missing.");

    // Parse the MongoDB connection string
    let client_options = ClientOptions::parse(url).await?;

    // Create the MongoDB client
    let client = Client::with_options(client_options)?;

    // Get the database
    Ok(client.database("simple_api"))
}
