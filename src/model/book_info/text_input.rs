use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default)]
pub struct TextInput {
    pub text: String,
    pub cursor: usize,
    pub multiline: bool,
}

impl TextInput {
    pub fn new(text: String) -> Self {
        let cursor = UnicodeSegmentation::graphemes(text.as_str(), true).count();
        Self {
            text,
            cursor,
            multiline: false,
        }
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    pub fn insert_char(&mut self, c: char) {
        let graphemes: Vec<&str> =
            UnicodeSegmentation::graphemes(self.text.as_str(), true).collect();

        let cursor = self.cursor.min(graphemes.len());

        let mut new = String::new();

        // before cursor
        for g in &graphemes[..cursor] {
            new.push_str(g);
        }

        // inserted char
        new.push(c);

        // after cursor
        for g in &graphemes[cursor..] {
            new.push_str(g);
        }

        self.text = new;
        self.cursor += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor == 0 {
            return;
        }

        let graphemes: Vec<&str> =
            UnicodeSegmentation::graphemes(self.text.as_str(), true).collect();

        let mut new = String::new();

        for (i, g) in graphemes.iter().enumerate() {
            if i != self.cursor - 1 {
                new.push_str(g);
            }
        }

        self.text = new;
        self.cursor -= 1;
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let len = UnicodeSegmentation::graphemes(self.text.as_str(), true).count();
        if self.cursor < len {
            self.cursor += 1;
        }
    }
}
