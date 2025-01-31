use {
  bson::{doc, Bson},
  chrono::prelude::*,
  serde::{Deserialize, Serialize},
};

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

mod course;
mod course_listing;
mod course_page;
mod instructor;
mod requirements;
mod review;
mod schedule;

pub use crate::{
  course::Course,
  course_listing::CourseListing,
  course_page::CoursePage,
  instructor::Instructor,
  requirements::{Requirement, Requirements},
  review::Review,
  schedule::Schedule,
};
