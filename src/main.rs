
#[macro_use] 
extern crate text_io;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use] 
extern crate lazy_static;
extern crate regex;


extern crate hyper;
extern crate hyper_tls;

extern crate markovchain;

use hyper::Client;
use hyper_tls::HttpsConnector;
use hyper::rt::{self, Future, Stream};
use regex::Regex;

use std::sync::{Arc, RwLock};

use markovchain::markov_chain::GenericMarkovChain;

fn main() {
    println!("Enter the board:");
    let board: String = read!("{}");
    println!("Enter the thread:");
    let thread: String = read!("{}");
    let thread_i = thread.parse::<i32>().unwrap();

    let markov_chain: Arc<RwLock<GenericMarkovChain<String>>> = Arc::new(RwLock::new(GenericMarkovChain::new(2)));
    /*let fut = fetch_threads("d")
        // use the parsed vector
        .map(|users| {
            
            // print ids
            let threads :Vec<i32> = users.iter().flat_map(|s| s.threads.iter()).map(|t| t.no).collect();
            println!("Thread ids: {:#?}", threads);
           
        })
        // if there was an error print it
        .map_err(|e| {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            }
        }); */

   
    let fut = fetch_thread(&board, &thread_i)
        // use the parsed vector
        .map(move |posts| {
            
            
            let comments :Vec<String> = posts.into_iter()
                                        .map(|post| html_unescape(&post.com))
                                        .map(|s| clean_post(&s))
                                        .filter(|s| !s.is_empty()).collect();
            // println!("Comments: {:#?}", comments);
            comments.iter()
            .flat_map(|s| s.split("\n"))
            .for_each(|s| markov_chain.write().unwrap()
                        .add(&s.split_whitespace()
                            .filter(|&s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect()
                            )
            );
            
            println!("MarkovChain: {:#?}", markov_chain.write().unwrap().generate(20));
                
           
        })
        // if there was an error print it
        .map_err(|e| {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            }
        });
    

    // Run the runtime with the future trying to fetch, parse and print json.
    rt::run(fut);
    
}

fn html_unescape(input: &str) -> String{
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut unescaped_string = String::new();
    let mut mode = 0;
    for c in chars{
        if mode == 0 && c == '&' {
            mode = 1;
            unescaped_string.push(c);
        } else if mode == 1 && (c == ';' || c == ' ' || c == '\n') {
            unescaped_string.push(c);
            mode = 0;
            match unescaped_string.as_ref() {
                "&#032;" => result.push(' '),
                "&#033;" => result.push('!'),
                "&#034;" => result.push('"'),
                "&#035;" => result.push('#'),
                "&#036;" => result.push('$'),
                "&#037;" => result.push('%'),
                "&#038;" => result.push('&'),
                "&#039;" => result.push('\''),
                "&#040;" => result.push('('),
                "&#041;" => result.push(')'),
                "&#042;" => result.push('*'),
                "&#043;" => result.push('+'),
                "&#044;" => result.push(','),
                "&#045;" => result.push('-'),
                "&#046;" => result.push('.'),
                "&#047;" => result.push('/'),
                "&#048;" => result.push('0'),
                "&#049;" => result.push('1'),
                "&#050;" => result.push('2'),
                "&#051;" => result.push('3'),
                "&#052;" => result.push('4'),
                "&#053;" => result.push('5'),
                "&#054;" => result.push('6'),
                "&#055;" => result.push('7'),
                "&#056;" => result.push('8'),
                "&#057;" => result.push('9'),
                "&#058;" => result.push(':'),
                "&#059;" => result.push(';'),
                "&#060;" => result.push('<'),
                "&#061;" => result.push('='),
                "&#062;" => result.push('>'),
                "&#063;" => result.push('?'),
                "&#064;" => result.push('@'),
                "&#065;" => result.push('A'),
                "&#066;" => result.push('B'),
                "&#067;" => result.push('C'),
                "&#068;" => result.push('D'),
                "&#069;" => result.push('E'),
                "&#070;" => result.push('F'),
                "&#071;" => result.push('G'),
                "&#072;" => result.push('H'),
                "&#073;" => result.push('I'),
                "&#074;" => result.push('J'),
                "&#075;" => result.push('K'),
                "&#076;" => result.push('L'),
                "&#077;" => result.push('M'),
                "&#078;" => result.push('N'),
                "&#079;" => result.push('O'),
                "&#080;" => result.push('P'),
                "&#081;" => result.push('Q'),
                "&#082;" => result.push('R'),
                "&#083;" => result.push('S'),
                "&#084;" => result.push('T'),
                "&#085;" => result.push('U'),
                "&#086;" => result.push('V'),
                "&#087;" => result.push('W'),
                "&#088;" => result.push('X'),
                "&#089;" => result.push('Y'),
                "&#090;" => result.push('Z'),
                "&#091;" => result.push('['),
                "&#092;" => result.push('\\'),
                "&#093;" => result.push(']'),
                "&#094;" => result.push('^'),
                "&#095;" => result.push('_'),
                "&#096;" => result.push('`'),
                "&#097;" => result.push('a'),
                "&#098;" => result.push('b'),
                "&#099;" => result.push('c'),
                "&#100;" => result.push('d'),
                "&#101;" => result.push('e'),
                "&#102;" => result.push('f'),
                "&#103;" => result.push('g'),
                "&#104;" => result.push('h'),
                "&#105;" => result.push('i'),
                "&#106;" => result.push('j'),
                "&#107;" => result.push('k'),
                "&#108;" => result.push('l'),
                "&#109;" => result.push('m'),
                "&#110;" => result.push('n'),
                "&#111;" => result.push('o'),
                "&#112;" => result.push('p'),
                "&#113;" => result.push('q'),
                "&#114;" => result.push('r'),
                "&#115;" => result.push('s'),
                "&#116;" => result.push('t'),
                "&#117;" => result.push('u'),
                "&#118;" => result.push('v'),
                "&#119;" => result.push('w'),
                "&#120;" => result.push('x'),
                "&#121;" => result.push('y'),
                "&#122;" => result.push('z'),
                "&#123;" => result.push('{'),
                "&#124;" => result.push('|'),
                "&#125;" => result.push('}'),
                "&#126;" => result.push('~'),
                "&quot;" => result.push('"'),
                "&amp;" => result.push('&'),
                "&lt;" => result.push('<'),
                "&gt;" => result.push('<'),
                _ => result.push_str(&unescaped_string)
            }
            unescaped_string.clear();
        } else if mode == 1 {
            unescaped_string.push(c);
        } else {
            result.push(c);
        }
    }
    if !unescaped_string.is_empty(){
        result.push_str(&unescaped_string);
    }
    result
    
}


