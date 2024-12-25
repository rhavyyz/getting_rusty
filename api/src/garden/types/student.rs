use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Student {
    pub age: i64,
    pub id: String,
    pub zap: String,
}
