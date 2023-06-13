use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize,Debug)]
pub struct Location(pub String);
// pub struct LocationResult{
//     #[serde(rename = "Location")]
//     location: String,
// }