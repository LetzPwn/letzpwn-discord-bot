use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use failure::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Duration {
    pub hours: u32,
    pub days: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub onsite: bool,
    pub title: String,
    pub description: String,
    pub weight: f32,
    pub url: String,
    pub restrictions: String,
    pub format: String,
    pub start: DateTime<Utc>,
    pub participants: u32,
    pub duration: Duration,
    pub logo: String,
}

pub fn get_event(event_id : u32) -> Result<Event, Error> {
    let request_url = format!("https://ctftime.org/api/v1/events/{}/", event_id);
    let mut response = reqwest::get(&request_url)?;
    let event: Event = response.json()?;
    Ok(event)
}

pub fn get_upcoming_events(limit : usize) -> Result<Vec<Event>, Error> {
    let request_url = format!("https://ctftime.org/api/v1/events/?limit=50");
    let mut response = reqwest::get(&request_url)?;
    let events: Vec<Event> = response.json()?;
    let onsite_events: Vec<Event> = events.into_iter().filter(|e| !e.onsite).take(limit).collect();
    Ok(onsite_events)
}
