use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewTutor {
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTutor {
    pub tutor_name: Option<String>,
    pub tutor_pic_url: Option<String>,
    pub tutor_profile: Option<String>,
}

impl From<web::Json<NewTutor>> for NewTutor {
    fn from(new_tutor: web::Json<NewTutor>) -> Self {
        NewTutor {
            tutor_name: new_tutor.tutor_name.clone(),
            tutor_pic_url: new_tutor.tutor_pic_url.clone(),
            tutor_profile: new_tutor.tutor_profile.clone(),
        }
    }
}

impl From<web::Json<UpdateTutor>> for UpdateTutor {
    fn from(new_tutor: web::Json<UpdateTutor>) -> Self {
        UpdateTutor {
            tutor_name: new_tutor.tutor_name.clone(),
            tutor_pic_url: new_tutor.tutor_pic_url.clone(),
            tutor_profile: new_tutor.tutor_profile.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorRegistrationForm {
    pub username: String,
    pub password: String,
    pub confirmation: String,
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorResponse {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub tutor_id: i32,
    pub user_password: String,
}
