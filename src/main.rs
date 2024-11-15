use serde::{ser, Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use serde_json;

use std::fs::File;
use std::io::Read;
use std::time::Duration;
use uuid::Uuid;
use url::Url;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Deserialize, Serialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug
}

fn main(){
    let event=  Event{name: "Event 1".to_string(), date: "2024-11-14".to_string()};

    let json = serde_json::to_string(&event).unwrap();
    println!("{:?}", json);

    let des_event: Event = serde_json::from_str(&json).unwrap();
    println!("{:?}", des_event);


    // let mut file = File::open("./request.json").unwrap();
    // let mut json_str = String::new();
    // file.read_to_string(&mut json_str).unwrap();
    // //println!("{}", json_str);

    // let response: Response = serde_json::from_str(&json_str).unwrap();
    // println!("{:?}", response);

    // let yaml_str = serde_yaml::to_string(&response).expect("Failed to convert to YAML");
    // println!("YAML Output:\n{}", yaml_str);

    // let toml_str = toml::to_string(&response).expect("Failed to convert to TOML");
    // println!("TOML Output:\n{}", toml_str);
}

#[derive(Debug, Deserialize, Serialize)]
struct Event{
    name: String,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    date: String,
}

fn serialize_date<S: Serializer>(date: &str, serializer: S) ->
Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("Date: {}", date))
}

fn deserialize_date<'de, D: Deserializer<'de>>(deserializer: D) ->
Result<String, D::Error> {
    let data: &str = Deserialize::deserialize(deserializer)?;
    Ok(data.replace("Date: ", ""))
}


#[cfg(test)]
mod tests{
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_1(){
        let mut file = File::open("./request.json").unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();
        
        let response: Response = serde_json::from_str(&json_str).unwrap();

        assert_eq!(response.stream.public_tariff.price, 100);
        assert_eq!(response.stream.user_id, Uuid::from_str("8d234120-0bda-49b2-b7e0-fbd3912f6cbf").unwrap());

        assert!(response.debug.duration > Duration::from_secs(0));
        assert_eq!(response.request_type, RequestType::Success);
    }
} 

