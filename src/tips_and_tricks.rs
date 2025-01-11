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
            ("", "Clicking on the field in the top left corner will take you up a level in the file tree."),
            ("", "Clicking on an entry in the left side panel will open it. If it is a folder (years or months) this folder will be opened on the left side. If it is a journal entry, it will open in the main panel to the right."),
            ("", "Journal entry dates cannot be changed."),
            ("", "You can add custom moods!"),
            ("", "Had an important day? Mark it and find it later!"),
            ("Theme", "Urd always follows the system theme."),
            ("", ""),
            ("", ""),
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

