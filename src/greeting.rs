use serenity::{
    prelude::*,
    model::prelude::*,
};

pub fn greet_new_member(ctx: &Context, new_member: &Member) {
    let user = new_member.user.read();
    let geeting = format!(r#"
__Lëtzebuergesch__
Moien **{}**!
Gratulatioun, du hues et an d'LetzPwn Communautéit gepackt. Fill dech wéi doheem! Ech wäert dech am <#570926538091134978> um Courant halen wéini en neien CTF Event stattfënnt wou ma Interessi hunn deelzehuelen.
Bis dohinner kanns de dech jo am <#637283250422808588> kuerz virstellen.
Natierlech kanns de dech awer och an all aner Kanal mellen wanns de einfach nëmmen quaselen wëlls oder awer och wanns de Froen iwwert Hacking/CTF's 
hues.

Ech kann och nach puer aner Saachen, schreif einfach `!help` an ech soen der wat  ech nach alles maachen kann.

__English__
Hello **{}**!
Gratulation, you're now part of the LetzPwn community. I will notify the members in <#570926538091134978> about upcoming CTF events.
In the meantime you can present yourself in the <#637283250422808588> channel. If you have questions or just want to talk, feel free to write in any of the other channels. I can also do soe other stuff, just write `!help` to see what I can do for you!
"#, new_member.display_name(), new_member.display_name());
    user.direct_message(ctx, |m| {m.content(geeting)}).unwrap();
}