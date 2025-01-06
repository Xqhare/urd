use eframe::egui::Color32;
use nabu::{tvec_to_xff_value, Object, XffValue};

pub struct Mood {
    pub name: String,
    pub colour: Color32,
}

impl Default for Mood {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            colour: Color32::from_rgba_unmultiplied(0, 0, 0, 255),
        }
    }
}

impl From<(String, XffValue)> for Mood {
    fn from(value: (String, XffValue)) -> Self {
        let colour = value.1.into_array().unwrap();
        Self {
            name: value.0,
            colour: Color32::from_rgba_unmultiplied(
                colour
                    .get(0)
                    .unwrap()
                    .into_number()
                    .unwrap()
                    .into_usize()
                    .unwrap()
                    .try_into()
                    .expect("Colour value out of range"),
                colour
                    .get(1)
                    .unwrap()
                    .into_number()
                    .unwrap()
                    .into_usize()
                    .unwrap()
                    .try_into()
                    .expect("Colour value out of range"),
                colour
                    .get(2)
                    .unwrap()
                    .into_number()
                    .unwrap()
                    .into_usize()
                    .unwrap()
                    .try_into()
                    .expect("Colour value out of range"),
                colour
                    .get(3)
                    .unwrap()
                    .into_number()
                    .unwrap()
                    .into_usize()
                    .unwrap()
                    .try_into()
                    .expect("Colour value out of range"),
            ),
        }
    }
}

pub fn default_moods() -> Object {
    let mut out = Object::new();
    // Only one with opacity set to 0
    out.insert("", tvec_to_xff_value![u8; 0, 0, 0, 0]);
    out.insert("Alert", tvec_to_xff_value![u8; 0, 100, 150, 255]);
    out.insert("Angry", tvec_to_xff_value![u8; 120, 0, 0, 255]);
    out.insert("Blank", tvec_to_xff_value![u8; 132, 132, 132, 255]);
    out.insert("Bored", tvec_to_xff_value![u8; 50, 50, 66, 255]);
    out.insert("Calm", tvec_to_xff_value![u8; 50, 75, 100, 255]);
    out.insert("Confident", tvec_to_xff_value![u8; 160, 130, 0, 255]);
    out.insert("Confused", tvec_to_xff_value![u8; 255, 175, 0, 255]);
    out.insert("Depressed", tvec_to_xff_value![u8; 55, 30, 0, 255]);
    out.insert("Embarrassed", tvec_to_xff_value![u8; 175, 0, 150, 255]);
    out.insert("Energetic", tvec_to_xff_value![u8; 0, 155, 55, 255]);
    out.insert("Excited", tvec_to_xff_value![u8; 0, 200, 75, 255]);
    out.insert("Happy", tvec_to_xff_value![u8; 127, 255, 127, 255]);
    out.insert("Hollow", tvec_to_xff_value![u8; 200, 200, 200, 255]);
    out.insert("Hopeful", tvec_to_xff_value![u8; 125, 125, 0, 255]);
    out.insert("Ill", tvec_to_xff_value![u8; 80, 180, 0, 255]);
    out.insert("Intense", tvec_to_xff_value![u8; 108, 88, 0 , 255]);
    out.insert("Nervous", tvec_to_xff_value![u8; 40, 80, 120, 255]);
    out.insert("Overwhelmed", tvec_to_xff_value![u8; 125, 150, 175, 255]);
    out.insert("Playful", tvec_to_xff_value![u8; 75, 50, 190, 255]);
    out.insert("Relaxed", tvec_to_xff_value![u8; 0, 125, 200, 255]);
    out.insert("Restless", tvec_to_xff_value![u8; 75, 85, 105, 255]);
    out.insert("Sad", tvec_to_xff_value![u8; 0, 45, 55, 255]);
    out.insert("Scared", tvec_to_xff_value![u8; 69, 69, 0, 255]);
    out.insert("Serious", tvec_to_xff_value![u8; 42, 69, 138, 255]);
    out.insert("Shocked", tvec_to_xff_value![u8; 50, 150, 100, 255]);
    out.insert("Silly", tvec_to_xff_value![u8; 145, 100, 175, 255]);
    out.insert("Sleepy", tvec_to_xff_value![u8; 69, 69, 196, 255]);
    out.insert("Stressed", tvec_to_xff_value![u8; 200, 80, 0, 255]);
    out.insert("Surprised", tvec_to_xff_value![u8; 0, 222, 111, 255]);
    out.insert("Troubled", tvec_to_xff_value![u8; 60, 15, 0, 255]);
    out.insert("Worried", tvec_to_xff_value![u8; 45, 0, 30, 255]);
    out
}
