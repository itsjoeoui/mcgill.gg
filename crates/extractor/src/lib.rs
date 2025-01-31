use {
  anyhow::anyhow,
  course_listing_ext::CourseListingExt,
  course_page_ext::CoursePageExt,
  model::{
    CourseListing, CoursePage, Instructor, Requirement, Requirements, Schedule,
  },
  requirements_ext::RequirementsExt,
  schedule_ext::ScheduleExt,
  scraper::{ElementRef, Html, Selector},
  select::Select,
};

mod course_listing_ext;
mod course_page_ext;
mod requirements_ext;
mod schedule_ext;
mod select;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

pub fn extract_course_listings(
  text: &str,
) -> Result<Option<Vec<CourseListing>>> {
  match Html::parse_fragment(text)
    .root_element()
    .select_optional("div[class='view-content']")?
  {
    Some(content) => Ok(Some(
      content
        .select_many("div[class~='views-row']")?
        .iter()
        .map(CourseListing::from_listing)
        .collect::<Result<Vec<CourseListing>, _>>()?
        .into_iter()
        .map(|listing| listing.filter_terms())
        .collect(),
    )),
    None => Ok(None),
  }
}

pub fn extract_course_page(text: &str) -> Result<CoursePage> {
  CoursePage::from_html(Html::parse_fragment(text))
}

pub fn extract_course_schedules(text: &str) -> Result<Vec<Schedule>> {
  let html = Html::parse_fragment(text);

  Ok(
    match html
      .root_element()
      .select_single("errors")?
      .select_many("error")?
      .is_empty()
    {
      false => Vec::new(),
      _ => html
        .root_element()
        .select_many("block")?
        .iter()
        .map(Schedule::from_block)
        .collect(),
    },
  )
}

fn extract_course_instructors(html: &Html) -> Result<Vec<Instructor>> {
  let mut instructors = Vec::new();

  let catalog = html
    .root_element()
    .select_single("div[class='node node-catalog clearfix']")?;

  let raw = catalog
    .select_single("p[class='catalog-terms']")?
    .inner_html();

  let terms = raw
    .trim()
    .split(' ')
    .skip(1)
    .filter(|entry| !entry.is_empty())
    .collect::<Vec<&str>>();

  let mut tokens = catalog
    .select_single("p[class='catalog-instructors']")?
    .inner_html()
    .trim()
    .split(' ')
    .skip(1)
    .collect::<Vec<&str>>()
    .join(" ");

  terms
    .join(" ")
    .split(", ")
    .map(|term| {
      (
        term.split(' ').take(1).collect::<String>(),
        term.to_string(),
      )
    })
    .for_each(|(term, full_term)| {
      if tokens.contains(&format!("({term})")) {
        let split = tokens.split(&format!("({term})")).collect::<Vec<&str>>();

        let inner = split[0]
          .split(';')
          .map(|s| {
            Instructor::default()
              .set_name(&s.trim().split(", ").collect::<Vec<&str>>())
              .set_term(&full_term)
          })
          .collect::<Vec<Instructor>>();

        if split.len() > 1 {
          tokens = split[1].trim().to_string();
        }

        instructors.extend(inner);
      }
    });

  Ok(instructors)
}

fn extract_course_requirements(html: &Html) -> Result<Requirements> {
  match html
    .root_element()
    .select_optional("ul[class='catalog-notes']")?
  {
    Some(notes) => Requirements::from_notes(notes),
    None => Ok(Requirements::default()),
  }
}

#[cfg(test)]
mod tests {
  use {
    super::{
      CourseListing, CoursePage, Html, Instructor, Requirements, Schedule,
    },
    include_dir::{include_dir, Dir},
    pretty_assertions::assert_eq,
  };

  static MOCK_DIR: Dir<'_> = include_dir!("crates/extractor/mocks");

  fn get_content(name: &str) -> String {
    MOCK_DIR
      .get_file(name)
      .unwrap()
      .contents_utf8()
      .unwrap()
      .to_string()
  }

