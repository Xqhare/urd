use std::collections::VecDeque;

use crate::settings::Settings;

#[derive(Clone, Debug)]
pub struct Journal {
    pub entries: VecDeque<JournalEntry>,
    pub current_entry: JournalEntry,
}

impl Journal {
    pub fn new(settings: &Settings) -> Self {
        Self {
            entries: VecDeque::new(),
            current_entry: JournalEntry::new(settings),
        }
    }
}

#[derive(Clone, Debug)]
pub struct JournalEntry {
    pub title: String,
    pub text: String,
    pub date_time: String,
}

impl JournalEntry {
    pub fn new(settings: &Settings) -> Self {
        let (date_time, date) = {
            if let Some(timezone) = settings.timezone {
                let (date_time, date) = {
                    let mut date_time = horae::Utc::now();
                    date_time.with_timezone(timezone);
                    (date_time.to_string(), date_time.date().to_string())
                };
                (date_time, date)
            } else {
                (horae::Utc::now().to_string(), horae::Utc::now().date().to_string())
            }
        };
        Self {
            title: date,
            text: String::new(),
            date_time,
        }
    }
}
