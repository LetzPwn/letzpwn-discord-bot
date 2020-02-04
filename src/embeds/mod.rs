use chrono::DateTime;
use chrono::Local;
use serenity::builder::CreateEmbed;
use crate::ctftime;

impl From<ctftime::Event> for CreateEmbed {
    fn from(event: ctftime::Event) -> Self {
        let footer_text = format!("{} | Weight: {} | Participants: {}",
            event.format, event.weight, event.participants);
        let duration_text = format!("{} days, {} hours", event.duration.days, event.duration.hours);
        let local_time : DateTime<Local>  = event.start.into();
        let mut e = CreateEmbed::default();

        e.title(&event.title);
        e.description(&event.description);
        e.url(&event.url);
        e.color(14038402);
        e.thumbnail(&event.logo);

        e.fields(vec![
            ("Date", &local_time.format("%e %B %Y at %H:%M").to_string(), true),
            ("Duration", &duration_text, true),
        ]);

        e.footer(|f| {
            f.text(footer_text);
            f
        });

        e
    }
}
