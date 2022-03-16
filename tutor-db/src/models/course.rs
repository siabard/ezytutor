use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            course_id: course.course_id,
            tutor_id: course.tutor_id,
            course_name: course.course_name.clone(),
            course_description: course.course_description.clone(),
            course_format: course.course_format.clone(),
            course_structure: course.course_structure.clone(),
            course_duration: course.course_duration.clone(),
            course_language: course.course_language.clone(),
            course_price: course.course_price,
            course_level: course.course_level.clone(),
            posted_time: course.posted_time,
        }
    }
}

// Web Service 에서 입력되는 항목으로
// 우리는 JSON이나 일반 String으로 된 값을
// Rust 객체로 바꿀 것이므로 Deserialize가 필요하다.
// 이 값이 다시 Web Service로 나갈 일은 없으므로 Serialize는
// 필요하지않다.

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(new_course: web::Json<CreateCourse>) -> Self {
        CreateCourse {
            tutor_id: new_course.tutor_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_structure: new_course.course_structure.clone(),
            course_duration: new_course.course_duration.clone(),
            course_language: new_course.course_language.clone(),
            course_price: new_course.course_price,
            course_level: new_course.course_level.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub course_name: Option<String>,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(new_course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_structure: new_course.course_structure.clone(),
            course_duration: new_course.course_duration.clone(),
            course_language: new_course.course_language.clone(),
            course_price: new_course.course_price,
            course_level: new_course.course_level.clone(),
        }
    }
}
