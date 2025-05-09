use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, Tutor, UpdateTutor};
use sqlx::postgres::PgPool;

pub async fn get_all_tutors_db(pool: &PgPool) -> Result<Vec<Tutor>, EzyTutorError> {
    //Prepare SQL Statement -> manually aproach with query! instead of query_as!
    //just for learning reasons, if a struct has e.g. more fields than the database table
    //which is sometimes true in real world apps
    let tutor_rows =
        sqlx::query!("SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile FROM ezy_tutor_c6")
            .fetch_all(pool)
            .await?;

    //Extract result manually without sqlx::FromRow
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
        0 => Err(EzyTutorError::NotFound("No tutors found".into())),
        _ => Ok(tutors),
    }
}

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, EzyTutorError> {
    //Prepare SQL statement
    let tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile
        FROM ezy_tutor_c6 WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map(|tutor_row| Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    Ok(tutor_row)
}

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: NewTutor) -> Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!(
        "INSERT INTO ezy_tutor_c6 (tutor_name, tutor_pic_url, tutor_profile) 
        VALUES ($1,$2,$3) RETURNING tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        new_tutor.tutor_name,
        new_tutor.tutor_pic_url,
        new_tutor.tutor_profile
    )
    .fetch_one(pool)
    .await?;
    //Retrieve result
    Ok(Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
}

pub async fn update_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
    update_tutor: UpdateTutor,
) -> Result<Tutor, EzyTutorError> {
    //Retrieve current tutor_row
    let current_tutor_row = sqlx::query_as!(
        Tutor,
        "SELECT * FROM ezy_tutor_c6 WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound("tutor id not found".into()))?;

    //Construct the parameters for update
    let name = if let Some(updatename) = update_tutor.tutor_name {
        updatename
    } else {
        current_tutor_row.tutor_name
    };
    let picurl = if let Some(update_picurl) = update_tutor.tutor_pic_url {
        update_picurl
    } else {
        current_tutor_row.tutor_pic_url
    };
    let profile = if let Some(update_profile) = update_tutor.tutor_profile {
        update_profile
    } else {
        current_tutor_row.tutor_profile
    };
    //Prepare SQL Statement
    let tutor_row = sqlx::query_as!(Tutor,"UPDATE ezy_tutor_c6 SET tutor_name = $1,
                            tutor_pic_url = $2, tutor_profile = $3 WHERE tutor_id = $4 RETURNING tutor_id, tutor_name,
                            tutor_pic_url, tutor_profile", name, picurl, profile, tutor_id)
                            .fetch_one(pool)
                            .await;

    if let Ok(tutor) = tutor_row {
        Ok(tutor)
    } else {
        Err(EzyTutorError::NotFound("Tutor not found".into()))
    }
}

pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<String, EzyTutorError> {
    //Prepare SQL Statement
    let tutor_row = sqlx::query!("DELETE FROM ezy_tutor_c6 WHERE tutor_id = $1", tutor_id)
        .execute(pool)
        .await?;
    Ok(format!("Deleted {:?} record", tutor_row))
}