  #[test]
  fn extract_course_listings() {
    assert_eq!(
      super::extract_course_listings(&get_content("course_listings.html"))
        .unwrap()
        .unwrap(),
      vec![
        CourseListing {
          department: "Bioresource Engineering".into(),
          faculty: "Agricultural &amp; Environmental Sciences".into(),
          level: "Undergraduate".into(),
          terms: ["Fall 2022".into()].to_vec(),
          url: "/study/2022-2023/courses/aeph-120".into(),
        },
        CourseListing {
          department: "Bioresource Engineering".into(),
          faculty: "Agricultural &amp; Environmental Sciences".into(),
          level: "Undergraduate".into(),
          terms: ["Winter 2023".into()].to_vec(),
          url: "/study/2022-2023/courses/aeph-122".into(),
        },
        CourseListing {
          department: "Institute for Aerospace Eng.".into(),
          faculty: "Faculty of Engineering".into(),
          level: "Undergraduate".into(),
          terms: ["Fall 2022".into()].to_vec(),
          url: "/study/2022-2023/courses/aero-401".into(),
        },
        CourseListing {
          department: "Institute for Aerospace Eng.".into(),
          faculty: "Faculty of Engineering".into(),
          level: "Undergraduate".into(),
          terms: ["Winter 2023".into()].to_vec(),
          url: "/study/2022-2023/courses/aero-410".into(),
        },
        CourseListing {
          department: "Institute for Aerospace Eng.".into(),
          faculty: "Faculty of Engineering".into(),
          level: "Undergraduate".into(),
          terms: ["Fall 2022".into()].to_vec(),
          url: "/study/2022-2023/courses/aero-460d1".into(),
        },
        CourseListing {
          department: "Institute for Aerospace Eng.".into(),
          faculty: "Faculty of Engineering".into(),
          level: "Undergraduate".into(),
          terms: ["Winter 2023".into()].to_vec(),
          url: "/study/2022-2023/courses/aero-460d2".into(),
        },
        CourseListing {
          department: "Islamic Studies".into(),
          faculty: "Faculty of Arts".into(),
          level: "Undergraduate".into(),
          terms: ["Fall 2022".into()].to_vec(),
          url: "/study/2022-2023/courses/afri-200".into(),
        },
        CourseListing {
          department: "Islamic Studies".into(),
          faculty: "Faculty of Arts".into(),
          level: "Undergraduate".into(),
          terms: ["Fall 2022".into()].to_vec(),
          url: "/study/2022-2023/courses/afri-401".into(),
        },
        CourseListing {
          department: "Islamic Studies".into(),
          faculty: "Faculty of Arts".into(),
          level: "Undergraduate".into(),
          terms: [].to_vec(),
          url: "/study/2022-2023/courses/afri-480".into(),
        },
        CourseListing {
          department: "Islamic Studies".into(),
          faculty: "Faculty of Arts".into(),
          level: "Undergraduate".into(),
          terms: ["Fall 2022".into()].to_vec(),
          url: "/study/2022-2023/courses/afri-481".into(),
        },
        CourseListing {
          department: "Islamic Studies".into(),
          faculty: "Faculty of Arts".into(),
          level: "Undergraduate".into(),
          terms: [].to_vec(),
          url: "/study/2022-2023/courses/afri-499".into(),
        },
        CourseListing {
          department: "Islamic Studies".into(),
          faculty: "Faculty of Arts".into(),
          level: "Graduate, Undergraduate".into(),
          terms: ["Winter 2023".into()].to_vec(),
          url: "/study/2022-2023/courses/afri-598".into(),
        },
        CourseListing {
          department: "Agricultural Economics".into(),
          faculty: "Agricultural &amp; Environmental Sciences".into(),
          level: "Undergraduate".into(),
          terms: ["Fall 2022".into()].to_vec(),
          url: "/study/2022-2023/courses/agec-200".into(),
        },
        CourseListing {
          department: "Agricultural Economics".into(),
          faculty: "Agricultural &amp; Environmental Sciences".into(),
          level: "Undergraduate".into(),
          terms: ["Winter 2023".into()].to_vec(),
          url: "/study/2022-2023/courses/agec-201".into(),
        },
        CourseListing {
          department: "Agricultural Economics".into(),
          faculty: "Agricultural &amp; Environmental Sciences".into(),
          level: "Undergraduate".into(),
          terms: ["Winter 2023".into()].to_vec(),
          url: "/study/2022-2023/courses/agec-231".into(),
        },
        CourseListing {
          department: "Agricultural Economics".into(),
          faculty: "Agricultural &amp; Environmental Sciences".into(),
          level: "Undergraduate".into(),
          terms: [].to_vec(),
          url: "/study/2022-2023/courses/agec-242".into(),
        },
        CourseListing {
          department: "Agricultural Economics".into(),
          faculty: "Agricultural &amp; Environmental Sciences".into(),
          level: "Undergraduate".into(),
          terms: ["Winter 2023".into()].to_vec(),
          url: "/study/2022-2023/courses/agec-320".into(),
        },
        CourseListing {
          department: "Natural Resource Sciences".into(),
          faculty: "Agricultural &amp; Environmental Sciences".into(),
          level: "Undergraduate".into(),
          terms: [].to_vec(),
          url: "/study/2022-2023/courses/agec-330".into(),
        },
        CourseListing {
          department: "Natural Resource Sciences".into(),
          faculty: "Agricultural &amp; Environmental Sciences".into(),
          level: "Undergraduate".into(),
          terms: ["Fall 2022".into()].to_vec(),
          url: "/study/2022-2023/courses/agec-332".into(),
        },
        CourseListing {
          department: "Agricultural Economics".into(),
          faculty: "Agricultural &amp; Environmental Sciences".into(),
          level: "Undergraduate".into(),
          terms: ["Fall 2022".into()].to_vec(),
          url: "/study/2022-2023/courses/agec-333".into(),
        },
      ]
      .to_vec(),
    );
  }

