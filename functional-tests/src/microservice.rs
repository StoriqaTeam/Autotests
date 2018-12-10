use diesel::connection::Connection;
use diesel::pg::PgConnection;
use diesel::query_dsl::RunQueryDsl;
use failure::Error as FailureError;
use reqwest::Client;

pub struct UsersMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

pub struct StoresMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

impl StoresMicroservice {
    pub fn clear_all_data(&self) -> Result<(), FailureError> {
        let conn = PgConnection::establish(self.database_url.as_ref())?;
        let _ = diesel::sql_query("TRUNCATE TABLE attribute_values, attributes, base_products, cat_attr_values, categories, coupon_scope_base_products, coupon_scope_categories, coupons, currency_exchange, custom_attributes, moderator_product_comments, moderator_store_comments, prod_attr_values, products, stores, used_coupons, user_roles, wizard_stores;")
            .execute(&conn)?;
        let _ = diesel::sql_query("INSERT INTO user_roles (user_id, name) VALUES (1, 'superuser')")
            .execute(&conn)?;
        Ok(())
    }
}

impl UsersMicroservice {
    pub fn clear_all_data(&self) -> Result<(), FailureError> {
        let conn = PgConnection::establish(self.database_url.as_ref())?;
        let _ = diesel::sql_query("TRUNCATE TABLE identities, reset_tokens, user_roles, users;")
            .execute(&conn)?;
        let _ = diesel::sql_query("INSERT INTO users (id, email, last_login_at, email_verified, saga_id) VALUES (1, 'admin@storiqa.com', now(), true, 'a4cb84cb-62a7-45c6-939e-7c57cc399d5a') ON CONFLICT (id) DO NOTHING;")
            .execute(&conn)?;
        let _ = diesel::sql_query("INSERT INTO identities (user_id, email, provider, password, saga_id) SELECT id, email, 'email', 'ivcHmQPHBx9EUGql4Zv8EaXCkQcswPuL905JCp5ss5k=.js5QVSk6FG', 'a4cb84cb-62a7-45c6-939e-7c57cc399d5a' FROM users WHERE email = 'admin@storiqa.com' LIMIT 1;")
            .execute(&conn)?;
        let _ = diesel::sql_query("INSERT INTO user_roles (user_id, name) SELECT id, 'superuser' FROM users WHERE email = 'admin@storiqa.com' LIMIT 1;")
            .execute(&conn)?;

        Ok(())
    }

    pub fn verify_email(&self, email: &str) -> Result<(), FailureError> {
        let conn = PgConnection::establish(self.database_url.as_ref())?;
        let _ = diesel::sql_query(format!(
            "UPDATE users SET email_verified=true WHERE email='{}';",
            email
        ))
        .execute(&conn)?;

        Ok(())
    }
}
