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
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String
}

#[command]
#[description = "Create a dns logger"]
pub fn dnsstealer(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let answer = "Use the following url to create a dns stealer:\nhttp://requestbin.net/dns";
    msg.channel_id.say(&ctx.http, answer).unwrap();
    Ok(())
}

#[command]
#[description = "Create a temporary http logger (can be used for cookie stealing fi)"]
pub fn cookiestealer(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let do_steps = || -> Result<(), Error> {
        let client = reqwest::Client::new();
        let mut response = client.post("http://requestbin.net/api/v1/bins").send()
            .context("Error contacting requestbin.net")?;
        let data: Data = response.json().context("requestbin.net returned wrong json")?;
        let url = format!("http://requestbin.net/r/{}", data.name);
        let inspect_url = format!("http://requestbin.net/r/{}?inspect", data.name);
        let answer = format!("Use the following url to send request to:\n{}\nUse the following url for viewing the logs:\n{}", url, inspect_url);
        msg.channel_id.say(&ctx.http, answer).unwrap();
        Ok(())
    };
    if let Err(err) = do_steps() {
        msg.channel_id.say(&ctx.http, err).unwrap();
    }
    Ok(())
}
