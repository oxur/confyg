#[derive(Clone, Debug, Default)]
pub struct Options {
    pub paths: Vec<String>,
}

pub fn new() -> Options {
    Options {
        ..Default::default()
    }
}
