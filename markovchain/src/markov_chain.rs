
use std::hash::Hash;
use std::collections::HashMap;

use indexmap::IndexSet;

use rand::prelude::*;


#[derive(Debug)]
pub struct GenericMarkovChain <T: Eq + Hash + Clone>{
    order: i32,
    starter: IndexSet<Vec<T>>,
    markov_body: HashMap<Vec<T>, ProbabilityDistribution<T>>,
}

impl<T: Eq + Hash + Clone> GenericMarkovChain<T> {

    pub fn new(order: i32)-> GenericMarkovChain<T> {
         GenericMarkovChain { order: order, starter: IndexSet::new(), markov_body: HashMap::new() }
    }

    pub fn add(&mut self, tokens: &Vec<T>) where T: Eq + Hash + Clone{
        if !tokens.is_empty(){
            
            let mut key: Vec<T> = Vec::new();
            if tokens.len() <= self.order as usize  {
                for e in tokens{
                    key.push(e.clone());
                }
                self.starter.insert(key);
            } else {
                let mut index = 0;
                let mut index2;
                let mut count;
                while index < tokens.len() {
                    key.push(tokens[index].clone());
                    count = 1;
                    index2 = index+1;
                    while index2 < tokens.len() && count < self.order{
                        key.push(tokens[index2].clone());
                        index2 +=1;
                        count+=1;
                    }
                    if index2 < tokens.len() {
                        let prob = self.markov_body.entry(key.clone()).or_insert(ProbabilityDistribution::new());
                        prob.add(&tokens[index2]);
                    }
                    if index == 0 {
                        self.starter.insert(key.clone());
                    }
                    index+=1;
                    key.clear();
                }
            }
            
        }
        
    }

    pub fn generate(&mut self, max_words: i32) -> Vec<T> {
        let mut rng = thread_rng();
        let mut res : Vec<T> = Vec::new();
        //pick start word
        let start = self.starter.get_index(rng.gen_range(0, self.starter.len()));
        if let Some(x) = start {
            
            for token in x {
                res.push(token.clone());   
            }
            let mut count = res.len();

            while count < max_words as usize{
                    let index = if res.len() > self.order as usize {res.len()-self.order as usize} else {0};
                    let key = res[index .. res.len()].to_vec();
                    if let Some(prob) = self.markov_body.get(&key){
                        if let Some(next) = prob.pick(&mut rng){
                            res.push(next.clone());
                            count+=1;
                            
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }  
            } 
        }
        res
        
    }
}

#[derive(Debug)]
struct ProbabilityDistribution<T: Eq + Hash + Clone> {
    records: HashMap<T, i32>,
    total: i32,
}

impl<T: Eq + Hash + Clone> ProbabilityDistribution<T> {
    fn new() -> ProbabilityDistribution<T> where T: Eq + Hash + Clone {
        ProbabilityDistribution { records: HashMap::new(), total: 0}
    }

    fn add(&mut self, s: &T) where T: Eq + Hash + Clone{
        
        let c = self.records.entry(s.clone()).or_insert(0);
        *c += 1;
        self.total +=1;
    }

    fn pick(&self, rng: &mut ThreadRng)-> Option<&T> where T: Eq + Hash + Clone{
        let x = rng.gen_range(0, self.total);
        let mut index = 0;
        let mut count;
        for (k, v) in &self.records {
            count = v;
            if x < index + count {
               return Some(k);
            }
            index += count;
        }
        None
    }
}