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
    pub date_time: horae::Utc,
}

impl JournalEntry {
    pub fn new(settings: &Settings) -> Self {
        let (date, title) = {
            if let Some(timezone) = settings.timezone.timezone {
                let (date_time, date) = {
                    let mut date_time = horae::Utc::now();
                    date_time.with_timezone(timezone);
                    (date_time, date_time.date().to_string())
                };
                (date_time, date)
            } else {
                (horae::Utc::now(), horae::Utc::now().date().to_string())
            }
        };
        Self {
            title,
            text: String::new(),
            date_time: date,
        }
    }
}
