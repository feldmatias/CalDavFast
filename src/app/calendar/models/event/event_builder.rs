use crate::app::calendar::models::calendar_types::{
    attendee::Attendee,
    event_classification::EventClassification,
    event_status::EventStatus,
    geolocation::GeoLocation,
    recurrence::{date::Date, Recurrence},
};

use super::Event;

pub struct EventBuilder {
    start: Date,
    end: Option<Date>,
    description: Option<String>,
    summary: Option<String>,
    classification: Option<EventClassification>,
    geo_location: Option<GeoLocation>,
    location: Option<String>,
    organizer: Option<String>,
    priority: Option<u8>,
    status: Option<EventStatus>,
    transparent: bool,
    url: Option<String>,
    attachments: Option<Vec<String>>,
    attendees: Option<Vec<Attendee>>,
    categories: Option<Vec<String>>,
    comments: Option<Vec<String>>,
    contacts: Option<Vec<String>>,
    recurrence: Option<Recurrence>,
}

impl EventBuilder {
    pub fn new(start_date: Date) -> Self {
        Self {
            start: start_date,
            end: None,
            description: None,
            summary: None,
            classification: None,
            geo_location: None,
            location: None,
            organizer: None,
            priority: None,
            status: None,
            transparent: false,
            url: None,
            attachments: None,
            attendees: None,
            categories: None,
            comments: None,
            contacts: None,
            recurrence: None,
        }
    }

    pub fn set_end_date(mut self, end_date: Date) -> Self {
        self.end = Some(end_date);
        self
    }

    pub fn set_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn set_summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn set_classification(mut self, classification: EventClassification) -> Self {
        self.classification = Some(classification);
        self
    }

    pub fn set_geo_location(mut self, geo_location: GeoLocation) -> Self {
        self.geo_location = Some(geo_location);
        self
    }

    pub fn set_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    pub fn set_organizer(mut self, organizer: String) -> Self {
        self.organizer = Some(organizer);
        self
    }

    pub fn set_priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn set_status(mut self, status: EventStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn set_transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    pub fn set_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn set_attachments(mut self, attachments: Vec<String>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn set_attendees(mut self, attendees: Vec<Attendee>) -> Self {
        self.attendees = Some(attendees);
        self
    }

    pub fn set_categories(mut self, categories: Vec<String>) -> Self {
        self.categories = Some(categories);
        self
    }

    pub fn set_comments(mut self, comments: Vec<String>) -> Self {
        self.comments = Some(comments);
        self
    }

    pub fn set_contacts(mut self, contacts: Vec<String>) -> Self {
        self.contacts = Some(contacts);
        self
    }

    pub fn set_recurrence(mut self, recurrence: Recurrence) -> Self {
        self.recurrence = Some(recurrence);
        self
    }

    pub fn build(self) -> Event {
        Event::new(
            self.start,
            self.end,
            self.description,
            self.summary,
            self.classification,
            self.geo_location,
            self.location,
            self.organizer,
            self.priority,
            self.status,
            self.transparent,
            self.url,
            self.attachments,
            self.attendees,
            self.categories,
            self.comments,
            self.contacts,
            self.recurrence,
        )
    }
}
