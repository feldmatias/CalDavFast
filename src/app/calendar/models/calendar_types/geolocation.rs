/*
Format Definition:  This property is defined by the following
      notation:

       geo        = "GEO" geoparam ":" geovalue CRLF

       geoparam   = *(";" other-param)

       geovalue   = float ";" float
       ;Latitude and Longitude components

   Example:  The following is an example of this property:

       GEO:37.386013;-122.082932
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoLocation {
    latitude: f64,
    longitude: f64,
}
