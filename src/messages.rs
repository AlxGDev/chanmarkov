
pub struct MarkovGenerate{
    pub max_words: i32,
}

impl actix::Message for MarkovGenerate {
    type Result = Vec<String>;
}

pub struct MarkovFeed{
    pub input: String,
}

impl actix::Message for MarkovFeed {
    type Result = ();
}