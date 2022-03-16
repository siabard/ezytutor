use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, Tutor, UpdateTutor};
use sqlx::postgres::PgPool;

pub async fn get_all_tutors_db(pool: &PgPool) -> Result<Vec<Tutor>, EzyTutorError> {
    let tutor_rows =
        sqlx::query!("SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile FROM ezy_tutor")
            .fetch_all(pool)
            .await?;

    let tutors: Vec<Tutor> = tutor_rows
        .iter()
        .map(|tutor_row| Tutor {
            tutor_id: tutor_row.tutor_id,
            tutor_name: tutor_row.tutor_name.clone(),
            tutor_pic_url: tutor_row.tutor_pic_url.clone(),
            tutor_profile: tutor_row.tutor_profile.clone(),
        })
        .collect();

    match tutors.len() {
        0 => Err(EzyTutorError::NotFound("No Tutors found".into())),
        _ => Ok(tutors),
    }
}

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!("SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile FROM ezy_tutor WHERE tutor_id = $1", tutor_id).fetch_one(pool).await.map(|tutor_row| Tutor {
	tutor_id: tutor_row.tutor_id,
	tutor_name: tutor_row.tutor_name.clone(),
	tutor_pic_url: tutor_row.tutor_pic_url.clone(),
	tutor_profile: tutor_row.tutor_profile.clone(),
    }).map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    Ok(tutor_row)
}

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: NewTutor) -> Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!("insert into ezy_tutor (tutor_name, tutor_pic_url, tutor_profile ) values ($1, $2, $3) returning tutor_id, tutor_name, tutor_pic_url, tutor_profile ", new_tutor.tutor_name, new_tutor.tutor_pic_url, new_tutor.tutor_profile).fetch_one(pool).await?;

    Ok(Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name.clone(),
        tutor_pic_url: tutor_row.tutor_pic_url.clone(),
        tutor_profile: tutor_row.tutor_profile.clone(),
    })
}

pub async fn update_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
    update_tutor: UpdateTutor,
) -> Result<Tutor, EzyTutorError> {
    let current_tutor = sqlx::query_as!(
        Tutor,
        "SELECT * FROM ezy_tutor WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    let name = if let Some(name) = update_tutor.tutor_name {
        name
    } else {
        current_tutor.tutor_name
    };

    let pic_url = if let Some(pic_url) = update_tutor.tutor_pic_url {
        pic_url
    } else {
        current_tutor.tutor_pic_url
    };

    let profile = if let Some(profile) = update_tutor.tutor_profile {
        profile
    } else {
        current_tutor.tutor_profile
    };

    let tutor_row = sqlx::query_as!(Tutor, "update ezy_tutor SET tutor_name = $1, tutor_pic_url = $2, tutor_profile = $3 WHERE tutor_id = $4 returning tutor_id, tutor_name, tutor_pic_url, tutor_profile ", name, pic_url, profile, tutor_id).fetch_one(pool).await;

    match tutor_row {
        Ok(tutor) => Ok(tutor),
        _ => Err(EzyTutorError::NotFound("Tutor id not found".into())),
    }
}

pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<String, EzyTutorError> {
    let result = sqlx::query!("DELETE FROM ezy_tutor WHERE tutor_id = $1", tutor_id)
        .execute(pool)
        .await?;

    Ok(format!("Tutor {:?} is successfully deleted", result))
}
