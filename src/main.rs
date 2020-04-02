mod commands;
mod ctftime;
mod embeds;
mod encryption;
mod greeting;
mod planning;

use std::{collections::HashSet, env, sync::Arc,};
use serenity::{
    prelude::*,
    framework::standard::{
        Args, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, StandardFramework,
        macros::{group, help},
    },
    model::prelude::*,
    model::id::ChannelId
};

// And this crate to schedule our tasks.
use white_rabbit::{Scheduler};
use commands::{ctftime::*, encryption::*, stealer::*, dnsrebind::*};
use greeting::*;
use planning::*;

use chrono::{Utc, Duration,TimeZone};
use std::str::FromStr;

struct SchedulerKey;
impl TypeMapKey for SchedulerKey {
    type Value = Arc<RwLock<Scheduler>>;
}

struct Handler;
impl EventHandler for Handler {
  
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

//For now add Letz cyber security event manually. Need a better way (Database/Github?) in the future...
        let channel_id = ChannelId::from_str("635136457643786280").unwrap();
        let test_event = planning::Event {
            title: "Lëtz Cybersecurity Challenge 2020".to_string(),
            description: 
"**IMPORTANT: To participate you MUST register again on 03/04/2020 between 16:00 and 18:00 (not earlier and not later)**

The Lëtz Cybersecurity Challenge provides 24 challenges for 24 hours of fun.

__The competition is divided in two parts:__

**Part1**
Start on 03/04/2020 at 18:00 (it might take some time until all challenges will be uploaded)
End on 04/04/2020 at 06:00
Challenges in all categories of low and medium complexity

**Part2**
Start on 04/04/2020 at 12:00 (noon) (it might take some time until all challenges will be uploaded)
End on 04/04/2020 at 24:00 (midnight)
Challenges in all categories of medium and higher complexity

**Flag format**
LCSC{FLAG-TO-FIND}
The flags are cases insensitive: LCSC{FLAG-TO-FIND} is equivalent to 
LCSC{FLaG-To-Find}
But: it might be that you will find flags in format VSC{FLAG-TO-FIND}. You just have to replace VSC by LCSC.
".to_string(),
            duration: Duration::hours(30),
            format: "Jeopardy".to_string(),
            logo: "https://ctf.cybersecuritychallenge.lu/files/245f6f5845e9a891f5bc7a071fd7f0c3/CSChallenge-2020.png".to_string(),
            start: Utc.ymd(2020, 04, 03).and_hms(16, 00, 0),
            url: "https://2020.cybersecuritychallenge.lu/register".to_string()

            
        };
        add_default_reminders_for_event(&ctx, &test_event, channel_id);
    }

    fn guild_member_addition(
        &self,
        ctx: Context,
        _guild_id: GuildId,
        new_member: Member
    ) {
        greet_new_member(&ctx, &new_member);
    }
}

#[group]
#[commands(event, upcoming, b64e, b64d, cookiestealer, dnsstealer, dnsrebind)]
struct General;

#[help]
// `{}` refers to a command's name.
#[command_not_found_text = "Could not find: `{}`."]
// Define the maximum Levenshtein-distance between a searched command-name
// and commands. If the distance is lower than or equal the set distance,
// it will be displayed as a suggestion.
// Setting the distance to 0 will disable suggestions.
#[max_levenshtein_distance(3)]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, &help_options, groups, owners)
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect(
        "Expected a token in the environment",
    );
    let mut client = Client::new(&token, Handler)
        .expect("Err creating client");

    {
        let mut data = client.data.write();
        // We create a new scheduler with 4 internal threads. Why 4? It really
        // is just an arbitrary number, you are often better setting this
        // based on your CPU.
        // When a task is due, a thread from the threadpool will be used to
        // avoid blocking the scheduler thread.
        let scheduler = Scheduler::new(4);
        let scheduler = Arc::new(RwLock::new(scheduler));

        data.insert::<SchedulerKey>(scheduler);
    }

    // We will fetch your bot's id.
    let bot_id = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            info.id
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    client.with_framework(
        // Configures the client, allowing for options to mutate how the
        // framework functions.
        StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("!"))
        .on_dispatch_error(|ctx, msg, error| {
            if let DispatchError::Ratelimited(seconds) = error {
                let _ = msg.channel_id.say(&ctx.http, &format!("Try this again in {} seconds.", seconds));
            }
        })
        .after(|_ctx, _msg, cmd_name, error| {
            if let Err(why) = error {
                println!("Error in {}: {:?}", cmd_name, why);
            }
        })
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}