
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await; 
    println!("Hello, world!");
    Ok(())
}
