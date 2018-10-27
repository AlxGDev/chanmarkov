//extern crate hyper;
//extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate actix_web;
use futures::Future;
use futures::Stream;

//use hyper::Client;
//use hyper_tls::HttpsConnector;
use errors::FetchError;
use std::time::Duration;

use actix_web::{client, HttpMessage};


/*pub fn fetch_json_hyper<T: serde::de::DeserializeOwned>(uri: hyper::Uri) -> impl Future<Item=T, Error=FetchError> {
    
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);
    let get = client
                // Fetch the url...
            .get(uri)
                // And then, if we get a response back...
            .and_then(|res| {
                    // asynchronously concatenate chunks of the body
                res.into_body().concat2()
            })
            .from_err::<FetchError>()
                // use the body after concatenation
            .and_then(|body| {
                    // try to parse as json with serde_json
                let thing: T = serde_json::from_slice(&body)?;
                    
                Ok(thing)
            })
            .from_err();
    get
        
}*/

pub fn fetch_json_actix<T: serde::de::DeserializeOwned>(url: String) -> impl Future<Item=T, Error=FetchError> {
    let get = client::get(url)
            .timeout(Duration::new(10, 0))  // <- Create request builder
            .finish()
            .unwrap()
            .send()
            .from_err::<FetchError>()              // <- Send http request
            .and_then(|response| {                // <- server http response
                response.payload().concat2()
                .from_err::<FetchError>()  
            })
            .from_err::<FetchError>() 
            .and_then(|body| {
                    // try to parse as json with serde_json
                let thing: T = serde_json::from_slice(&body)?;
                    
                Ok(thing)
            })
            .from_err();
    get

        
}
