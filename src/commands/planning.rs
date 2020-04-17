use serenity::{
    prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::{command},
    },
    model::prelude::*,
};
use std::str::FromStr;
use crate::ctftime;
use crate::planning;



#[command]
#[allowed_roles(admin)]
#[description = "Makes the bot remind users of an upcoming ctf challenge. Pass the ctftime id as parameter!"]
pub fn plan(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let ctftime_id = args.single::<u32>().unwrap();
    let event = ctftime::get_event(ctftime_id)?;

    let plan_event : planning::Event = event.into();
    let channel_id = ChannelId::from_str(planning::EVENT_REMINDER_CHANNEL)?;
    planning::add_default_reminders_for_event(&ctx, &plan_event, channel_id);
    planning::add_event_to_file(planning::EVENT_STORAGE_FILE, &plan_event)?;
    msg.channel_id.send_message(&ctx.http, |m| {m.content("Added the following event to the planning:").embed(|e| {*e = plan_event.into(); e})})?;
    Ok(())
}

#[command]
#[allowed_roles(admin)]
#[description = "Shows all currently planned events"]
pub fn plan_list(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {

    match planning::read_events_from_file(planning::EVENT_STORAGE_FILE) {
        Ok(events) => events.iter().for_each(|event| {
            let plan_event = event.clone();
            msg.channel_id.send_message(&ctx.http, |m| {m.embed(|e| {*e = plan_event.into(); e})});
        }),
        Err(err) => println!("Couldn't read planned_events.json: {}", err)
    }
    Ok(())
}