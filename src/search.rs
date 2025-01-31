use crate::journal_entries::JournalEntry;

#[derive(Default)]
pub struct Search {
    pub query: String,
    pub results: Vec<JournalEntry>,
}

