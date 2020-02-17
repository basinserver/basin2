use crate::nbt::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(u32),
}

impl ClickEvent {
    pub fn serialize(&self) -> Value {
        let mut map = serde_json::Map::new();
        match self {
            ClickEvent::OpenUrl(url) => {
                map.insert("action".to_string(), Value::String("open_url".to_string()));
                map.insert("value".to_string(), Value::String(url.clone()));
            }
            ClickEvent::RunCommand(command) => {
                map.insert(
                    "action".to_string(),
                    Value::String("run_command".to_string()),
                );
                map.insert("value".to_string(), Value::String(command.clone()));
            }
            ClickEvent::SuggestCommand(command) => {
                map.insert(
                    "action".to_string(),
                    Value::String("suggest_command".to_string()),
                );
                map.insert("value".to_string(), Value::String(command.clone()));
            }
            ClickEvent::ChangePage(page) => {
                map.insert(
                    "action".to_string(),
                    Value::String("change_page".to_string()),
                );
                map.insert(
                    "value".to_string(),
                    Value::Number(serde_json::Number::from_f64(*page as f64).unwrap()),
                );
            }
        }
        Value::Object(map)
    }

    pub fn from(value: Value) -> Option<ClickEvent> {
        match value {
            Value::Object(map) => match (map.get("action"), map.get("value")) {
                (Some(Value::String(action)), Some(Value::String(value))) => match &**action {
                    "open_url" => Some(ClickEvent::OpenUrl(value.clone())),
                    "run_command" => Some(ClickEvent::RunCommand(value.clone())),
                    "suggest_command" => Some(ClickEvent::SuggestCommand(value.clone())),
                    "change_page" => value
                        .parse::<u32>()
                        .map(|value| ClickEvent::ChangePage(value))
                        .ok(),
                    _ => None,
                },
                (Some(Value::String(action)), Some(Value::Number(value))) => {
                    if action != "change_page" {
                        None
                    } else {
                        value
                            .as_u64()
                            .map(|value| ClickEvent::ChangePage(value as u32))
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HoverEvent {
    ShowText(Box<ChatComponent>),
    ShowItem(Nbt),
    ShowEntity(Nbt),
}

impl HoverEvent {
    pub fn serialize(&self) -> Value {
        let mut map = serde_json::Map::new();
        match self {
            HoverEvent::ShowText(text) => {
                map.insert("action".to_string(), Value::String("show_text".to_string()));
                map.insert("value".to_string(), text.serialize());
            }
            HoverEvent::ShowItem(nbt) => {
                map.insert("show_item".to_string(), Value::String("".to_string()));
                // TODO
            }
            HoverEvent::ShowEntity(nbt) => {
                map.insert("show_entity".to_string(), Value::String("".to_string()));
                // TODO
            }
        }
        Value::Object(map)
    }

    pub fn from(value: Value) -> Option<HoverEvent> {
        match value {
            Value::Object(map) => {
                match (map.get("action"), map.get("value")) {
                    (Some(Value::String(action)), Some(Value::String(value))) => {
                        match &**action {
                            "show_text" => Some(HoverEvent::ShowText(Box::new(
                                ChatComponent::from(value.clone()),
                            ))),
                            "show_item" => Some(HoverEvent::ShowItem(Nbt::End)), // TODO
                            "show_entity" => Some(HoverEvent::ShowEntity(Nbt::End)), // TODO
                            _ => None,
                        }
                    }
                    (Some(Value::String(action)), Some(Value::Object(value))) => {
                        match &**action {
                            "show_text" => ChatComponent::parse(Value::Object(value.clone()))
                                .map(|component| HoverEvent::ShowText(Box::new(component))),
                            "show_item" => Some(HoverEvent::ShowItem(Nbt::End)), // TODO
                            "show_entity" => Some(HoverEvent::ShowEntity(Nbt::End)), // TODO
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ComponentScore {
    pub name: String,
    pub objective: String,
    pub value: Option<String>,
}

impl ComponentScore {
    pub fn serialize(&self) -> Value {
        let mut map = serde_json::Map::new();
        map.insert("name".to_string(), Value::String(self.name.clone()));
        map.insert(
            "objective".to_string(),
            Value::String(self.objective.clone()),
        );
        if self.value.is_some() {
            map.insert(
                "objective".to_string(),
                Value::String(self.value.as_ref().unwrap().clone()),
            );
        }
        Value::Object(map)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ComponentType {
    Str(String),
    Translation {
        translate: String,
        with: Vec<ChatComponent>,
    },
    Keybind(String),
    Score(ComponentScore),
}

impl Default for ComponentType {
    fn default() -> ComponentType {
        ComponentType::Str("".to_string())
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ChatComponent {
    bold: bool,
    italic: bool,
    underlined: bool,
    strikethrough: bool,
    obfuscated: bool,
    color: Option<String>,
    insertion: Option<String>,
    clickEvent: Option<ClickEvent>,
    hoverEvent: Option<HoverEvent>,
    extra: Vec<ChatComponent>,
    data: ComponentType,
}

impl From<String> for ChatComponent {
    fn from(string: String) -> ChatComponent {
        ChatComponent {
            data: ComponentType::Str(string),
            ..Default::default()
        }
    }
}

fn parse_bool(value: Option<&Value>) -> bool {
    match value {
        Some(Value::Bool(b)) => *b,
        Some(Value::String(string)) => string == "true",
        _ => false,
    }
}

impl ChatComponent {
    pub fn parse(value: Value) -> Option<ChatComponent> {
        match value {
            Value::Object(value) => {
                let bold = parse_bool(value.get("bold"));
                let italic = parse_bool(value.get("bold"));
                let underlined = parse_bool(value.get("bold"));
                let strikethrough = parse_bool(value.get("bold"));
                let obfuscated = parse_bool(value.get("bold"));
                let color = match value.get("color") {
                    Some(Value::String(string)) => Some(string.clone()),
                    _ => None,
                };
                let insertion = match value.get("insertion") {
                    Some(Value::String(string)) => Some(string.clone()),
                    _ => None,
                };
                let clickEvent = match value.get("clickEvent") {
                    Some(Value::Object(value)) => ClickEvent::from(Value::Object(value.clone())),
                    _ => None,
                };
                let hoverEvent = match value.get("hoverEvent") {
                    Some(Value::Object(value)) => HoverEvent::from(Value::Object(value.clone())),
                    _ => None,
                };
                let extra: Vec<ChatComponent> = match value.get("extra") {
                    Some(Value::Array(values)) => values
                        .iter()
                        .flat_map(|value| ChatComponent::parse(value.clone()))
                        .collect(),
                    _ => vec![],
                };
                let data = if value.contains_key("text") {
                    match value.get("text").map(|value| value.as_str()) {
                        Some(Some(string)) => ComponentType::Str(string.to_string()),
                        _ => return None,
                    }
                } else if value.contains_key("translate") {
                    let with: Vec<ChatComponent> = match value.get("with") {
                        Some(Value::Array(withs)) => withs
                            .iter()
                            .flat_map(|value| ChatComponent::parse(value.clone()))
                            .collect(),
                        _ => vec![],
                    };
                    match value.get("translate").map(|value| value.as_str()) {
                        Some(Some(string)) => ComponentType::Translation {
                            translate: string.to_string(),
                            with,
                        },
                        _ => return None,
                    }
                } else if value.contains_key("keybind") {
                    match value.get("keybind").map(|value| value.as_str()) {
                        Some(Some(string)) => ComponentType::Keybind(string.to_string()),
                        _ => return None,
                    }
                } else if value.contains_key("score") {
                    match value.get("score") {
                        Some(Value::Object(map)) => {
                            match (map.get("name"), map.get("objective"), map.get("value")) {
                                (
                                    Some(Value::String(name)),
                                    Some(Value::String(objective)),
                                    value,
                                ) => ComponentType::Score(ComponentScore {
                                    name: name.clone(),
                                    objective: objective.clone(),
                                    value: match value {
                                        Some(Value::String(string)) => Some(string.clone()),
                                        Some(Value::Number(value)) => Some(format!("{}", value)),
                                        Some(Value::Bool(b)) => Some(format!("{}", b)),
                                        _ => None,
                                    },
                                }),
                                _ => return None,
                            }
                        }
                        _ => return None,
                    }
                } else {
                    return None;
                };
                return Some(ChatComponent {
                    bold,
                    italic,
                    underlined,
                    strikethrough,
                    obfuscated,
                    color,
                    insertion,
                    clickEvent,
                    hoverEvent,
                    extra,
                    data,
                });
            }
            Value::String(value) => {
                return Some(ChatComponent::from(value));
            }
            _ => None,
        }
    }

    pub fn serialize(&self) -> Value {
        let mut map = serde_json::Map::new();
        if self.bold {
            map.insert("bold".to_string(), Value::Bool(self.bold));
        }
        if self.italic {
            map.insert("italic".to_string(), Value::Bool(self.italic));
        }
        if self.underlined {
            map.insert("underlined".to_string(), Value::Bool(self.underlined));
        }
        if self.strikethrough {
            map.insert("strikethrough".to_string(), Value::Bool(self.strikethrough));
        }
        if self.obfuscated {
            map.insert("obfuscated".to_string(), Value::Bool(self.obfuscated));
        }
        match &self.color {
            Some(color) => {
                map.insert("color".to_string(), Value::String(color.clone()));
            }
            _ => (),
        }
        match &self.insertion {
            Some(insertion) => {
                map.insert("insertion".to_string(), Value::String(insertion.clone()));
            }
            _ => (),
        }
        match &self.clickEvent {
            Some(clickEvent) => {
                map.insert("clickEvent".to_string(), clickEvent.serialize());
            }
            _ => (),
        }
        match &self.hoverEvent {
            Some(hoverEvent) => {
                map.insert("hoverEvent".to_string(), hoverEvent.serialize());
            }
            _ => (),
        }
        if self.extra.len() > 0 {
            map.insert(
                "extra".to_string(),
                Value::Array(self.extra.iter().map(|item| item.serialize()).collect()),
            );
        }
        match &self.data {
            ComponentType::Str(string) => {
                map.insert("text".to_string(), Value::String(string.clone()));
            }
            ComponentType::Translation { translate, with } => {
                map.insert("translate".to_string(), Value::String(translate.clone()));
                if with.len() > 0 {
                    map.insert(
                        "with".to_string(),
                        with.iter().map(|item| item.serialize()).collect(),
                    );
                }
            }
            ComponentType::Keybind(string) => {
                map.insert("keybind".to_string(), Value::String(string.clone()));
            }
            ComponentType::Score(score) => {
                map.insert("name".to_string(), score.serialize());
            }
        }
        Value::Object(map)
    }
}
