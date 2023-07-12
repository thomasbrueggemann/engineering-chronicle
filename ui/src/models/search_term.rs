use serde::{Serialize, Deserialize};
use slug::slugify;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SearchTerm {
    pub id: String,
    pub search_term: String
}

impl SearchTerm {
    pub fn new(search_term: &str) -> SearchTerm {
        SearchTerm { 
            id: slugify(search_term), 
            search_term: search_term.to_string()
        }
    }
}