#[derive(Clone, Debug, Default)]
pub struct Options {
    pub top_level: String,
    pub sections: Vec<String>,
}

impl Options {
    pub fn new() -> Options {
        Options {
            ..Default::default()
        }
    }
}
