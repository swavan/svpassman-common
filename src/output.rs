use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Output {
    pub message: String,
    pub meta: std::collections::HashMap<String, String>
}

impl Output {
    pub fn new(message: String) -> Self {
        Self { message, meta: std::collections::HashMap::new() }
    }
    pub fn add_meta(&mut self, key: String, value: String) -> &Self {
        self.meta.insert(key, value);
        self
    }

    pub fn set_meta(&mut self, meta: std::collections::HashMap<String, String>) -> &Self {
        for item in meta.iter() {
            self.meta.insert(item.0.clone(), item.1.clone());
        }
        self
    }
    pub fn account_created() -> Self {
        Output::new( "account successfully created".to_string())
    }
    pub fn saved() -> Self {
        Output::new( "successfully saved".to_string())
    }
    pub fn removed() -> Self {
        Output::new( "successfully removed".to_string())
    }
    pub fn no_action() -> Self {
        Output::new( "No action performed".to_string())
    }
}
