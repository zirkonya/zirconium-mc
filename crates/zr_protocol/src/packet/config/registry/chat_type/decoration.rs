use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentField {
    Text {
        text: String,
    },
    Translatable {
        translate: String,
        with: Option<Vec<Box<TextComponent>>>,
    },
    KeyBind {
        key_bind: String,
    },
    Score {
        score: Score,
    },
    Selector {
        selector: String,
        separator: Option<Box<TextComponent>>,
    },
    Nbt {
        nbt: String,
        interpret: Option<i8>,
        block: Option<String>,
        entity: Option<String>,
        storage: Option<String>,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Score {
    name: String,
    objectives: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClickEvent {
    action: String,
    value: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverEvent {
    ShowText {
        contents: String,
    },
    ShowItem {
        id: String,
        count: Option<i32>,
        tag: Option<String>,
    },
    ShowEntity {
        #[serde(rename = "type")]
        type_field: String,
        id: String,
        name: Option<String>,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Style {
    color: Option<String>,
    bold: Option<i8>,
    italic: Option<i8>,
    underlined: Option<i8>,
    strikethrough: Option<i8>,
    obfuscated: Option<i8>,
    font: Option<String>,
    insertion: Option<String>,
    click_event: Option<ClickEvent>,
    hover_event: Option<HoverEvent>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TextComponent {
    #[serde(flatten)]
    contents: Option<ContentField>,
    #[serde(flatten)]
    style: Option<Style>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Decoration {
    translation_key: String,
    style: Option<TextComponent>,
    parameters: Vec<String>,
}
