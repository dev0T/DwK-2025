use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoTitle(String);

impl TodoTitle {
    pub fn parse(title: String) -> Result<TodoTitle, String> {
        let size = title.trim().graphemes(true).count();

        if size == 0 {
            Err(format!("Title is empty."))
        } else if size > 140 {
            Err(format!(
                "Title is too long. Max size is 140, got {:?}.",
                size
            ))
        } else {
            Ok(Self(title))
        }
    }
}

impl AsRef<str> for TodoTitle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<String> for TodoTitle {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Into<String> for TodoTitle {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: TodoTitle,
}

impl Todo {
    pub fn new(id: Uuid, title: TodoTitle) -> Self {
        Self { id, title }
    }
}
