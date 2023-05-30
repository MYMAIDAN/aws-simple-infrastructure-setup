#![allow(clippy::result_large_err)]
use std::format;
use std::io::Chain;

mod database;
use axum::routing::get;
use axum::Router;

use aws_sdk_secretsmanager::Client;

use clap::Parser;
mod secret_manager;

#[derive(Debug,Parser)]
struct Opt{
    #[structopt(short,long)]
    region: Option<String>,

    #[structopt(short,long)]
    query: String,

    #[structopt(short,long)]
    cluster_arn: String,

    #[structopt(short,long)]
    secret_arn: String,

    #[structopt(short,long)]
    verbose: bool
}

// async fn query_cluster( client: &Client,
//                         cluster_arn: &str,
//                         query: &str,
//                         secret_arn: &str
//                         ) -> Result<(), Error>{
//     let st = client.execute_sql()
//         .db_cluster_or_instance_arn(cluster_arn)
//         .database("postgres")
//         .sql_statements(query)
//         .aws_secret_store_arn("arn:aws:secretsmanager:us-east-1:757285457443:secret:rds!db-6f7ef3a1-a2e3-497f-a58e-5f2c9a34249a-ccLMOb");
//
//     let result = st.send().await?;
//
//     println!("Result:{:?}", result);
//
//     Ok(())
//

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt::init();

//
//     let region_provider = RegionProviderChain::first_try(region.map(Region::new))
//                                                                           .or_default_provider()
//                                                                           .or_else(Region::new("us-east-1"));
// vj


    println!();

    // if verbose {
    //     println!("RDS data client version: {}", PKG_VERSION);
    //     println!(
    //         "Region:                  {}",
    //         region_provider.region().await.unwrap().as_ref()
    //     );
    //     println!("Cluster ARN:             {}", &cluster_arn);
    //     println!("Secrets ARN:             {}", &secret_arn);
    //     println!("Query:");
    //     println!("  {}", &query);
    //     println!();
    // }
    //
    // let shared_config = aws_config::from_env().load().await;
    // let client = Client::new(&shared_config);
    //
    // query_cluster(&client, &cluster_arn, &query, &secret_arn).await;
    //
    let shared_config = aws_config::from_env().load().await;
    let client = Client::new(&shared_config);

    let cred = secret_manager::get_secret_value(&client,"rds!db-ff7ad625-f456-4c85-84e2-78fbcb247c81").await?;
    let url = format!("postgres://{}:{}@myauroradbinstance.cur5knkivca0.us-east-1.rds.amazonaws.com/",
                      cred.username,cred.password);

    let connection = crate::database::connection::establish_connection(&url);

    let cors = tower_http::cors::CorsLayer::permissive();
    let app  = Router::new()
    .route("/simple_call",get(simple_call_handler))
    .layer(cors);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    println!("Hello, world!");
    Ok(())
}

async fn simple_call_handler() -> axum::Json<Vec<String>> {
    println!("Hello World! call");
    axum::Json(vec!["foo".to_owned(),"bar".to_owned()])
}



