use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Message {
    SendAuthCode { discord_id : u64, code : String }, 
}