//removes html tags and adds line breaks
fn clean_post(post: &str) -> String{
    lazy_static! {
        static ref LINK_RE: Regex = Regex::new(r"<a.*</a>|<span .*?>|</span>").unwrap();
    }
    lazy_static! {
        static ref BREAK_RE1: Regex = Regex::new(r"^<br>+").unwrap();
    }
    lazy_static! {
        static ref BREAK_RE2: Regex = Regex::new(r"<br>+").unwrap();
    }
    
    let chars: Vec<char> = BREAK_RE2
                    .replace_all(&BREAK_RE1
                    .replace_all(&LINK_RE
                    .replace_all(post, "") , ""), "\n").into_owned().chars().collect();
    
    let mut result = String::new();
    
    let mut index = 0;
    while index < chars.len(){
        if index != chars.len()-1 && 
           (chars[index] == '?' || chars[index] == '!' || chars[index] == '.') &&
           chars[index+1] == ' ' {
           result.push(chars[index]);
           result.push('\n');
        } else {
            result.push(chars[index]);
        }
        index = index+1;
    }
    
    result               
 
}

fn fetch_threads(board: &str) -> impl Future<Item=Vec<ChanPage>, Error=FetchError> {
    
    let url = str::replace("http://a.4cdn.org/?/threads.json", "?", board).parse().unwrap();
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);
    let get = client
        // Fetch the url...
        .get(url)
        // And then, if we get a response back...
        .and_then(|res| {
            // asynchronously concatenate chunks of the body
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        // use the body after concatenation
        .and_then(|body| {
            // try to parse as json with serde_json
            let users = serde_json::from_slice(&body)?;
            
            Ok(users)
        })
        .from_err();
    
     return get;   
}

fn fetch_thread(board: &str, thread: &i32) -> impl Future<Item=Vec<Post>, Error=FetchError> {
    
    let url = "http://a.4cdn.org/?/thread/#.json".replace("?", board).replace("#", thread.to_string().as_str()).parse().unwrap();
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);
    let get = client
        // Fetch the url...
        .get(url)
        // And then, if we get a response back...
        .and_then(|res| {
            // asynchronously concatenate chunks of the body
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        // use the body after concatenation
        .and_then(|body| {
            // try to parse as json with serde_json
            let thread: Thread = serde_json::from_slice(&body)?;
            
            Ok(thread.posts)
        })
        .from_err();
    
     return get;   
}



#[derive(Deserialize, Debug)]
struct ChanPage {
    page: i32,
    threads:  Vec<ThreadInfo>,
}

#[derive(Deserialize, Debug)]
struct ThreadInfo{
    no: i32,
}

#[derive(Deserialize, Debug)]
struct Thread{
    posts: Vec<Post>,
}

#[derive(Deserialize, Debug)]
struct Post{
    #[serde(default)]
    com: String,
}

// Define a type so we can return multiple types of errors
enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}


