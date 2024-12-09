use crate::settings::Settings;

#[derive(Clone)]
pub struct Journal {
    pub entries: Vec<JournalEntry>,
    pub current_entry: JournalEntry,
}

impl Journal {
    pub fn new(settings: &Settings) -> Self {
        Self {
            entries: Vec::new(),
            current_entry: JournalEntry::new(settings),
        }
    }
}

#[derive(Clone)]
pub struct JournalEntry {
    pub title: String,
    pub text: String,
    pub date_time: String,
}

impl JournalEntry {
    pub fn new(settings: &Settings) -> Self {
        let date_time = {
            if let Some(timezone) = settings.timezone {
                let date_time = {
                    let mut date_time = horae::Utc::now();
                    date_time.with_timezone(timezone);
                    date_time.to_string()
                };
                date_time
            } else {
                horae::Utc::now().to_string()
            }
        };
        Self {
            title: String::new(),
            text: String::new(),
            date_time,
        }
    }
}
