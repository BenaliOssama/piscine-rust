use chrono::Duration;
use colored::*;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
         let (r, g, b) = self.color;
        write!(f, "{}", self.content.truecolor(r,g,b))?;
         Ok(())
    }
}

use Event::*;
use Event::*;
impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        let content = match self {
            Event::Remainder(msg) => format!("Reminder: {}", msg),
            Event::Registration(dur) => format!("Registration in {} seconds", dur.num_seconds()),
            Event::Appointment(who) => format!("Appointment with {}", who),
            Event::Holiday => "Holiday!".to_string(),
        };

        Notification {
            size: 50,
            color: (50, 50, 50),
            position: Position::Bottom,
            content,
        }
    }
}
