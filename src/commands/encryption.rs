use failure::Error;
use serenity::{
    prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::{command},
    },
    model::prelude::*,
};
use failure::ResultExt;

#[command]
#[description = "Encodes to base64"]
pub fn b64e(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut do_steps = || -> Result<(), Error> {
        let string = args.single::<String>().context("Wrong parameters")?;
        let encoded = base64::encode(&string);
        let message = format!("`{}`", encoded);
        msg.channel_id.say(&ctx.http, message).unwrap();
        Ok(())
    };
    if let Err(err) = do_steps() {
        msg.channel_id.say(&ctx.http, err).unwrap();
    }
    Ok(())
}

#[command]
#[description = "Decodes b64"]
pub fn b64d(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut do_steps = || -> Result<(), Error> {
        let mut string = args.single::<String>().context("Wrong parameters")?;
        string.retain(|c| !c.is_whitespace());
        let bytes = base64::decode(&string).context("Not valid base64")?;
        let decoded = std::str::from_utf8(&bytes[..]).context("Valid base64, but doesn't contain a UTF-8 String")?;
        let message = format!("`{}`", decoded);
        msg.channel_id.say(&ctx.http, message).unwrap();
        Ok(())
    };
    if let Err(err) = do_steps() {
        msg.channel_id.say(&ctx.http, err).unwrap();
    }
    Ok(())
}
