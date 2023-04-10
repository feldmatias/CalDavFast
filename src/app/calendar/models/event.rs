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

use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::calendar_types::{
    attendee::Attendee, event_classification::EventClassification, event_status::EventStatus, geolocation::GeoLocation,
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
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    start: DateTime<Utc>,

    /* Event end date */
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    end: DateTime<Utc>,

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
    // recurrence
}
