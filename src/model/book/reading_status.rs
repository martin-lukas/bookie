use log::error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReadingStatus {
    #[default]
    ToRead,
    Reading,
    Read,
}

impl ReadingStatus {
    pub fn index(&self) -> usize {
        match self {
            ReadingStatus::ToRead => 0,
            ReadingStatus::Reading => 1,
            ReadingStatus::Read => 2,
        }
    }

    pub fn from(index: usize) -> Self {
        match index {
            0 => ReadingStatus::ToRead,
            1 => ReadingStatus::Reading,
            2 => ReadingStatus::Read,
            _ => {
                error!("Invalid index for reading status: {index}");
                panic!("Invalid index for reading status");
            }
        }
    }
}
