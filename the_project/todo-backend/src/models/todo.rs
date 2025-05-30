use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoTitle(String);

impl TodoTitle {
    pub fn parse(title: String) -> Result<TodoTitle, String> {
        let is_empty = title.trim().is_empty();
        let is_too_long = title.graphemes(true).count() > 140;

        if is_empty || is_too_long {
            Err(format!("Error while parsing string"))
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
