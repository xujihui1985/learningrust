use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        email = %form.email,
        name = %form.name,
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>,
) -> impl Responder {
    // let query_span = tracing::info_span!("saving new subscriber details in the database");

    // tracing::info!(
    //     "save '{}' '{}' as new subscriber in database",
    //     form.email,
    //     form.name
    // );
    // let res = sqlx::query(
    //     r#"
    //     INSERT INTO subscriptions(id, email, name, subscribed_at)
    //     VALUES ($1, $2, $3, $4)
    //     "#,
    // )
    // .bind(Uuid::new_v4())
    // .bind(&form.email)
    // .bind(&form.name)
    // .bind(Utc::now())
    // .execute(pool.get_ref())
    // .instrument(query_span)
    // .await;
    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name: form.0.name.parse().unwrap()
    };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => {
            tracing::info!("new subscriber details has been saved");
            HttpResponse::Ok()
        }
        Err(e) => {
            tracing::error!("failed to execute query: {:?}", e);
            HttpResponse::InternalServerError()
        }
    }
}

#[tracing::instrument(
    name="saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&new_subscriber.email)
    .bind(new_subscriber.name.inner_ref())
    .bind(Utc::now())
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to execute query {:?}", e);
        e
    })?;
    Ok(())
}
