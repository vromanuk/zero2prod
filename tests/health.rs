#[cfg(test)]
mod tests {
    // use actix_web::web::Data;
    use actix_web::{test, App};
    // use sqlx::postgres::PgPoolOptions;
    // use sqlx::PgPool;
    use zero2prod::api::health;
    // use zero2prod::configuration::get_configuration;

    // async fn cleanup(pool: &PgPool, email: &str) -> Result<(), sqlx::Error> {
    //     sqlx::query!("DELETE FROM subscriptions WHERE email = $1", email)
    //         .execute(pool)
    //         .await?;
    //     Ok(())
    // }

    #[actix_web::test]
    async fn test_health_check_works() {
        let app = test::init_service(App::new().service(health)).await;
        let req = actix_web::test::TestRequest::get()
            .uri("/health")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
    //
    // #[actix_web::test]
    // async fn subscribe_returns_a_200_for_valid_form_data() {
    //     // Given
    //     let configuration = get_configuration().expect("Failed to read configuration.");
    //     let connection_pool =
    //         PgPoolOptions::new().connect_lazy_with(configuration.database.connect_options());
    //     let db_pool = Data::new(connection_pool.clone());
    //     let email_server = MockServer::start().await;
    //     let app = test::init_service(App::new().app_data(db_pool.clone()).service(subscribe)).await;
    //
    //     Mock::given(path("/email"))
    //         .and(method("POST"))
    //         .respond_with(ResponseTemplate::new(200))
    //         .mount(&email_server)
    //         .await;
    //
    //     let subscriber = SubscribeFormData {
    //         name: "hello".to_string(),
    //         email: "hello-world@example.com".to_string(),
    //     };
    //
    //     let req = actix_web::test::TestRequest::post()
    //         .uri("/subscriptions")
    //         .set_form(subscriber.clone())
    //         .to_request();
    //
    //     // When
    //     let resp = test::call_service(&app, req).await;
    //
    //     // Then
    //     assert!(resp.status().is_success());
    //
    //     let saved = sqlx::query!("SELECT email, name FROM subscriptions")
    //         .fetch_one(&connection_pool)
    //         .await
    //         .expect("Failed to fetch saved subscription.");
    //
    //     assert_eq!(saved.email, "hello-world@example.com");
    //     assert_eq!(saved.name, "hello");
    //
    //     cleanup(&connection_pool, &subscriber.email)
    //         .await
    //         .expect("Failed to cleanup saved subscription.");
    // }
    //
    // #[actix_web::test]
    // async fn subscribe_returns_a_400_when_data_is_missing() {
    //     // Given
    //     let configuration = get_configuration().expect("Failed to read configuration.");
    //     let connection_pool =
    //         PgPoolOptions::new().connect_lazy_with(configuration.database.connect_options());
    //     let db_pool = Data::new(connection_pool.clone());
    //     let app = test::init_service(App::new().app_data(db_pool.clone()).service(subscribe)).await;
    //
    //     let test_cases = vec![
    //         SubscribeFormData {
    //             name: "hello".to_string(),
    //             email: "".to_string(),
    //         },
    //         SubscribeFormData {
    //             name: "".to_string(),
    //             email: "test@example.com".to_string(),
    //         },
    //         SubscribeFormData {
    //             name: "".to_string(),
    //             email: "".to_string(),
    //         },
    //         SubscribeFormData {
    //             name: "hello".to_string(),
    //             email: "hello-world".to_string(),
    //         },
    //     ];
    //
    //     // When
    //     for invalid_form_data in test_cases {
    //         let req = actix_web::test::TestRequest::post()
    //             .uri("/subscriptions")
    //             .set_form(invalid_form_data)
    //             .to_request();
    //
    //         let resp = test::call_service(&app, req).await;
    //
    //         // Then
    //         assert!(resp.status().is_client_error());
    //     }
    // }
}
