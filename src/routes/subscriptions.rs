use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct SubscribeFormData {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<SubscribeFormData>, pool: web::Data<PgPool>) -> impl Responder {
    if let Err(e) = form.validate() {
        return HttpResponse::BadRequest().body(format!("Invalid input: {:?}", e));
    }

    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (email, name, subscribed_at)
    VALUES ($1, $2, $3)
            "#,
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
