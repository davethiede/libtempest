use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum Tempest {
    #[serde(rename="hub_status")]
    HS(HubStatus),

    #[serde(rename = "rapid_wind")]
    RW(RapidWind),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HubStatus {
    pub serial_number: String,  // "HB-00027548"
    pub firmware_revision: u32, // 171
    pub uptime: u32,            // 86271
    pub rssi: i32,              // -29
    pub timestamp: u64,         // 1639424393
    pub reset_flags: String,    // "BOR,PIN,POR"
    pub seq: u32,               // 8508
    pub fs: Vec<u32>,           // [1,0,15675411,524288]
    pub radio_status: Vec<u32>, // [25,1,0,3,17773]
    pub mqtt_stats: Vec<u32>,   // [20,0]
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RapidWind {
    pub hub_sn: String,        // "HB-00027548"
    pub ob: (u64, f32, u64),   // [1635567982,1.15,6]
    pub serial_number: String, // "ST-00028405"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rapid_wind() {
        let buf = r#"
        {
            "serial_number":"ST-00028405",
            "type":"rapid_wind",
            "hub_sn":"HB-00027548",
            "ob":[1639426004,1.15,6]
        }"#;
        let _deserialized: Tempest = serde_json::from_str(&buf).unwrap();
        let t = Tempest::RW(RapidWind {
            hub_sn: "HB-00027548".to_string(),
            ob: (1639426004, 1.15, 6),
            serial_number: "ST-00028405".to_string(),
        });
        assert_eq!(t, _deserialized);
    }
}
