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
use uuid::Uuid;

#[command]
#[description = "Creates a DNS rebind url. First parameter is the public ip of a server the \
    attacker controls. Second parameter is the private ip of a local server to compromise."]
pub fn dnsrebind(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut do_steps = || -> Result<(), Error> {
        let uuid = Uuid::new_v4();
        let ip1 = args.single::<String>().context("Wrong parameters. See !help dnsrebind")?;
        let ip2 = args.single::<String>().context("Wrong parameters. See !help dnsrebind")?;
        let url = format!("http://A.{}.1time.{}.forever.{}.rebind.network", ip1, ip2, uuid.to_hyphenated());
        //Post the url in a script format. Otherwise Discord will send a bot to visit the url and the
        //dns rebinding will happen too early.
        let answer = format!("Use the following url: `{}`", url);
        msg.channel_id.say(&ctx.http, answer).unwrap();
        Ok(())
    };
    if let Err(err) = do_steps() {
        msg.channel_id.say(&ctx.http, err).unwrap();
    }
    Ok(())
}
