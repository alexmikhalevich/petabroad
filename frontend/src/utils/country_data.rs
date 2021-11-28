use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaccinesData {
    pub name: String,
    pub desc: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CountryData {
    pub id: String,
    pub vaccines: Option<Vec<VaccinesData>>,
}
