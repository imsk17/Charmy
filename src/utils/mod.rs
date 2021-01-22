use std::str::FromStr;
use teloxide::prelude::{Request, UpdateWithCx};
use teloxide::types::{ChatKind, ChatMember, ChatPermissions, Message};

#[derive(Debug)]
pub enum UnitOfTime {
    Seconds,
    Minutes,
    Hours,
    Days,
    None,
}

impl FromStr for UnitOfTime {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "d" | "days" => Ok(UnitOfTime::Days),
            "h" | "hours" => Ok(UnitOfTime::Hours),
            "m" | "minutes" => Ok(UnitOfTime::Minutes),
            "s" | "seconds" => Ok(UnitOfTime::Seconds),
            "" => Ok(UnitOfTime::None),
            _ => Err("Allowed units: h, m, s, d"),
        }
    }
}

#[derive(Debug)]
pub struct RestrictTime {
    pub time: u32,
    pub unit: UnitOfTime,
}

trait New {
    fn new(time: u32, unit: UnitOfTime) -> RestrictTime;
}

impl New for RestrictTime {
    fn new(time: u32, unit: UnitOfTime) -> RestrictTime {
        RestrictTime { time, unit }
    }
}

impl FromStr for RestrictTime {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let command= s.split(" ").collect::<Vec<&str>>();
        let length = command.len() as i32;
        match length > 1 {
            false => Ok(RestrictTime { time: 0, unit: UnitOfTime::None }),
            true => Ok(RestrictTime { time: command[1].parse::<u32>().expect("Malformed Command"), unit: UnitOfTime::from_str(command[2]).expect("Malformed Command")})
        }
    }
}

trait TimeProperties {
    fn is_empty(&self) -> bool;
}

pub trait ToSeconds {
    fn to_seconds(&self) -> u32;
}

impl ToSeconds for RestrictTime {
    fn to_seconds(&self) -> u32 {
        match self.unit {
            UnitOfTime::Seconds => self.time,
            UnitOfTime::Minutes => self.time * 60,
            UnitOfTime::Hours => self.time * 3600,
            UnitOfTime::Days => self.time * 86400,
            UnitOfTime::None => 0
        }
    }
}

impl ToString for RestrictTime {
    fn to_string(&self) -> String {
        match self.unit {
            UnitOfTime::Seconds => String::from(self.time.to_string() + " Seconds"),
            UnitOfTime::Minutes => String::from(self.time.to_string() + " Minutes"),
            UnitOfTime::Hours => String::from(self.time.to_string() + " Hours"),
            UnitOfTime::Days => String::from(self.time.to_string() + " Days"),
            _ => {String::from(" ")}
        }
    }
}

pub async fn is_admin(cx: &UpdateWithCx<Message>, cm: &ChatMember) -> bool {
    let admins = cx
        .bot
        .get_chat_administrators(cx.update.chat_id())
        .send()
        .await;
    if admins.unwrap().contains(cm) {
        true
    } else {
        false
    }
}

pub async fn sender_is_admin(cx: &UpdateWithCx<Message>) -> bool {
    let sender = cx
        .bot
        .get_chat_member(cx.update.chat_id(), cx.update.from().unwrap().id)
        .send()
        .await;
    is_admin(cx, &sender.unwrap()).await
}

pub fn is_private(cx: &UpdateWithCx<Message>) -> bool {
    match cx.update.chat.kind {
        ChatKind::Private(_) => true,
        ChatKind::Public(_) => false,
        _ => true,
    }
}

pub fn create_unmute_perms() -> ChatPermissions {
    let mut perm = ChatPermissions::new();
    perm.can_send_other_messages = Option::from(true);
    perm.can_send_media_messages = Option::from(true);
    perm.can_add_web_page_previews = Option::from(true);
    perm.can_send_media_messages = Option::from(true);
    perm.can_change_info = Option::from(true);
    perm.can_invite_users = Option::from(true);
    perm.can_pin_messages = Option::from(true);
    perm.can_send_polls = Option::from(true);
    perm
}