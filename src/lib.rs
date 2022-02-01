use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tempest {
    EvtPrecip {
        serial_number: String,  // SK-00008453
        hub_sn: String,         // HB-0000001
        evt: RainStartEventObs, // [1493322445]
    },
    EvtStrike {
        serial_number: String,        // SK-00008453
        hub_sn: String,               // HB-0000001
        evt: LightningStrikeEventObs, // [1493322445,27,3848]
    },
    RapidWind {
        serial_number: String, // "ST-00028405"
        hub_sn: String,        // "HB-00027548"
        ob: RapidWindOb,       // [1635567982,1.15,6]
    },
    ObsAir {
        serial_number: String,       // "ST-00028405"
        hub_sn: String,              // "HB-00027548"
        obs: Vec<ObservationAirObs>, // [[1493164835,835.0,10.0,45,0,0,3.46,1]]
        firmware_revision: u8,       // 17
    },
    ObsSky {
        serial_number: String,       // "SK-00008453"
        hub_sn: String,              // "HB-00000001"
        obs: Vec<ObservationSkyObs>, // [[1493321340,9000,10,0.0,2.6,4.6,7.4,187,3.12,1,130,null,0,3]]
        firmware_revision: u8,       // 29
    },
    ObsSt {
        serial_number: String,      // "SK-00000512"
        hub_sn: String,             // "HB-00013030"
        obs: Vec<ObservationStObs>, // [[1588948614,0.18,0.22,0.27,144,6,1017.57,22.37,50.26,328,0.03,3,0.00000,0,0,0,2.410,1]]
        firmware_revision: u8,      // 129
    },
    DeviceStatus {
        serial_number: String,  // "AR-00004049"
        hub_sn: String,         // "HB-00000001"
        timestamp: u64,         // 1510855923
        uptime: u32,            // 2189
        voltage: f32,           // 3.50
        firmware_revision: u32, // 17
        rssi: i32,              // -17
        hub_rssi: i32,          // -87
        sensor_status: u32,     // 0
        debug: u32,             // 0
    },
    HubStatus {
        serial_number: String,     // "HB-00027548"
        firmware_revision: String, // 171
        uptime: u32,               // 86271
        rssi: i32,                 // -29
        timestamp: u64,            // 1639424393
        reset_flags: String,       // "BOR,PIN,POR"
        seq: u32,                  // 8508
        fs: Vec<u32>,              // [1,0,15675411,524288] -- internal use
        radio_stats: Vec<u32>,     // [25,1,0,3,17773]
        mqtt_stats: Vec<u32>,      // [20,0] -- internal use
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RainStartEventObs {
    pub epoch: u64, // 1635567982 Seconds
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LightningStrikeEventObs {
    pub epoch: u64,    // 1635567982 Seconds
    pub distance: u16, // km
    pub energy: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RapidWindOb {
    pub epoch: u64,          // 1635567982 Seconds
    pub wind_speed: f32,     // 1.15 mps
    pub wind_direction: u32, // 6 Degrees
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ObservationAirObs {
    pub epoch: u64,                         // 1635567982 Seconds
    pub station_pressure: f32,              // 835.0 MB
    pub air_temperature: f32,               // 10.0 Degrees C
    pub relative_humidity: u32,             // 45 %
    pub lightning_strike_count: u32,        // 0 Km
    pub lightning_strike_avg_distance: u32, // 0 Km
    pub battery: f32,
    pub report_interval: u32, // 1 Minutes
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ObservationSkyObs {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ObservationStObs {
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
        let t = Tempest::EvtPrecip {
            hub_sn: "HB-00000001".to_string(),
            evt: RainStartEventObs { epoch: 1493322445 },
            serial_number: "SK-00008453".to_string(),
        };
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
        let t = Tempest::EvtStrike {
            hub_sn: "HB-00000001".to_string(),
            evt: LightningStrikeEventObs {
                epoch: 1493322445,
                distance: 27,
                energy: 3848,
            },
            serial_number: "AR-00004049".to_string(),
        };
        assert_eq!(t, deserialized);
    }

    #[test]
    fn rapid_wind() {
        let buf = r#"
        {
            "serial_number":"ST-00028405",
            "type":"rapid_wind",
            "hub_sn":"HB-00027548",
            "ob":[1639426004,1.15,6]
        }"#;
        let deserialized: Tempest = serde_json::from_str(&buf).unwrap();
        let t = Tempest::RapidWind {
            hub_sn: "HB-00027548".to_string(),
            ob: RapidWindOb {
                epoch: 1639426004,
                wind_speed: 1.15,
                wind_direction: 6,
            },
            serial_number: "ST-00028405".to_string(),
        };
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
        let t = Tempest::ObsAir {
            hub_sn: "HB-00000001".to_string(),
            obs: vec![ObservationAirObs {
                epoch: 1493164835,
                station_pressure: 835.0,
                air_temperature: 10.0,
                relative_humidity: 45,
                lightning_strike_count: 0,
                lightning_strike_avg_distance: 0,
                battery: 3.46,
                report_interval: 1,
            }],
            serial_number: "AR-00004049".to_string(),
            firmware_revision: 17,
        };
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
        let t = Tempest::ObsSky {
            hub_sn: "HB-00000001".to_string(),
            obs: vec![ObservationSkyObs {
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
            serial_number: "SK-00008453".to_string(),
            firmware_revision: 29,
        };
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
        let t = Tempest::ObsSt {
            hub_sn: "HB-00013030".to_string(),
            obs: vec![ObservationStObs {
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
            serial_number: "AR-00000512".to_string(),
            firmware_revision: 129,
        };
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
        let t = Tempest::DeviceStatus {
            serial_number: "AR-00004049".to_string(),
            hub_sn: "HB-00000001".to_string(),
            timestamp: 1510855923,
            uptime: 2189,
            voltage: 3.50,
            firmware_revision: 17,
            rssi: -17,
            hub_rssi: -87,
            sensor_status: 0,
            debug: 0,
        };
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
        let t = Tempest::HubStatus {
            serial_number: "HB-00000001".to_string(),
            firmware_revision: "35".to_string(),
            uptime: 1670133,
            rssi: -62,
            timestamp: 1495724691,
            reset_flags: "BOR,PIN,POR".to_string(),
            seq: 48,
            fs: vec![1, 0, 15675411, 524288],
            radio_stats: vec![2, 1, 0, 3, 2839],
            mqtt_stats: vec![1, 0],
        };
        assert_eq!(t, deserialized);
    }
}
