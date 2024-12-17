use std::collections::{BTreeMap, VecDeque};

use nabu::{Object, XffValue};

use crate::settings::Settings;


#[derive(Clone, Debug)]
pub enum EntryType {
    JournalEntry(JournalEntry),
    Folder(Folder),
}

impl EntryType {
    pub fn is_folder(&self) -> bool {
        match self {
            EntryType::JournalEntry(_) => false,
            EntryType::Folder(_) => true,
        }
    }

    pub fn is_journal_entry(&self) -> bool {
        match self {
            EntryType::JournalEntry(_) => true,
            EntryType::Folder(_) => false,
        }
    }

    pub fn get_folder_mut(&mut self) -> Option<&mut Folder> {
        match self {
            EntryType::JournalEntry(_) => None,
            EntryType::Folder(f) => Some(f),
        }
    }

    pub fn get_journal_entry_mut(&mut self) -> Option<&mut JournalEntry> {
        match self {
            EntryType::JournalEntry(e) => Some(e),
            EntryType::Folder(_) => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Folder {
    pub name: String,
    pub entries: VecDeque<EntryType>,
}

impl Folder {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            entries: VecDeque::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Journal {
    pub entries: VecDeque<EntryType>,
    pub current_entry: JournalEntry,
}

impl Journal {
    /// Think of it like default, but needs a settings object
    pub fn new(settings: &Settings) -> Self {
        let (year, month) = {
            let mut date_time = horae::Utc::now();
            date_time.with_timezone(settings.timezone.timezone);
            (date_time.date().year, date_time.date().month)
        };
        let month_folder = Folder::new(month.to_string());
        let mut year_folder = Folder::new(year.to_string());
        year_folder.entries.push_back(EntryType::Folder(month_folder));
        let entries = VecDeque::from([EntryType::Folder(year_folder)]);
        Self {
            entries,
            current_entry: JournalEntry::new(&Settings::default()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct JournalEntry {
    pub title: String,
    pub text: String,
    /// Stores:
    /// - Context tags
    /// - Project tags
    /// - Special tags
    /// - Bespoke tags / User defined tags
    /// - Full date split up [year, month, day]
    pub metadata: BTreeMap<String, XffValue>,
}

impl JournalEntry {
    pub fn new(settings: &Settings) -> Self {
        let (title, full_date_split) = {
            let mut date_time = horae::Utc::now();
            date_time.with_timezone(settings.timezone.timezone);
            let date_split = {
                let mut out = Object::new();
                out.insert("year", date_time.date().year);
                out.insert("month", date_time.date().month);
                out.insert("day", date_time.date().day);
                out
            };
            (date_time.date().to_string(), date_split)
        };
        let metadata = {
            let mut out = BTreeMap::new();
            out.insert("date".to_string(), XffValue::from(full_date_split));
            out
        };
        Self {
            title,
            text: String::new(),
            metadata,
        }
    }
}
