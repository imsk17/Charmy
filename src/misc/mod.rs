use teloxide::prelude::{Request, UpdateWithCx};
use teloxide::requests::ResponseResult;
use teloxide::types::Message;

use crate::utils;

#[warn(unused_must_use)]
pub async fn start(cx: &UpdateWithCx<Message>) -> ResponseResult<()> {
    if utils::is_private(cx) {
        cx
            .reply_to("Hi, I am Charmy. A friendly group management, La!. You can know about my commands by using /help. My master is @imsk17.")
            .send()
            .await?;
    } else {
        cx
            .reply_to("I am Listening, La.")
            .send()
            .await?;
    };
    Ok(())
}

pub async fn help(cx: &UpdateWithCx<Message>, text: String) -> ResponseResult<()> {
    if utils::is_private(cx) {
        let _ = cx
            .answer(text)
            .send()
            .await?;
    } else {
        let _ = cx
            .reply_to("For Help, Ask it Directly in the bot private chat.")
            .send()
            .await;
    };
    Ok(())
}