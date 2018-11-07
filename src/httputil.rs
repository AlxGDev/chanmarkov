
use serde;
use serde_json;
use futures::Future;
use futures::Stream;

use hyper::Client;
use hyper_tls::HttpsConnector;
use errors::FetchError;
use std::time::Duration;

use actix_web::{client, HttpMessage};

pub fn fetch_json_hyper<T: serde::de::DeserializeOwned>(uri: hyper::Uri) -> impl Future<Item=T, Error=FetchError> {
    
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);
    client.get(uri)
        .and_then(|res| {
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            let json_result: T = serde_json::from_slice(&body)?; 
            Ok(json_result)
        })
        .from_err()
    
        
}

pub fn fetch_json_actix<T: serde::de::DeserializeOwned>(url: String) -> impl Future<Item=T, Error=FetchError> {
    client::get(url)
        .timeout(Duration::new(10, 0))  
        .finish()
        .unwrap()
        .send()
        .from_err::<FetchError>()             
        .and_then(|response| {                
            response.payload().concat2()
            .from_err::<FetchError>()  
        })
        .from_err::<FetchError>() 
        .and_then(|body| {
            let json_result: T = serde_json::from_slice(&body)?;   
            Ok(json_result)
        })
        .from_err()
    

        
}
