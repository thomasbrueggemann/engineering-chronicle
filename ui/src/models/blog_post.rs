use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogPost {
    pub url: String,
    pub title: String,
    pub content: String,
    pub blog: Blog,
    pub categories: Vec<String>,
    pub published: Published,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    pub title: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Published {
    #[serde(rename = "$date")]
    pub date: Date,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    #[serde(rename = "$numberLong")]
    pub number_long: String,
}
