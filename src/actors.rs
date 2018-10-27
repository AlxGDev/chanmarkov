use actix::{Actor, Handler, MessageResult, Context, AsyncContext, fut, fut::ActorFuture};

use std::time::Duration;

use regex::Regex;
use futures::Future;
use futures::future::join_all;

use markovchain::markov_chain::GenericMarkovChain;
use errors::FetchError;
use models::ChanPage;
use models::Thread;
use httputil;
use messages::MarkovGenerate;



pub struct MarkovActor {
    markov_chain: GenericMarkovChain<String>,
    board: String,
    refresh_rate: u64,
}


impl MarkovActor {

    pub fn new(board: String, order: i32, refresh_rate: u64)-> MarkovActor {
         MarkovActor { markov_chain: GenericMarkovChain::new(order), board: board, refresh_rate: refresh_rate }
    }

    fn get_new_chain(&self, ctx: &mut Context<Self>) {
       
        let markov_chain: GenericMarkovChain<String> = GenericMarkovChain::new(2);
        let board_clone = self.board.clone();
        ctx.run_later(Duration::new(self.refresh_rate, 0), |_act, ctx| {
            println!("Updating chain");
            let wrapped = fut::wrap_future::<_, Self>(
               fill_markov(board_clone, markov_chain)
              ).then(|result, actor, ctx_|{
                    match result {
                        Ok(e) => actor.markov_chain = e,
                        Err(e) => match e {
                            //FetchError::Http(e) => eprintln!("http error: {}", e),
                            FetchError::Json(e) => eprintln!("json parsing error: {}", e),
                            FetchError::Other(e) => eprintln!("other error: {}", e),
                        },
                    }
                    actor.get_new_chain(ctx_);
                    fut::ok::<_, _,_>(())
              }); 
            ctx.spawn(
               wrapped
            );

        });
    }
}

impl Actor for MarkovActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
       
        println!("MarkovActor started!");
        self.get_new_chain(ctx);

    }
}

impl Handler<MarkovGenerate> for MarkovActor {
    type Result = MessageResult<MarkovGenerate>;   // <- Message response type

    fn handle(&mut self, msg: MarkovGenerate, ctx: &mut Context<Self>) -> Self::Result {
        MessageResult(self.markov_chain.generate(msg.max_words))
    }
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

fn fill_markov(board: String, mut markov_chain: GenericMarkovChain<String>) -> impl Future<Item=GenericMarkovChain<String>, Error=FetchError> {

    fetch_threads(&board)
        .or_else(|e| {
            match e {
                //FetchError::Http(e) => eprintln!("http error during page retrieval: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error during page retrieval: {}", e),
                FetchError::Other(e) => eprintln!("other error during page retrieval: {}", e),
            }
            let fake_pages: Vec<ChanPage> = Vec::new();
            Ok(fake_pages)
        })
        .and_then(move |pages| {
            let threads :Vec<i32> =pages.iter().flat_map(|s| s.threads.iter()).map(|t| t.no).collect();
            let mut futures = Vec::new();
            threads.iter().for_each(|i| {
                futures.push(
                    fetch_thread(&board, *i)
                    .or_else(|e| {
                        match e {
                            //FetchError::Http(e) => eprintln!("http error: {}", e),
                            FetchError::Json(e) => eprintln!("json parsing error: {}", e),
                            FetchError::Other(e) => eprintln!("other error: {}", e),
                        }
                        let fake_result: Thread = Thread{posts: Vec::new()};
                        Ok(fake_result)
                    })
                );
            });
            join_all(futures)
        })
        .map(move |result|{

            result.iter().for_each(|t|{
                let comments :Vec<String> = t.posts.iter()   
                                            .map(|post| html_unescape(&post.com))
                                            .map(|s| clean_post(&s))
                                            .filter(|s| !s.is_empty()).collect();
                //println!("Got comments: {}",comments.len());
                comments.iter()
                .flat_map(|s| s.split("\n"))
                .for_each(|s| {
                    let tokens: &Vec<String> = &s.split_whitespace()
                                .filter(|&s| !s.is_empty())
                                .map(|s| s.to_string())
                                .collect();
                    markov_chain.add(tokens)
                } 
                );
            });
            println!("Finished Updating chain");
            markov_chain
        })
        
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
        index += 1;
    }
    
    result               
 
}

fn fetch_threads(board: &str) -> impl Future<Item=Vec<ChanPage>, Error=FetchError> {
    let url = str::replace("http://a.4cdn.org/?/threads.json", "?", board);//.parse().unwrap();
    httputil::fetch_json_actix::<Vec<ChanPage>>(url)
  
}


fn fetch_thread(board: &str, thread: i32) -> impl Future<Item=Thread, Error=FetchError> {
    println!("Fetching thread: {}",thread);
    let url = "http://a.4cdn.org/?/thread/#.json".replace("?", board).replace("#", thread.to_string().as_str());//.parse().unwrap();
    httputil::fetch_json_actix::<Thread>(url)
    
}
