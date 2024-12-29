use std::collections::{BTreeMap, VecDeque};

use nabu::{Array, Object, XffValue};

use crate::{paths::JOURNAL_FILE, settings::Settings};


#[derive(Clone, Debug)]
pub enum EntryType {
    JournalEntry(JournalEntry),
    Folder(Folder),
}

impl From<Folder> for EntryType {
    fn from(f: Folder) -> Self {
        EntryType::Folder(f)
    }
}

impl From<JournalEntry> for EntryType {
    fn from(e: JournalEntry) -> Self {
        EntryType::JournalEntry(e)
    }
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

    pub fn get_folder(&self) -> Option<&Folder> {
        match self {
            EntryType::JournalEntry(_) => None,
            EntryType::Folder(f) => Some(f),
        }
    }

    pub fn get_journal_entry(&self) -> Option<&JournalEntry> {
        match self {
            EntryType::JournalEntry(e) => Some(e),
            EntryType::Folder(_) => None,
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

    pub fn serialize(&self) -> XffValue {
        match self {
            EntryType::JournalEntry(e) => e.serialize(),
            EntryType::Folder(f) => XffValue::from(f.serialize()),
        }
    }

    pub fn deserialize(value: &XffValue) -> Self {
        let bind = value.into_object().unwrap();
        let tmp_check = bind.get("name");
        if tmp_check.is_some() {
            // Folder
            Self::Folder(Folder::deserialize(value))
        } else {
            // JournalEntry
            Self::JournalEntry(JournalEntry::deserialize(value))
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

    pub fn serialize(&self) -> Object {
        let mut out = Object::new();
        out.insert("name", XffValue::String(self.name.clone()));
        out.insert("entries", XffValue::Array(self.entries.iter().map(|e| e.serialize()).collect()));
        out
    }

    pub fn deserialize(value: &XffValue) -> Self {
        let name = value.into_object().unwrap().get("name").unwrap().into_string().unwrap();
        let entries = value.into_object().unwrap().get("entries").unwrap().into_array().unwrap();
        let mut out = VecDeque::new();
        for entry in entries {
            out.push_back(EntryType::deserialize(&entry));
        }
        Self {
            name,
            entries: out,
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
        let mut month_folder = Folder::new(month.to_string());
        let current_entry = JournalEntry::new(settings);
        month_folder.entries.push_back(EntryType::JournalEntry(current_entry.clone()));
        let mut year_folder = Folder::new(year.to_string());
        year_folder.entries.push_back(EntryType::Folder(month_folder));
        let entries = VecDeque::from([EntryType::Folder(year_folder)]);
        Self {
            entries,
            current_entry,
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let serialized = self.serialize();
        let out = nabu::serde::write(JOURNAL_FILE, serialized);
        match out {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn load(settings: &Settings) -> Result<Self, String> {
        let data = nabu::serde::read(JOURNAL_FILE);
        match data {
            Ok(d) => {
                Ok(Journal::deserialize(&d, settings))

            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn serialize(&self) -> XffValue {
        let mut serialized = Object::new();
        let entries = {
            let mut out = Array::new();
            for entry in self.entries.iter() {
                out.push(entry.serialize());
            }
            out
        };
        serialized.insert("entries", XffValue::from(entries));
        XffValue::from(serialized)
    }

    fn deserialize(read_value: &XffValue, settings: &Settings) -> Self {
        let all_entries = read_value.into_object().unwrap().get("entries").unwrap().into_array().unwrap();
        let mut entries = VecDeque::new();
        for entry in all_entries {
            entries.push_back(EntryType::deserialize(&entry));
        }
        let last_entry = {
            let last_year = entries.front().unwrap().get_folder().unwrap();
            let last_month = last_year.entries.front().unwrap().get_folder().unwrap();
            let last_entry = last_month.entries.front().unwrap().get_journal_entry().unwrap();
            let mut current_date = horae::Utc::now();
            current_date.with_timezone(settings.timezone.timezone);
            if current_date.date().to_string() == last_entry.title {
                last_entry.clone()
            } else {
                let out = JournalEntry::new(settings);
                entries.front_mut().unwrap().get_folder_mut().unwrap().entries.front_mut().unwrap().get_folder_mut().unwrap().entries.push_front(EntryType::JournalEntry(out.clone()));
                out
            }
        };
        Self {
            entries,
            current_entry: last_entry,
        }
    }

    pub fn export(&self) -> Result<(), String> {
        todo!("export journal into a directory with the same structure as journal, but with the journal entries as .txt / .md files")
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
            out.insert("project_tags".to_string(), XffValue::Array(Array::new()));
            out.insert("context_tags".to_string(), XffValue::Array(Array::new()));
            out.insert("special_tags".to_string(), XffValue::from(Object::new()));
            out.insert("bespoke_tags".to_string(), XffValue::Array(Array::new()));
            out
        };
        Self {
            title,
            text: String::new(),
            metadata,
        }
    }

    pub fn serialize(&self) -> XffValue {
        let mut serialized = Object::new();
        serialized.insert("title", XffValue::String(self.title.clone()));
        serialized.insert("text", XffValue::String(self.text.clone()));
        serialized.insert("metadata", XffValue::from(self.metadata.clone()));
        XffValue::from(serialized)
    }

    pub fn deserialize(serialized: &XffValue) -> Self {
        let title = serialized.into_object().unwrap().get("title").unwrap().into_string().unwrap();
        let text = serialized.into_object().unwrap().get("text").unwrap().into_string().unwrap();
        let metadata = serialized.into_object().unwrap().get("metadata").unwrap().into_object().unwrap().into_btree_map();
        Self {
            title,
            text,
            metadata,
        }
    }

    pub fn overwrite(&mut self, serialized: String) {
        self.text = serialized.clone();
        let new_metadata = deserialize_entry_metadata(serialized);
        self.metadata.insert("project_tags".to_string(), new_metadata.get("project_tags").unwrap().clone());
        self.metadata.insert("context_tags".to_string(), new_metadata.get("context_tags").unwrap().clone());
        self.metadata.insert("special_tags".to_string(), new_metadata.get("special_tags").unwrap().clone());
        self.metadata.insert("bespoke_tags".to_string(), new_metadata.get("bespoke_tags").unwrap().clone());
    }

    pub fn reset(&mut self) {
        let tmp_metadata_date = self.metadata.get("date").unwrap().into_object().unwrap();
        // Title stays, text is reset, metadata is reset except date
        self.text = String::new();
        self.metadata = {
            let mut out = BTreeMap::new();
            out.insert("date".to_string(), XffValue::from(tmp_metadata_date));
            out.insert("project_tags".to_string(), XffValue::Array(Array::new()));
            out.insert("context_tags".to_string(), XffValue::Array(Array::new()));
            out.insert("special_tags".to_string(), XffValue::from(Object::new()));
            out.insert("bespoke_tags".to_string(), XffValue::Array(Array::new()));
            out
        };
    }
}

fn deserialize_entry_metadata(text: String) -> BTreeMap<String, XffValue> {
    // I know, unicode segmentation etc...
    // But splitting by unicode whitespace is enough for this
    let mut project_tags: Vec<XffValue> = Vec::new();
    let mut context_tags: Vec<XffValue> = Vec::new();
    let mut bespoke_tags: Vec<XffValue> = Vec::new();
    let mut special_tags: Object = Object::new();
    let mut metadata: BTreeMap<String, XffValue> = BTreeMap::new();
    
    for word in text.split_whitespace() {
        // Check if a word starts with + or @, or has a : wrapped in it
        if word.starts_with("+") && word.len() > 1 {
            // Project tag
            project_tags.push(XffValue::String(word.to_string()));
        } else if word.starts_with("@") && word.len() > 1 {
            // Context tag
            context_tags.push(XffValue::String(word.to_string()));
        } else if word.contains(":") && !word.starts_with(":") && !word.ends_with(":") {
            // Special tag
            let (key, value) = word.split_once(":").unwrap();
            special_tags.insert(key.to_string(), XffValue::String(value.to_string()));
        } else if word.starts_with("#") && word.len() > 1 {
            // User Tag
            bespoke_tags.push(XffValue::String(word.to_string()));
        }
    }
    metadata.insert("project_tags".to_string(), XffValue::Array(Array::from(project_tags)));
    metadata.insert("context_tags".to_string(), XffValue::Array(Array::from(context_tags)));
    metadata.insert("special_tags".to_string(), XffValue::from(special_tags));
    metadata.insert("bespoke_tags".to_string(), XffValue::Array(Array::from(bespoke_tags)));
    metadata
}

#[test]
fn deserialize_metadata() {
    let str0 = "+project1 +project2 @context1 @context2 key1:val1 key2:val2";
    let metadata0 = deserialize_entry_metadata(str0.to_string());
    assert_eq!(metadata0.get("project_tags").unwrap().into_array().unwrap().len(), 2);
    assert_eq!(metadata0.get("context_tags").unwrap().into_array().unwrap().len(), 2);
    assert_eq!(metadata0.get("special_tags").unwrap().into_object().unwrap().len(), 2);

    assert_eq!(metadata0.get("project_tags").unwrap().into_array().unwrap().get(0).unwrap().into_string().unwrap(), "+project1");
    assert_eq!(metadata0.get("project_tags").unwrap().into_array().unwrap().get(1).unwrap().into_string().unwrap(), "+project2");
    assert_eq!(metadata0.get("context_tags").unwrap().into_array().unwrap().get(0).unwrap().into_string().unwrap(), "@context1");
    assert_eq!(metadata0.get("context_tags").unwrap().into_array().unwrap().get(1).unwrap().into_string().unwrap(), "@context2");
    assert_eq!(metadata0.get("special_tags").unwrap().into_object().unwrap().get("key1").unwrap().into_string().unwrap(), "val1");
    assert_eq!(metadata0.get("special_tags").unwrap().into_object().unwrap().get("key2").unwrap().into_string().unwrap(), "val2");

    let str1 = "This may not crash if feed only text";
    let metadata1 = deserialize_entry_metadata(str1.to_string());
    assert_eq!(metadata1.get("project_tags").unwrap().into_array().unwrap().len(), 0);
    assert_eq!(metadata1.get("context_tags").unwrap().into_array().unwrap().len(), 0);
    assert_eq!(metadata1.get("special_tags").unwrap().into_object().unwrap().len(), 0);

    let str2 = "This is @an +actual test:123";
    let metadata2 = deserialize_entry_metadata(str2.to_string());
    assert_eq!(metadata2.get("project_tags").unwrap().into_array().unwrap().len(), 1);
    assert_eq!(metadata2.get("context_tags").unwrap().into_array().unwrap().len(), 1);
    assert_eq!(metadata2.get("special_tags").unwrap().into_object().unwrap().len(), 1);

    assert_eq!(metadata2.get("project_tags").unwrap().into_array().unwrap().get(0).unwrap().into_string().unwrap(), "+actual");
    assert_eq!(metadata2.get("context_tags").unwrap().into_array().unwrap().get(0).unwrap().into_string().unwrap(), "@an");
    assert_eq!(metadata2.get("special_tags").unwrap().into_object().unwrap().get("test").unwrap().into_string().unwrap(), "123");
}
