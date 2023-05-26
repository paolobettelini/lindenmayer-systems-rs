use lindenmayer_parser::*;

#[derive(Debug)]
pub struct ConfigLines {
    pub axiom: (String, bool),
    pub iter: (String, bool),
    pub initial_pos: (String, bool),
    pub initial_rot: (String, bool),
    pub initial_thickness: (String, bool),
    pub background_color: (String, bool),
    pub initial_color: (String, bool),
    pub canvas: (String, bool),
    pub seed: (String, bool),
    pub injections: (String, bool),
}

impl Default for ConfigLines {
    fn default() -> Self {
        let axiom = (String::from(""), true);
        let iter = (String::from("1"), false);
        let initial_pos = (String::from("375,560"), false);
        let initial_rot = (String::from("0"), false);
        let initial_thickness = (String::from("1"), false);
        let background_color = (String::from("white"), false);
        let initial_color = (String::from("black"), false);
        let canvas = (String::from("750,750"), false);
        let seed = (String::from("Default Seed"), false);
        let injections = (String::from(""), false);

        ConfigLines {
            axiom,
            iter,
            initial_pos,
            initial_rot,
            initial_thickness,
            background_color,
            initial_color,
            canvas,
            seed,
            injections,
        }
    }
}

impl ConfigLines {
    pub fn update(&mut self, line: &str, error: bool) {
        let index = {
            let index = line.find(' ');
            if let Some(value) = index {
                value
            } else {
                return;
            }
        };
        let stripped_line = line[index..].trim().to_string();

        if line.starts_with(AXIOM) {
            self.axiom = (stripped_line, error);
        } else if line.starts_with(ITER) {
            self.iter = (stripped_line, error);
        } else if line.starts_with(INITIAL_POS) {
            self.initial_pos = (stripped_line, error);
        } else if line.starts_with(INITIAL_ROT) {
            self.initial_rot = (stripped_line, error);
        } else if line.starts_with(INITIAL_THICKNESS) {
            self.initial_thickness = (stripped_line, error);
        } else if line.starts_with(BACKGROUND) {
            self.background_color = (stripped_line, error);
        } else if line.starts_with(INITIAL_COLOR) {
            self.initial_color = (stripped_line, error);
        } else if line.starts_with(CANVAS) {
            self.canvas = (stripped_line, error);
        } else if line.starts_with(SEED) {
            self.seed = (stripped_line, error);
        } else if line.starts_with(INJECT) {
            self.injections = (stripped_line, error);
        }
    }
}
