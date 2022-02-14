# libtempest

## A parser definition for weatherflow/tempest json records.

This library is the result of expanding a simple UDP packet
monitor written to help diagnose connection issues with a
`WeatherFlow` Tempest weather station. UDP packets from the
hub contain JSON formatted strings. The same JSON format
is used in the results from the cloud REST API.

All fields are specified as public to enable this library
to be used as a connector for other services.

### Example
```rust
use libtempest::Tempest;
use serde_json;
let buf = r#"
{
   "serial_number": "SK-00008453",
   "type":"evt_precip",
   "hub_sn": "HB-00000001",
   "evt":[1493322445]
}"#;
let rec: Tempest = serde_json::from_str(&buf).unwrap();
if let Tempest::EvtPrecip(x) = &rec {
    println!("{:?}", x);
    println!("{:?}", x.serial_number);
    println!("{:?}", x.evt);
    println!("{:?}", x.evt.epoch);
}
```
### References
- [`WeatherFlow UDP`](https://weatherflow.github.io/Tempest/api/udp/v171/)

License: Apache-2.0
