#[derive(Clone, Debug, Default)]
pub struct Options {
    pub paths: Vec<String>,
    pub project: String,
}

pub fn new() -> Options {
    Options {
        ..Default::default()
    }
}
