use std::{fmt::format, str::FromStr};

pub struct SubscriberName(String);

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

impl FromStr for SubscriberName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let forbidden_chars = ['/', '('];
        let contains_forbidden_chars = s.chars().any(|c| forbidden_chars.contains(&c));
        if is_empty_or_whitespace || contains_forbidden_chars {
            Err(format!("{} is not a valid subscribe name", s))
        } else {
            Ok(Self(s.to_owned()))
        }
    }
}

impl SubscriberName {
    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<String> for SubscriberName {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
