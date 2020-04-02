use serenity::{
    prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::{command},
    },
    model::prelude::*,
};
use crate::ctftime;

#[command]
#[description = "Shows a specific CTF event, given its ID."]
pub fn event(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let ctftime_id = args.single::<u32>().unwrap();
    let event = ctftime::get_event(ctftime_id)?;
    msg.channel_id.send_message(&ctx.http, |m| {m.embed(|e| {*e = event.into(); e})})?;
    Ok(())
}

#[command]
#[description = "Lists upcoming CTF events that are not on-site. You can pass the amount of upcoming events as argument (default = 5)."]
pub fn upcoming(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let limit = args.single::<usize>().unwrap_or(5);
    let mut events = ctftime::get_upcoming_events(limit)?;
    for event in events.drain(..) {
        msg.channel_id.send_message(&ctx.http, |m| {m.embed(|e| {*e = event.into(); e})})?;
    }
    Ok(())
}