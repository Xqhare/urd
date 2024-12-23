use crate::journal_entries::JournalEntry;


pub struct Search {
    pub query: String,
    pub results: Vec<JournalEntry>,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
        }
    }
}
