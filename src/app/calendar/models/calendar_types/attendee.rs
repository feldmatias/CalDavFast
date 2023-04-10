/*
Here are the possible values for the PARTSTAT parameter of the ATTENDEE property in iCalendar:

ACCEPTED: Indicates that the attendee has accepted the invitation.
DECLINED: Indicates that the attendee has declined the invitation.
TENTATIVE: Indicates that the attendee has tentatively accepted the invitation.
DELEGATED: Indicates that the attendee has delegated their attendance to another person.
NEEDS-ACTION: Indicates that the attendee has not yet responded to the invitation.
Here are the possible values for the CUTYPE parameter:

INDIVIDUAL: Indicates that the attendee is an individual person.
GROUP: Indicates that the attendee is a group of people, such as a mailing list.
RESOURCE: Indicates that the attendee is a physical resource, such as a meeting room or equipment.
ROOM: Indicates that the attendee is a room or other physical space.
And here are the possible values for the ROLE parameter:

CHAIR: Indicates that the attendee is the chairperson or organizer of the event.
REQ-PARTICIPANT: Indicates that the attendee is a required participant in the event.
OPT-PARTICIPANT: Indicates that the attendee is an optional participant in the event.
NON-PARTICIPANT: Indicates that the attendee is not expected to participate in the event, but is being kept informed.
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AttendeeStatus {
    Accepted,
    Declined,
    Tentative,
    Delegated,
    NeedsAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AttendeeType {
    Individual,
    Group,
    Resource,
    Room,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AttendeeRole {
    Chair,
    ReqParticipant,
    OptParticipant,
    NonParticipant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attendee {
    pub name: String,
    pub email: String,
    pub status: AttendeeStatus,
    pub attendee_type: AttendeeType,
    pub attendee_role: AttendeeRole,
}
