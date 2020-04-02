use crate::SchedulerKey;
use chrono::{DateTime, Utc};
use white_rabbit::{Duration, DateResult};
use serenity::{
    prelude::*,
    model::id::ChannelId,
};

#[derive(Debug, Clone)]
pub struct Event {
    pub title: String,
    pub description: String,
    pub url: String,
    pub format: String,
    pub start: DateTime<Utc>,
    pub duration: Duration,
    pub logo: String,
}

use serenity::builder::CreateEmbed;

pub fn add_reminder_for_event(ctx: &Context, event: &Event, duration_before: Duration, channel: ChannelId, message: String, embed_event: bool) {
    let scheduler = {
        let mut ctx = ctx.data.write();
        ctx.get_mut::<SchedulerKey>().expect("Expected Scheduler.").clone()
    };
    let mut scheduler = scheduler.write();
    let http = ctx.http.clone();
    let embed : CreateEmbed = event.clone().into();
    scheduler.add_task_datetime(event.start - duration_before, move |_| {
         channel.say(&http, &message).unwrap(); 
         if embed_event {
            channel.send_message(&http, |m| {m.embed(|e| {*e = embed.clone(); e})}).unwrap();
         }
         DateResult::Done
        });
}

pub fn add_default_reminders_for_event(ctx: &Context, event: &Event, channel: ChannelId) {
    if Utc::now() <= event.start - Duration::days(3) { //Only remind if the reminder is not already in the past
        add_reminder_for_event(ctx, event, Duration::days(3), channel, 
            format!("Friendly reminder that we will participate at the {} in 3 days!", event.title).to_string(), true);
    }
    if Utc::now() <= event.start - Duration::days(1) {
        add_reminder_for_event(ctx, event, Duration::days(1), channel, 
            format!("{} will start tomrrow!", event.title).to_string(), true);
    }
    if Utc::now() <= event.start - Duration::hours(3) {
        add_reminder_for_event(ctx, event, Duration::hours(3), channel, 
            format!("Kind reminder that {} is 3 hours away", event.title).to_string(), true);
    }
    if Utc::now() <= event.start - Duration::hours(1) {
        add_reminder_for_event(ctx, event, Duration::hours(1), channel, 
            format!("{} will start in 1 hour, get ready to pwn!", event.title).to_string(), true);
    }
    if Utc::now() <= event.start - Duration::seconds(3) {
        add_reminder_for_event(ctx, event, Duration::seconds(3), channel, 
            format!("{} starts in 3", event.title).to_string(), false);
        
    }
    if Utc::now() <= event.start - Duration::seconds(2) {
        add_reminder_for_event(ctx, event, Duration::seconds(2), channel, 
            format!("2").to_string(), false);
        
    }
    if Utc::now() <= event.start - Duration::seconds(1) {
        add_reminder_for_event(ctx, event, Duration::seconds(1), channel, 
            format!("1").to_string(), false);
        
    }
    if Utc::now() <= event.start - Duration::seconds(0) {
        add_reminder_for_event(ctx, event, Duration::seconds(0), channel, 
            format!("{} started, go go go! Let'z Pwn!", event.title).to_string(), true);
        
    }
    if Utc::now() <= event.start + event.duration {
        add_reminder_for_event(ctx, event, -event.duration, channel, 
            format!("Good job everyone, {} should be over. If you solved some interesting challenges, don't forget to post your writeups in our github repo: https://github.com/LetzPwn/ctf-writeups", event.title).to_string(), false);
    }
}