//! # A parser definition for weatherflow/tempest json records.
//!
//! This library is the result of expanding a simple UDP packet
//! monitor written to help diagnose connection issues with a
//! `WeatherFlow` Tempest weather station. UDP packets from the
//! hub contain JSON formatted strings. The same JSON format
//! is used in the results from the cloud REST API.
//!
//! All fields are specified as public to enable this library
//! to be used as a connector for other services.
//!
//! ## Example
//! ```rust
//! use libtempest::Tempest;
//! use serde_json;
//! let buf = r#"
//! {
//!    "serial_number": "SK-00008453",
//!    "type":"evt_precip",
//!    "hub_sn": "HB-00000001",
//!    "evt":[1493322445]
//! }"#;
//! let rec: Tempest = serde_json::from_str(&buf).unwrap();
//! if let Tempest::EvtPrecip(x) = &rec {
//!     println!("{:?}", x);
//!     println!("{:?}", x.serial_number);
//!     println!("{:?}", x.evt);
//!     println!("{:?}", x.evt.epoch);
//! }
//! ```
//! ## References
//! - [`WeatherFlow UDP`](https://weatherflow.github.io/Tempest/api/udp/v171/)

use serde::{Deserialize, Serialize};

/// Top level abstraction using serde tag feature to select
/// enum varient based on the value of the JSON `type` field.
///
/// The
/// varient names directly map to the type names with
/// `snake_case` conversion.
/// # Example
///
/// ```rust
/// use libtempest::Tempest;
/// use serde_json;
/// let buf = r#"
/// {
///      "serial_number": "SK-00008453",
///      "type":"rapid_wind",
///      "hub_sn": "HB-00000001",
///      "ob":[1493322445,2.3,128]
/// }"#;
/// let rec: Tempest = serde_json::from_str(&buf).unwrap();
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tempest {
    /// Rain Start Event [type = evt_precip]
    EvtPrecip(EvtPrecip),
    /// Lightning Strike Event [type = evt_strike]
    EvtStrike(EvtStrike),
    /// Rapid Wind [type = rapid_wind]
    RapidWind(RapidWind),
    /// Observation (AIR) [type = obs_air]
    ObsAir(ObsAir),
    /// Observation (Sky) [type = obs_sky]
    ObsSky(ObsSky),
    /// Observation (Tempest) [type = obs_st]
    ObsSt(ObsSt),
    /// Status (device) [type = device_status]
    DeviceStatus(DeviceStatus),
    /// Status (hub) [type = hub_status]
    HubStatus(HubStatus),
}

/// Structure defining the [Rain Start Event] enum
/// varient.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EvtPrecip {
    pub serial_number: String, // SK-00008453
    pub hub_sn: String,        // HB-0000001
    pub evt: EvtPrecipEvt,     // [1493322445]
}

/// Structure defining the [Lightning Strike] enum
/// varient.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EvtStrike {
    pub serial_number: String, // SK-00008453
    pub hub_sn: String,        // HB-0000001
    pub evt: EvtStrikeEvt,     // [1493322445,27,3848]
}

/// Structure defining the [Rapid Wind] enum
/// varient.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct RapidWind {
    pub serial_number: String, // "ST-00028405"
    pub hub_sn: String,        // "HB-00027548"
    pub ob: RapidWindOb,       // [1635567982,1.15,6]
}

/// Structure defining the [Air Observation] enum
/// varient.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ObsAir {
    pub serial_number: String, // "ST-00028405"
    pub hub_sn: String,        // "HB-00027548"
    pub obs: Vec<ObsAirObs>,   // [[1493164835,835.0,10.0,45,0,0,3.46,1]]
    pub firmware_revision: u8, // 17
}

/// Structure defining the [Sky Observation] enum
/// varient.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ObsSky {
    pub serial_number: String, // "SK-00008453"
    pub hub_sn: String,        // "HB-00000001"
    pub obs: Vec<ObsSkyObs>,   // [[1493321340,9000,10,0.0,2.6,4.6,7.4,187,3.12,1,130,null,0,3]]
    pub firmware_revision: u8, // 29
}

