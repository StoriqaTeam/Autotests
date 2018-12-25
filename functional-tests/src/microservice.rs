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

pub struct OrdersMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

pub struct BillingMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

pub struct DeliveryMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

pub struct WarehousesMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

pub struct SagaMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

pub struct NotificationsMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

pub struct GatewayMicroservice {
    pub url: String,
    pub client: Client,
}

impl GatewayMicroservice {
    pub fn healthcheck(&self) -> Result<(), FailureError> {
        healthcheck(&self.client, &self.url).map_err(|e| {
            e.context("Healthcheck in gateway microservice failed")
                .into()
        })
    }
}

impl OrdersMicroservice {
    pub fn clear_all_data(&self) -> Result<(), FailureError> {
        let conn = PgConnection::establish(self.database_url.as_ref())?;
        let _ = diesel::sql_query(
            "TRUNCATE TABLE cart_items_session, cart_items_user, order_diffs, orders, roles;",
        )
        .execute(&conn)?;
        let _ = diesel::sql_query(
            "INSERT INTO roles (user_id, name, data) VALUES (1, 'superadmin', 'null')",
        )
        .execute(&conn)?;

        Ok(())
    }

    pub fn healthcheck(&self) -> Result<(), FailureError> {
        healthcheck(&self.client, &self.url).map_err(|e| {
            e.context("Healthcheck in orders microservice failed")
                .into()
        })
    }
}

impl NotificationsMicroservice {
    pub fn clear_all_data(&self) -> Result<(), FailureError> {
        let ref json: serde_json::Value = serde_json::from_str(r#"
            {
                "user_id": 1,
                "email": "user@mail.com"
            }"#).unwrap();

        self.client.delete(&format!("{}/emarsys/contact", self.url))
            .json(json)
            .send()?;

        Ok(())
    }

    pub fn healthcheck(&self) -> Result<(), FailureError> {
        healthcheck(&self.client, &self.url).map_err(|e| {
            e.context("Healthcheck in notifications microservice failed")
                .into()
        })
    }
}

impl BillingMicroservice {
    pub fn healthcheck(&self) -> Result<(), FailureError> {
        healthcheck(&self.client, &self.url).map_err(|e| {
            e.context("Healthcheck in billing microservice failed")
                .into()
        })
    }
}

impl DeliveryMicroservice {
    pub fn healthcheck(&self) -> Result<(), FailureError> {
        healthcheck(&self.client, &self.url).map_err(|e| {
            e.context("Healthcheck in delivery microservice failed")
                .into()
        })
    }
}

impl WarehousesMicroservice {
    pub fn healthcheck(&self) -> Result<(), FailureError> {
        healthcheck(&self.client, &self.url).map_err(|e| {
            e.context("Healthcheck in warehouses microservice failed")
                .into()
        })
    }
}

impl SagaMicroservice {
    pub fn healthcheck(&self) -> Result<(), FailureError> {
        healthcheck(&self.client, &self.url)
            .map_err(|e| e.context("Healthcheck in saga microservice failed").into())
    }
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

    pub fn healthcheck(&self) -> Result<(), FailureError> {
        healthcheck(&self.client, &self.url).map_err(|e| {
            e.context("Healthcheck in stores microservice failed")
                .into()
        })
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

    pub fn healthcheck(&self) -> Result<(), FailureError> {
        healthcheck(&self.client, &self.url)
            .map_err(|e| e.context("Healthcheck in users microservice failed").into())
    }
}

fn healthcheck(client: &Client, url: &str) -> Result<(), FailureError> {
    let response = client.get(&format!("{}/healthcheck", url)).send()?;
    if !response.status().is_success() {
        return Err(failure::format_err!("Healthcheck failed"));
    }
    Ok(())
}
