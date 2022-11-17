#[derive(Clone, Debug, Default)]
pub struct Options {
    pub paths: Vec<String>,
    pub project: String,
}

impl Options {
    pub fn default() -> Options {
        Options{
            .. Default::default()
        }
    }
}
