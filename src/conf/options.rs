#[derive(Clone, Debug, Default)]
pub struct Options {
    pub paths: Vec<String>,
    pub project: String,
}

impl Options {
    pub fn new() -> Options {
        Options {
            ..Default::default()
        }
    }
}
