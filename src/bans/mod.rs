#[warn(unused_must_use)]
use teloxide::prelude::UpdateWithCx;
use teloxide::requests::{Request, ResponseResult};
use teloxide::types::{ChatPermissions, Message};

use std::str::FromStr;
use crate::bans::utils::{RestrictTime, ToSeconds};

use crate::utils;

pub async fn kick_user(cx: &UpdateWithCx<Message>) -> ResponseResult<()> {
    if utils::is_private(cx) {
        cx.reply_to("This command is meant to be used in group chats only.")
            .send()
            .await?;
    } else {
        match cx.update.reply_to_message() {
            Some(mess) => {
                if !utils::sender_is_admin(cx).await {
                    cx.reply_to("Weakness Disgusts Me! And, you are weak.")
                        .send()
                        .await?;
                    return Ok(());
                }
                let person_to_kick = cx
                    .bot
                    .get_chat_member(cx.update.chat_id(), mess.from().unwrap().id)
                    .send()
                    .await?;
                if utils::is_admin(cx, &person_to_kick).await {
                    cx.reply_to("This guy is an admin just like you. Find a peasant or forget it!")
                        .send()
                        .await?;
                } else {
                    cx.bot
                        .kick_chat_member(cx.update.chat_id(), mess.from().unwrap().id)
                        .send()
                        .await?;
                    cx.reply_to(format!("Kicked @{}", person_to_kick.user.username.unwrap()))
                        .send()
                        .await?;
                }
            }
            None => {
                cx.reply_to("Use this command in reply to another message")
                    .send()
                    .await?;
            }
        }
    }
    Ok(())
}

pub async fn ban_user(cx: &UpdateWithCx<Message>) -> ResponseResult<()> {
    if utils::is_private(cx) {
        cx.reply_to("This command is meant to be used in group chats only.")
            .send()
            .await?;
    } else {
        match cx.update.reply_to_message() {
            Some(mess) => {
                if !utils::sender_is_admin(cx).await {
                    cx.reply_to("Weakness Disgusts Me! And, you are weak.")
                        .send()
                        .await?;
                    return Ok(());
                }
                let person_to_kick = cx
                    .bot
                    .get_chat_member(cx.update.chat_id(), mess.from().unwrap().id)
                    .send()
                    .await?;
                if cx.update.text().unwrap().split(" ").collect::<Vec<&str>>().len() as i32 <= 1 {
                    cx.bot.kick_chat_member(cx.update.chat.id, mess.from().unwrap().id)
                        .send()
                        .await?;
                    cx.reply_to(format!("Banned @{}", person_to_kick.clone().user.username.unwrap())).send().await?;
                    return Ok(());
                }
                if utils::is_admin(cx, &person_to_kick).await {
                    cx.reply_to("This guy is an admin just like you. Find a peasant or forget it!")
                        .send()
                        .await?;
                } else {
                    let time = RestrictTime::from_str(cx.update.text().unwrap()).unwrap();
                    cx.bot
                        .kick_chat_member(cx.update.chat_id(), mess.from().unwrap().id)
                        .until_date(cx.update.date + time.to_seconds()as i32)
                        .send()
                        .await?;
                    cx.reply_to(format!(
                        "Banned @{} for {} ",
                        person_to_kick.user.username.unwrap(),
                        time.to_string()
                    ))
                        .send()
                        .await?;
                }
            }
            None => {
                cx.reply_to("Use this command in reply to another message")
                    .send()
                    .await?;
            }
        }
    }
    Ok(())
}

pub async fn mute_user(
    cx: &UpdateWithCx<Message>
) -> ResponseResult<()> {
    if utils::is_private(cx) {
        cx.reply_to("This command is meant to be used in group chats only.")
            .send()
            .await?;
    } else {
        match cx.update.reply_to_message() {
            Some(mess) => {
                if !utils::sender_is_admin(cx).await {
                    cx.reply_to("Weakness Disgusts Me! And, you are weak.")
                        .send()
                        .await?;
                    return Ok(());
                }
                let person_to_kick = cx
                    .bot
                    .get_chat_member(cx.update.chat_id(), mess.from().unwrap().id)
                    .send()
                    .await?;
                if cx.update.text().unwrap().split(" ").collect::<Vec<&str>>().len() as i32 <= 1 {
                    cx.bot.restrict_chat_member(cx.update.chat.id, mess.from().unwrap().id, ChatPermissions::default())
                        .send()
                        .await?;
                    cx.reply_to(format!("Muted @{}", person_to_kick.clone().user.username.unwrap())).send().await?;
                    return Ok(());
                }
                if utils::is_admin(cx, &person_to_kick).await {
                    cx.reply_to("This guy is an admin just like you. Find a peasant or forget it!")
                        .send()
                        .await?;
                }
                else {
                    let time =  RestrictTime::from_str(&*cx.update.text_owned().unwrap()).unwrap();
                    cx.bot
                        .restrict_chat_member(
                            cx.update.chat_id(),
                            mess.from().unwrap().id,
                            ChatPermissions::default(),
                        )
                        .until_date(cx.update.date +time.to_seconds() as i32)
                        .send()
                        .await?;
                    cx.reply_to(format!(
                        "Muted @{} for {} ",
                        person_to_kick.user.username.unwrap(),
                        time.to_string()
                    ))
                        .send()
                        .await?;
                }
            }
            None => {
                cx.reply_to("Use this command in reply to another message")
                    .send()
                    .await?;
            }
        }
    }
    Ok(())
}

pub async fn unmute_user(cx: &UpdateWithCx<Message>) -> ResponseResult<()> {
    if utils::is_private(cx) {
        cx.reply_to("This command is meant to be used in group chats only.")
            .send()
            .await?;
    } else {
        let perm = utils::create_unmute_perms();
        match cx.update.reply_to_message() {
            Some(replied) => {
                if utils::sender_is_admin(cx).await {
                    let guy_to_unrestrict = replied.from().unwrap();
                    cx.bot.restrict_chat_member(cx.update.chat_id(), replied.from().unwrap().id, perm).send().await?;
                    cx.reply_to(format!(
                        "Unmuted @{}",
                        guy_to_unrestrict.clone().username.unwrap()
                    ), ).send().await?;
                } else {
                    cx.reply_to("You do not have enough permissions.").send().await?;
                }
            }
            None => { cx.reply_to("Try replying to a message next time bud.").send().await?; }
        }
    }
    Ok(())
}

pub async fn unban_user(cx: &UpdateWithCx<Message>) -> ResponseResult<()> {
    if utils::is_private(cx) {
        cx.reply_to("This command is meant to be used in group chats only.")
            .send()
            .await?;
    } else {
        match cx.update.reply_to_message() {
            Some(replied) => {
                if utils::sender_is_admin(cx).await {
                    let guy_to_unrestrict = replied.from().unwrap();
                    cx.bot.unban_chat_member(cx.update.chat_id(), replied.from().unwrap().id).send().await?;
                    cx.reply_to(format!(
                        "Unbanned @{}",
                        guy_to_unrestrict.clone().username.unwrap()
                    ), ).send().await?;
                } else {
                    cx.reply_to("You do not have enough permissions.").send().await?;
                }
            }
            None => { cx.reply_to("Try replying to a message next time bud.").send().await?; }
        }
    }
    Ok(())
}