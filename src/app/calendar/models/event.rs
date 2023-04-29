/*
BEGIN:VEVENT
DTSTAMP: ; (Required, MUST NOT occur more than once)
UID: ; (Required, MUST NOT occur more than once)
DTSTART: ; (Required, MUST NOT occur more than once)
CLASS: ; (Optional, MUST NOT occur more than once)
CREATED: ; (Optional, MUST NOT occur more than once)
DESCRIPTION: ; (Optional, MUST NOT occur more than once)
GEO: ; (Optional, MUST NOT occur more than once)
LAST-MODIFIED: ; (Optional, MUST NOT occur more than once)
LOCATION: ; (Optional, MUST NOT occur more than once)
ORGANIZER: ; (Optional, MUST NOT occur more than once)
PRIORITY: ; (Optional, MUST NOT occur more than once)
SEQUENCE: ; (Optional, MUST NOT occur more than once)
STATUS: ; (Optional, MUST NOT occur more than once)
SUMMARY: ; (Optional, MUST NOT occur more than once)
TRANSP: ; (Optional, MUST NOT occur more than once)
URL: ; (Optional, MUST NOT occur more than once)
RECURRENCE-ID: ; (Optional, MUST NOT occur more than once)
RRULE: ; (Optional, MUST NOT occur more than once)
DTEND: ; (Optional, MUST NOT occur more than once if DURATION is present)
DURATION: ; (Optional, MUST NOT occur more than once if DTEND is present)
ATTACH: ; (Optional, MAY occur more than once)
ATTENDEE: ; (Optional, MAY occur more than once)
CATEGORIES: ; (Optional, MAY occur more than once)
COMMENT: ; (Optional, MAY occur more than once)
CONTACT: ; (Optional, MAY occur more than once)
EXDATE: ; (Optional, MAY occur more than once)
RSTATUS: ; (Optional, MAY occur more than once)
RELATED-TO: ; (Optional, MAY occur more than once)
RESOURCES: ; (Optional, MAY occur more than once)
RDATE: ; (Optional, MAY occur more than once)
END:VEVENT
*/

pub mod event_builder;

use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::calendar_types::{
    attendee::Attendee,
    event_classification::EventClassification,
    event_status::EventStatus,
    geolocation::GeoLocation,
    recurrence::{date::Date, Recurrence},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    /* Event start date */
    start: Date,

    /* Event end date */
    end: Option<Date>,

    recurrence: Option<Recurrence>,

    /* Event end date including recurrence */
    recurrence_end: Date,

    description: Option<String>,

    summary: Option<String>,

    classification: Option<EventClassification>,

    geo_location: Option<GeoLocation>,

    location: Option<String>,

    organizer: Option<String>,

    /* Priority number, from 0 to 9 */
    priority: Option<u8>,

    /* Event version, incremented on each update */
    version: u32,

    status: Option<EventStatus>,

    transparent: bool,

    url: Option<String>,

    attachments: Option<Vec<String>>,

    attendees: Option<Vec<Attendee>>,

    categories: Option<Vec<String>>,

    comments: Option<Vec<String>>,

    contacts: Option<Vec<String>>,
}

impl Event {
    #[allow(clippy::too_many_arguments)]
    fn new(
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
    ) -> Self {
        let recurrence_end = Date::new(Utc::now());
        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            start,
            end,
            recurrence_end,
            description,
            summary,
            classification,
            geo_location,
            location,
            organizer,
            priority,
            version: 0,
            status,
            transparent,
            url,
            attachments,
            attendees,
            categories,
            comments,
            contacts,
            recurrence,
        }
    }

    /*fn update(&mut self) {
        self.updated_at = Utc::now();
        self.version += 1;
    }*/
}