/// Structure defining the [Tempest Observation] enum
/// varient
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ObsSt {
    pub serial_number: String,  // "SK-00000512"
    pub hub_sn: String,         // "HB-00013030"
    pub obs: Vec<ObsStObs>, // [[1588948614,0.18,0.22,0.27,144,6,1017.57,22.37,50.26,328,0.03,3,0.00000,0,0,0,2.410,1]]
    pub firmware_revision: u32, // 129
}

/// Structure defining the [Device Status] enum
/// varient.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DeviceStatus {
    pub serial_number: String,  // "AR-00004049"
    pub hub_sn: String,         // "HB-00000001"
    pub timestamp: u64,         // 1510855923
    pub uptime: u32,            // 2189
    pub voltage: f32,           // 3.50
    pub firmware_revision: u32, // 17
    pub rssi: i32,              // -17
    pub hub_rssi: i32,          // -87
    pub sensor_status: u32,     // 0
    pub debug: u32,             // 0
}

/// Structure defining the [Hub Status] enum
/// varient.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct HubStatus {
    pub serial_number: String,     // "HB-00027548"
    pub firmware_revision: String, // 171
    pub uptime: u32,               // 86271
    pub rssi: i32,                 // -29
    pub timestamp: u64,            // 1639424393
    pub reset_flags: String,       // "BOR,PIN,POR"
    pub seq: u32,                  // 8508
    pub fs: Vec<u32>,              // [1,0,15675411,524288] -- internal use
    pub radio_stats: RadioStats,   // [25,1,0,3,17773]
    pub mqtt_stats: Vec<u32>,      // [20,0] -- internal use
}

/// Precipitation event detail.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EvtPrecipEvt {
    pub epoch: u64, // 1635567982 Seconds
}

/// Lightning strike event detail.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EvtStrikeEvt {
    pub epoch: u64,    // 1635567982 Seconds
    pub distance: u16, // km
    pub energy: u16,
}

/// Rapid Wind event detail.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RapidWindOb {
    pub epoch: u64,          // 1635567982 Seconds
    pub wind_speed: f32,     // 1.15 mps
    pub wind_direction: u32, // 6 Degrees
}

/// Air Observation detail.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ObsAirObs {
    pub epoch: u64,                         // 1635567982 Seconds
    pub station_pressure: f32,              // 835.0 MB
    pub air_temperature: f32,               // 10.0 Degrees C
    pub relative_humidity: u32,             // 45 %
    pub lightning_strike_count: u32,        // 0 Km
    pub lightning_strike_avg_distance: u32, // 0 Km
    pub battery: f32,
    pub report_interval: u32, // 1 Minutes
}

/// Sky Observation detail.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ObsSkyObs {
    pub epoch: u64,          // 1635567982 Seconds
    pub illuminance: u32,    // 835.0 MB
    pub uv: u32,             // 10.0 Degrees C
    pub rain_minute: f32,    // 45 %
    pub wind_lull_min3: f32, // 0 Km
    pub wind_avg: f32,       // 0 Km
    pub wind_gust_max3: f32, // 0 Km
    pub wind_direction: u32, // 0 Km
    pub battery: f32,
    pub report_interval: u32, // 1 Minutes
    pub solar_radiation: u32,
    pub rain_day: Option<u32>,
    pub precipitation_type: u8, // 0 = none, 1 = rain, 2 = hail
    pub wind_sample_interval: u32,
}

/// Tempest Observation detail.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ObsStObs {
    pub epoch: u64,          // 1635567982 Seconds
    pub wind_lull_min3: f32, // 0 Km
    pub wind_avg: f32,       // 0 Km
    pub wind_gust_max3: f32, // 0 Km
    pub wind_direction: u32, // 0 Km
    pub wind_sample_interval: u32,
    pub station_pressure: f32,
    pub air_temperature: f32, // degrees C
    pub relative_humidity: f32,
    pub illuminance: u32, // 835.0 MB
    pub uv: f32,          // 10.0 Degrees C
    pub solar_radiation: u32,
    pub rain_minute: f32,       // 45 %
    pub precipitation_type: u8, // 0 = none, 1 = rain, 2 = hail, 3 = rain + hail
    pub lightning_strike_dist: u32,
    pub lightning_strike_count: u32,
    pub battery: f32,
    pub report_interval: u32, // 1 Minutes
}

