use fortuna::Fortuna;


pub struct TipsNTricks {
    pub tips_and_tricks: Vec<TipOrTrick>,
    pub index: usize,
}

impl Default for TipsNTricks {
    fn default() -> Self {
        let mut out: Vec<TipOrTrick> = Vec::new();
        let tips = vec![
            ("You can hide me!", "Pressing the Escape key will hide this until the next time you open Urd."),
            ("Navigating the Sidebar 1", "Clicking on the field in the top left corner will take you up a level in the file tree."),
            ("Navigating the Sidebar 2", "Clicking on an entry in the left side panel will open it. If it is a folder (years or months) this folder will be opened on the left side. If it is a journal entry, it will open in the main panel to the right."),
            ("Entry dates and titles", "A journal entry has a date as it's title. The date cannot be changed. The only way to generate a journal entry for a specific date, is to launch Urd on that date and write one."),
            ("Custom moods", "You can add custom moods! It's right above the text box. Also check the settings page, you can edit the colour of any mood you like."),
            ("Important days", "Had an important day? Mark it and find it later!"),
            ("Theme", "Urd always follows the system theme."),
            ("Disable Tips and Tricks", "You can also press 'Don't show again' to not see them again. You can re-enable them in the settings page. "),
            ("Tags", "There are 4 types of tags. All are added into the entry itself. You can click on any one of them to open the search page and search for that tag."),
            ("Project tags", "The project tags of the entry. Added by prepending the word with '+'. e.g. +Holiday"),
            ("Context tags", "The context tags of the entry. Added by prepending the word with '@'. e.g. @Work"),
            ("Special tags", "These tags consists of a key:value pair. You can use this to store any information you want. e.g. date_due:2025-03-15"),
            ("Bespoke tags", "Tags for any miscellaneous information. They are added by prepending the word with '#'. e.g. #Tag"),
            ("Search", "TODO"),
            ("Backup", "TODO"),
            ("Restore", "TODO"),
            ("Saving", "TODO"),
            ("Export", "TODO"),
            // TODO: Add more tips!
        ];
        let index = {
            let mut rng = Fortuna::new();
            rng.random_index(tips.len())
        };
        for (title, text) in tips {
            out.push(TipOrTrick::new(title, text));
        }
        Self {
            tips_and_tricks: out,
            index,
        }
    }
}

pub struct TipOrTrick {
    pub title: String,
    pub text: String,
}

impl TipOrTrick {
    fn new<S: ToString>(title: S, text: S) -> Self {
        Self {
            title: title.to_string(),
            text: text.to_string(),
        }
    }
}

