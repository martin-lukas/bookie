#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FormField {
    Title,
    Authors,
    Year,
    Pages,
    ReadingStatus,
    Rating,
    Note,
}

impl FormField {
    pub const ORDER: [FormField; 7] = [
        FormField::Title,
        FormField::Authors,
        FormField::Year,
        FormField::Pages,
        FormField::ReadingStatus,
        FormField::Rating,
        FormField::Note,
    ];

    pub fn next(&self) -> Self {
        let pos = Self::ORDER.iter().position(|f| f == self).unwrap();
        Self::ORDER[(pos + 1) % Self::ORDER.len()]
    }

    pub fn prev(&self) -> Self {
        let pos = Self::ORDER.iter().position(|f| f == self).unwrap();
        let len = Self::ORDER.len();
        Self::ORDER[(pos + len - 1) % len]
    }
}
