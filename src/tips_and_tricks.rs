use fortuna::Fortuna;


pub struct TipsNTricks {
    pub tips_and_tricks: Vec<TipOrTrick>,
    pub index: usize,
}

impl Default for TipsNTricks {
    fn default() -> Self {
        let mut out: Vec<TipOrTrick> = Vec::new();
        // Some ad-hoc style guidelines
        // - Descriptive and short Title
        // - As short of a text as possible
        // - Friendly tone
        // - Consistent vocabulary
        // - Always describe a location of gui elements
        let tips = vec![
            ("You can hide me!", "Pressing the Escape key will hide this until the next time you open Urd."),
            ("Navigating the Sidebar 1", "Clicking on the field in the top left corner will take you up a level in the file tree."),
            ("Navigating the Sidebar 2", "Clicking on an entry in the left side panel will open it. If it is a folder (years or months) this folder will be opened on the left side. If it is a journal entry, it will open in the main panel to the right."),
            ("Entry dates and titles", "A journal entry has a date as it's title. The date cannot be changed. The only way to generate a journal entry for a specific date, is to launch Urd on that date and write one."),
            ("Custom moods", "You can add custom moods! It's right above the text box. Also check the settings page, you can edit the colour of any mood you like."),
            ("Important days", "Had an important day? Mark it with the checkbox above the text field and find it later by clicking on the Jpurnal menu button at the top!"),
            ("Theme", "Urd always follows the system theme."),
            ("Disable Tips and Tricks", "You can also press 'Don't show again' to not see them again. You can re-enable them in the settings page."),
            ("Tags", "There are 4 types of tags. All are added into the entry itself. You can click on any one of them to open the search page and search for that tag."),
            ("Project tags", "The project tags of the entry. Added by prepending the word with '+'. e.g. +Holiday"),
            ("Context tags", "The context tags of the entry. Added by prepending the word with '@'. e.g. @Work"),
            ("Special tags", "These tags consists of a key:value pair. You can use this to store any information you want. e.g. date_due:2025-03-15"),
            ("Bespoke tags", "Tags for any miscellaneous information. They are added by prepending the word with '#'. e.g. #Tag"),
            ("Settings", "Check out the settings page to fine tune Urd to your needs. You can find it inside the Urd menu button."),
            ("Password", "You can set up a password to secure your journal from prying eyes. Check the settings!"),
            ("Help", "You can open the Help window to learn more about anything in Urd. You can find it inside the Urd menu button."),
            ("Search", "You can use the search feature to find entries. Check the search page for more information."),
            ("Backup", "Never worry about data loss. You can create a backup of your journal or even set up an automatic backup. You can find it inside the Journal menu button under 'Backup'."),
            ("Restore", "If you have a backup, you can restore your journal from it. You can find it inside the Journal menu button under 'Backup'."),
            ("Saving", "You only need to save the journal entry you have currently open (If you made changes of course). All other entries are already saved in the background. Hit the 'Save entry' button above the title of the open entry to save."),
            ("Export", "You can export the journal into a directory of your choice. All entries will be exported as .txt files and put into folders just like they are in the side bar. You can find it inside the Journal menu button."),
            ("Opening an entry", "You will most often open an entry by clicking on it in the sidebar. Clicking an entry inside any submenu like search will also open it."),
            ("Mood subpage", "Opening the mood page, found inside the journal menu, will show you all entries you have created. They are displayed as little blocks with a colour specific to the mood of an entry. You can click on them to open and edit them."),
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

