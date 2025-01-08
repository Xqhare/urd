use fortuna::Fortuna;


pub struct TipsNTricks {
    pub tips_and_tricks: Vec<TipOrTrick>,
    pub index: usize,
}

impl Default for TipsNTricks {
    fn default() -> Self {
        let mut out: Vec<TipOrTrick> = Vec::new();
        let tips = vec![
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

