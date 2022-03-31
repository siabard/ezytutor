use crate::dbaccess::tutor::*;
use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, TutorRegistrationForm, TutorResponse, UpdateTutor, User};
use crate::state::AppState;
use actix_web::Error;
use actix_web::{error, web, HttpResponse};
use argon2::Config;
use serde_json::json;

pub async fn get_all_tutors(
    app_state: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutors = get_all_tutors_db(&app_state.db).await?;

    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutors);
    let rendered_html = tmpl
        .render("tutors.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("ERROR"))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered_html))
}

pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    web::Path(tutor_id): web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    get_tutor_details_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn post_new_tutor(
    app_state: web::Data<AppState>,
    new_tutor: web::Json<NewTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    post_new_tutor_db(&app_state.db, new_tutor.into())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    web::Path(tutor_id): web::Path<i32>,
    new_tutor: web::Json<UpdateTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    update_tutor_details_db(&app_state.db, tutor_id, new_tutor.into())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    web::Path(tutor_id): web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    delete_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|msg| HttpResponse::Ok().json(msg))
}

pub async fn show_register_form(
    _app_state: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, EzyTutorError> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    let html = tmpl
        .render("register.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("Template error".into()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegistrationForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    let username = params.username.clone();
    let user = get_user_record(&app_state.db, username.clone()).await;
    let user_not_found: bool = user.is_err();

    if user_not_found {
        if params.password != params.confirmation {
            ctx.insert("error", "Passwords do not match");
            ctx.insert("current_username", &params.username);
            ctx.insert("current_password", "");
            ctx.insert("current_confirmation", "");
            ctx.insert("current_name", &params.name);
            ctx.insert("current_imageurl", &params.imageurl);
            ctx.insert("current_profile", &params.profile);
            s = tmpl
                .render("register.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".into()))
                .unwrap();
        } else {
            let new_tutor = json!({
            "tutor_name": &params.name,
            "tutor_pic_url": &params.imageurl,
            "tutor_profile": &params.profile
            });

            let awc_client = awc::Client::default();
            let res = awc_client
                .post("http://localhost:13000/tutors")
                .send_json(&new_tutor)
                .await?
                .body()
                .await?;

            let tutor_response: TutorResponse =
                serde_json::from_str(&std::str::from_utf8(&res).unwrap()).unwrap();
            s = format!("Congratulations. You have been successfully registered with EzyTutor and your id is {}. To start using EzyTutor, please login with your credentials", tutor_response.tutor_id);

            // hash password
            let salt = b"somerandomsalt";
            let config = Config::default();
            let hash =
                argon2::hash_encoded(params.password.clone().as_bytes(), salt, &config).unwrap();
            let user = User {
                username: username.clone(),
                tutor_id: tutor_response.tutor_id,
                user_password: hash,
            };

            let _tutor_created = post_new_user(&app_state.db, user).await?;
        }
    } else {
        ctx.insert("error", "User ID already exists");
        ctx.insert("current_username", &params.username);
        ctx.insert("current_password", "");
        ctx.insert("current_confirmation", "");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))
            .unwrap();
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
