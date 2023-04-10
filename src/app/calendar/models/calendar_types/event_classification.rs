/*
Format Definition:  This property is defined by the following
      notation:

       class      = "CLASS" classparam ":" classvalue CRLF

       classparam = *(";" other-param)

       classvalue = "PUBLIC" / "PRIVATE" / "CONFIDENTIAL" / iana-token
                  / x-name
       ;Default is PUBLIC

   Example:  The following is an example of this property:

       CLASS:PUBLIC
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EventClassification {
    Public,
    Private,
    Confidential,
}
