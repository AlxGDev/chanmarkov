
#[derive(Deserialize, Debug)]
pub struct ChanPage {
    pub page: i32,
    pub threads:  Vec<ThreadInfo>,
}

#[derive(Deserialize, Debug)]
pub struct ThreadInfo{
    pub no: i32,
}

#[derive(Deserialize, Debug)]
pub struct Thread{
   pub posts: Vec<Post>,
}

#[derive(Deserialize, Debug)]
pub struct Post{
    #[serde(default)]
    pub com: String,
}