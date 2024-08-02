use  serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String, 
    pub content: String
}