/// Radio Stats detail.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RadioStats {
    pub version: u32,     // Version [25]
    pub reboots: u32,     // Reboot Count [1]
    pub i2c_errors: u32,  // I2C Bus Error Counts [0]
    pub radio_status: u8, // Radio Status (0 = Radio Off, ...)
    pub network_id: u32,  // Radio Network ID [2839]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evt_precip() {
        let buf = r#"
        {
            "serial_number": "SK-00008453",
            "type":"evt_precip",
	        "hub_sn": "HB-00000001",
	        "evt":[1493322445]
        }"#;
        let deserialized: Tempest = serde_json::from_str(&buf).unwrap();
        let t = Tempest::EvtPrecip(EvtPrecip {
            hub_sn: String::from("HB-00000001"),
            evt: EvtPrecipEvt { epoch: 1493322445 },
            serial_number: String::from("SK-00008453"),
        });
        assert_eq!(t, deserialized);
    }

    #[test]
    fn evt_strike() {
        let buf = r#"
        {
            "serial_number": "AR-00004049",
            "type":"evt_strike",
            "hub_sn": "HB-00000001",
            "evt":[1493322445,27,3848]
        }"#;
        let deserialized: Tempest = serde_json::from_str(&buf).unwrap();
        let t = Tempest::EvtStrike(EvtStrike {
            hub_sn: String::from("HB-00000001"),
            evt: EvtStrikeEvt {
                epoch: 1493322445,
                distance: 27,
                energy: 3848,
            },
            serial_number: String::from("AR-00004049"),
        });
        assert_eq!(t, deserialized);
    }

    #[test]
    fn rapid_wind() {
        let buf = r#"
        {
            "serial_number": "SK-00008453",
            "type": "rapid_wind",
            "hub_sn": "HB-00000001",
            "ob":[1493322445,2.3,128]
        }"#;
        let deserialized: Tempest = serde_json::from_str(&buf).unwrap();
        let t = Tempest::RapidWind(RapidWind {
            hub_sn: String::from("HB-00000001"),
            ob: RapidWindOb {
                epoch: 1493322445,
                wind_speed: 2.3,
                wind_direction: 128,
            },
            serial_number: String::from("SK-00008453"),
        });
        assert_eq!(t, deserialized);
    }

    #[test]
    fn obs_air() {
        let buf = r#"
        {
            "serial_number": "AR-00004049",
            "type":"obs_air",
            "hub_sn": "HB-00000001",
            "obs":[
                [1493164835,835.0,10.0,45,0,0,3.46,1]
            ],
            "firmware_revision": 17
        }"#;
        let deserialized: Tempest = serde_json::from_str(&buf).unwrap();
        let t = Tempest::ObsAir(ObsAir {
            hub_sn: String::from("HB-00000001"),
            obs: vec![ObsAirObs {
                epoch: 1493164835,
                station_pressure: 835.0,
                air_temperature: 10.0,
                relative_humidity: 45,
                lightning_strike_count: 0,
                lightning_strike_avg_distance: 0,
                battery: 3.46,
                report_interval: 1,
            }],
            serial_number: String::from("AR-00004049"),
            firmware_revision: 17,
        });
        assert_eq!(t, deserialized);
    }

    #[test]
    fn obs_sky() {
        let buf = r#"
        {
            "serial_number": "SK-00008453",
            "type":"obs_sky",
            "hub_sn": "HB-00000001",
            "obs":[
                [1493321340,9000,10,0.0,2.6,4.6,7.4,187,3.12,1,130,null,0,3]
            ],
            "firmware_revision": 29
        }"#;
        let deserialized: Tempest = serde_json::from_str(&buf).unwrap();
        let t = Tempest::ObsSky(ObsSky {
            hub_sn: String::from("HB-00000001"),
            obs: vec![ObsSkyObs {
                epoch: 1493321340,
                illuminance: 9000,
                uv: 10,
                rain_minute: 0.0,
                wind_lull_min3: 2.6,
                wind_avg: 4.6,
                wind_gust_max3: 7.4,
                wind_direction: 187,
                battery: 3.12,
                report_interval: 1,
                solar_radiation: 130,
                rain_day: None,
                precipitation_type: 0,
                wind_sample_interval: 3,
            }],
            serial_number: String::from("SK-00008453"),
            firmware_revision: 29,
        });
        assert_eq!(t, deserialized);
    }
    #[test]
    fn obs_st() {
        let buf = r#"
        {
            "serial_number": "AR-00000512",
            "type":"obs_st",
            "hub_sn": "HB-00013030",
            "obs":[
                [1588948614,0.18,0.22,0.27,144,6,1017.57,22.37,50.26,328,0.03,3,0.00000,0,0,0,2.410,1]
            ],
            "firmware_revision": 129
        }"#;
        let deserialized: Tempest = serde_json::from_str(&buf).unwrap();
        let t = Tempest::ObsSt(ObsSt {
            hub_sn: String::from("HB-00013030"),
            obs: vec![ObsStObs {
                epoch: 1588948614,
                wind_lull_min3: 0.18,
                wind_avg: 0.22,
                wind_gust_max3: 0.27,
                wind_direction: 144,
                wind_sample_interval: 6,
                station_pressure: 1017.57,
                air_temperature: 22.37,
                relative_humidity: 50.26,
                illuminance: 328,
                uv: 0.03,
                solar_radiation: 3,
                rain_minute: 0.00000,
                precipitation_type: 0,
                lightning_strike_dist: 0,
                lightning_strike_count: 0,
                battery: 2.410,
                report_interval: 1,
            }],
            serial_number: String::from("AR-00000512"),
            firmware_revision: 129,
        });
        assert_eq!(t, deserialized);
    }
    #[test]
    fn device_status() {
        let buf = r#"
        {
            "serial_number": "AR-00004049",
            "type": "device_status",
            "hub_sn": "HB-00000001",
            "timestamp": 1510855923,
            "uptime": 2189,
            "voltage": 3.50,
            "firmware_revision": 17,
            "rssi": -17,
            "hub_rssi": -87,
            "sensor_status": 0,
            "debug": 0
        }"#;
        let deserialized: Tempest = serde_json::from_str(&buf).unwrap();
        let t = Tempest::DeviceStatus(DeviceStatus {
            serial_number: String::from("AR-00004049"),
            hub_sn: String::from("HB-00000001"),
            timestamp: 1510855923,
            uptime: 2189,
            voltage: 3.50,
            firmware_revision: 17,
            rssi: -17,
            hub_rssi: -87,
            sensor_status: 0,
            debug: 0,
        });
        assert_eq!(t, deserialized);
    }

    #[test]
    fn hub_status() {
        let buf = r#"
        {
            "serial_number":"HB-00000001",
            "type":"hub_status",
            "firmware_revision":"35",
            "uptime":1670133,
            "rssi":-62,
            "timestamp":1495724691,
            "reset_flags": "BOR,PIN,POR",
            "seq": 48,
            "fs": [1, 0, 15675411, 524288],
            "radio_stats": [2, 1, 0, 3, 2839],
            "mqtt_stats": [1, 0]
        }"#;
        let deserialized: Tempest = serde_json::from_str(&buf).unwrap();
        let t = Tempest::HubStatus(HubStatus {
            serial_number: String::from("HB-00000001"),
            firmware_revision: String::from("35"),
            uptime: 1670133,
            rssi: -62,
            timestamp: 1495724691,
            reset_flags: String::from("BOR,PIN,POR"),
            seq: 48,
            fs: vec![1, 0, 15675411, 524288],
            radio_stats: RadioStats {
                version: 2,
                reboots: 1,
                i2c_errors: 0,
                radio_status: 3,
                network_id: 2839,
            },
            mqtt_stats: vec![1, 0],
        });
        assert_eq!(t, deserialized);
    }
}
