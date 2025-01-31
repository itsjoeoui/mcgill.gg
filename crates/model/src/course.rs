use super::*;

#[derive(
  Clone,
  Debug,
  Default,
  Deserialize,
  Eq,
  Hash,
  Ord,
  PartialEq,
  PartialOrd,
  Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Course {
  #[serde(rename = "_id")]
  pub id: String,
  pub title: String,
  pub credits: String,
  pub subject: String,
  pub code: String,
  pub level: String,
  pub url: String,
  pub department: String,
  pub faculty: String,
  pub faculty_url: String,
  pub terms: Vec<String>,
  pub description: String,
  pub instructors: Vec<Instructor>,
  pub prerequisites: Vec<String>,
  pub corequisites: Vec<String>,
  pub restrictions: Option<String>,
  pub schedule: Vec<Schedule>,
}