  #[test]
  fn extract_course_instructors() {
    assert_eq!(
      super::extract_course_instructors(&Html::parse_fragment(&get_content(
        "course_page.html"
      )))
      .unwrap(),
      vec![
        Instructor {
          name: "Adrian Roshan Vetta".into(),
          term: "Fall 2022".into()
        },
        Instructor {
          name: "Jérôme Fortier".into(),
          term: "Fall 2022".into()
        },
        Instructor {
          name: "Jérôme Fortier".into(),
          term: "Winter 2023".into()
        },
        Instructor {
          name: "Jeremy Macdonald".into(),
          term: "Winter 2023".into()
        }
      ]
    );
  }

  #[test]
  fn extract_course_requirements() {
    assert_eq!(
      super::extract_course_requirements(&Html::parse_fragment(
        &get_content("course_page.html")
      ))
      .unwrap(),
      Requirements {
        corequisites: vec!["MATH 133".into()],
        prerequisites: Vec::new(),
        restrictions: Some("For students in any Computer Science, Computer Engineering, or Software Engineering programs. Others only with the instructor's permission. Not open to students who have taken or are taking MATH 235.".into())
      }
    );
  }

  #[test]
  fn extract_course_page() {
    assert_eq!(
      super::extract_course_page(
        &get_content("course_page.html")
      )
      .unwrap(),
      CoursePage {
        title: "Discrete Structures".into(),
        credits: "3".into(),
        subject: "MATH".into(),
        code: "240".into(),
        faculty_url: "/study/2022-2023/faculties/science".into(),
        description: "Introduction to discrete mathematics and applications. Logical reasoning and methods of proof. Elementary number theory and cryptography  prime numbers, modular equations, RSA encryption. Combinatorics  basic enumeration, combinatorial methods, recurrence equations. Graph theory  trees, cycles, planar\ngraphs.".into(),
        instructors: vec![Instructor { name: "Adrian Roshan Vetta".into(), term: "Fall 2022".into() }, Instructor { name: "Jérôme Fortier".into(), term: "Fall 2022".into() }, Instructor { name: "Jérôme Fortier".into(), term: "Winter 2023".into() }, Instructor { name: "Jeremy Macdonald".into(), term: "Winter 2023".into() }],
        requirements: Requirements { corequisites: vec!["MATH 133".into()], prerequisites: vec![], restrictions: Some("For students in any Computer Science, Computer Engineering, or Software Engineering programs. Others only with the instructor's permission. Not open to students who have taken or are taking MATH 235.".into()) } }
    );
  }

  #[test]
  fn extract_course_schedules() {
    assert_eq!(
      super::extract_course_schedules(&get_content("course_schedules.xml"))
        .unwrap(),
      vec![Schedule {
        campus: Some("Downtown".into()),
        display: Some("Lec 045".into()),
        location: Some("BRONF 422".into()),
      }]
    );
  }
}
