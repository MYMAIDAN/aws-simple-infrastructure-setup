use std::println;

use aws_sdk_secretsmanager::{config::Region, meta::PKG_VERSION, Client, Error, types::error::InternalServiceError};

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize, Debug)]
pub struct RDSCreds{
    pub username: String,
    pub password: String,
    // engine: String,
//     host: String,
//     port: u32,
//     dbInstanceIdentifier: String
 }






pub async fn get_secret_value(client: &Client, key: &str) -> Result<RDSCreds, Error >{
    let rsp = client.get_secret_value().secret_id(key).send().await?;


    let Some(value) = rsp.secret_string() else {
        return Err(Error::InternalServiceError(InternalServiceError::builder().build()))
    };


    let rds: RDSCreds = match serde_json::from_str(value) {
        Ok(value) => value,
        Err(e) => {
            println!("Error:{:?}", e);
            return Err(Error::InternalServiceError(InternalServiceError::builder().build()))
        }
    };


    println!("Value:{:?}", rds);

    Ok(rds)
}

