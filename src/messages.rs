use crate::requests::Part;

#[derive(Debug, Clone)]
pub enum Content {
    Text(String),
}

impl Default for Content {
    fn default() -> Self {
        Content::Text("".to_string())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    User {
        content: Content,
        name: Option<String>,
    },
    System {
        content: Content,
        name: Option<String>,
    },
    Developer {
        content: Content,
        name: Option<String>,
    },
    Tool {
        content: String,
    },
}

impl Default for Message {
    fn default() -> Self {
        Message::User {
            content: Content::default(),
            name: None,
        }
    }
}

impl Message {
    pub fn to_part(&self) -> Part {
        match self {
            Message::User { content, .. }
            | Message::System { content, .. }
            | Message::Developer { content, .. } => match content {
                Content::Text(text) => Part::text(text),
            },
            Message::Tool { content } => Part::text(content),
        }
    }
}
