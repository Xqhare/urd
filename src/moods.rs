use eframe::egui::Color32;
use nabu::{Array, Object, XffValue};

pub struct Mood {
    pub name: String,
    pub colour: Color32,
}

impl Mood {
    pub fn new(name: String, colour: Color32) -> Self {
        Self { name, colour }
    }
}

impl Default for Mood {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            colour: Color32::from_rgba_unmultiplied(0, 0, 0, 255),
        }
    }
}

pub fn default_moods() -> Object {
    let mut out = Object::new();
    // Only one with opacity set to 0
    out.insert("", XffValue::from(Array::from(vec![0, 0, 0, 0])));
    out.insert("Alert", XffValue::from(Array::from(vec![0, 100, 150, 255])));
    out.insert("Angry", XffValue::from(Array::from(vec![120, 0, 0, 255])));
    out.insert("Blank", XffValue::from(Array::from(vec![132, 132, 132, 255])));
    out.insert("Bored", XffValue::from(Array::from(vec![50, 50, 66, 255])));
    out.insert("Calm", XffValue::from(Array::from(vec![50, 75, 100, 255])));
    out.insert("Confident", XffValue::from(Array::from(vec![160, 130, 0, 255])));
    out.insert("Confused", XffValue::from(Array::from(vec![255, 175, 0, 255])));
    out.insert("Depressed", XffValue::from(Array::from(vec![45, 30, 15, 255])));
    out.insert("Embarrassed", XffValue::from(Array::from(vec![175, 0, 150, 255])));
    out.insert("Energetic", XffValue::from(Array::from(vec![0, 155, 55, 255])));
    out.insert("Excited", XffValue::from(Array::from(vec![0, 200, 75, 255])));
    out.insert("Happy", XffValue::from(Array::from(vec![127, 255, 127, 255])));
    out.insert("Hollow", XffValue::from(Array::from(vec![200, 200, 200, 255])));
    out.insert("Hopeful", XffValue::from(Array::from(vec![125, 125, 0, 255])));
    out.insert("Ill", XffValue::from(Array::from(vec![80, 180, 0, 255])));
    out.insert("Intense", XffValue::from(Array::from(vec![88, 88, 0 , 255])));
    out.insert("Nervous", XffValue::from(Array::from(vec![40, 80, 120, 255])));
    out.insert("Overwhelmed", XffValue::from(Array::from(vec![125, 150, 175, 255])));
    out.insert("Playful", XffValue::from(Array::from(vec![75, 50, 190, 255])));
    out.insert("Relaxed", XffValue::from(Array::from(vec![0, 125, 200, 255])));
    out.insert("Restless", XffValue::from(Array::from(vec![75, 85, 105, 255])));
    out.insert("Sad", XffValue::from(Array::from(vec![20, 30, 45, 255])));
    out.insert("Scared", XffValue::from(Array::from(vec![69, 69, 0, 255])));
    out.insert("Serious", XffValue::from(Array::from(vec![42, 69, 138, 255])));
    out.insert("Shocked", XffValue::from(Array::from(vec![50, 150, 100, 255])));
    out.insert("Silly", XffValue::from(Array::from(vec![145, 100, 175, 255])));
    out.insert("Sleepy", XffValue::from(Array::from(vec![69, 69, 196, 255])));
    out.insert("Stressed", XffValue::from(Array::from(vec![200, 80, 0, 255])));
    out.insert("Surprised", XffValue::from(Array::from(vec![0, 222, 111, 255])));
    out.insert("Troubled", XffValue::from(Array::from(vec![60, 15, 0, 255])));
    out.insert("Worried", XffValue::from(Array::from(vec![45, 30, 30, 255])));
    out
}
