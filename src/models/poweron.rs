use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PowerOnResponse {
    pub uri: Option<String>,           // V1 V2 V3
    pub place_id: Option<String>,      // V1 V2 V3 V5
    pub host: Option<String>,          // V1 V2 V3
    pub stat: Option<u32>,             // V1 V2 V3
    pub name: Option<String>,          // v1 V2 V3 V5
    pub nickname: Option<String>,      // V1 V2 V3 V5
    pub year: Option<String>,          // V1 V2 V3
    pub month: Option<String>,         // V1 V2 V3
    pub day: Option<String>,           // V1 V2 V3
    pub hour: Option<String>,          // V1 V2 V3
    pub minute: Option<String>,        // V1 V2 V3
    pub second: Option<String>,        // V1 V2 V3
    pub setting: Option<String>,       // V1 V2 V3 V5
    pub region0: Option<String>,       // V1 V2 V3 V5
    pub region_name0: Option<String>,  // V1 V2 V3 V5
    pub region_name1: Option<String>,  // V1 V2 V3 V5
    pub region_name_2: Option<String>, // V1 V2 V3 V5
    pub region_name_3: Option<String>, // V1 V2 V3 V5
    pub country: Option<String>,       // V2 V3 V5
    pub timezone: Option<String>,      // V2
    pub res_class: Option<String>,     // V2
    pub res_ver: Option<u32>,          // V3
    pub allnet_id: Option<i8>,         // V3 V5
    pub utc_time: Option<String>,      // V3 V5
    pub token: Option<String>,         // V3 V5
}

