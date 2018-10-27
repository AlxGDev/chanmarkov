
#[macro_use] 
extern crate text_io;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use] 
extern crate lazy_static;
extern crate regex;
extern crate futures;

extern crate env_logger;

extern crate actix;
extern crate actix_web;

extern crate markovchain;

use futures::Future;
use actix::{Addr, Arbiter};

use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, State,
};

mod httputil;
mod errors;
mod models;
mod messages;
mod actors;

use actors::MarkovActor;
use messages::MarkovGenerate;

struct AppState{
    markov_actor: Addr<MarkovActor>,
}

fn generate_markov(state: State<AppState>) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `DbExecutor`
    let req = state.markov_actor.send(MarkovGenerate {max_words: 20,});
        
        req.from_err().and_then(|res| {
            Ok(HttpResponse::Ok().body(res.join(" ")))
        })
        .responder()
}



fn main() {
    println!("Enter the board:");
    let board: String = read!("{}");
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("chanmarkov");


    //let markov_actor = MarkovActor::new(board, 2, 300);
    let addr = Arbiter::start(move |t| MarkovActor::new(board, 2, 60));

    server::new(move || {
        App::with_state(AppState{markov_actor: addr.clone()})
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/api/generate", |r| r.method(http::Method::GET).with(generate_markov))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();


    
}





