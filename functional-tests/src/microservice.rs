use diesel::connection::Connection;
use diesel::pg::PgConnection;
use diesel::query_dsl::RunQueryDsl;
use failure::Error as FailureError;
use reqwest::Client;

pub const CURRENCY_EXCHANGE_DATA: &'static str = r#"{"BTC":{"BTC":1,"ETH":0.03125308,"EUR":0.00032189359069340675,"RUB":0.0000043859649122807014,"STQ":1e-7,"USD":0.0002915451895043732},"ETH":{"BTC":31.86540054808489,"ETH":1,"EUR":0.010283658265572796,"RUB":0.00014027212792818068,"STQ":0.0000032772323515661525,"USD":0.009324483793737874},"EUR":{"BTC":3091.01545511,"ETH":96.80517305,"EUR":1,"STQ":0.000316,"USD":0.90131254},"RUB":{"BTC":227401.13700264,"ETH":7100.28408493,"RUB":1,"STQ":0.022813,"USD":64.75},"STQ":{"BTC":9090909.090909092,"ETH":305135.5206847361,"EUR":3021.1480362537764,"RUB":42.58037044922291,"STQ":1,"USD":2725.166916473634},"USD":{"BTC":3419.98277928,"ETH":106.95,"EUR":1.1043704998696733,"RUB":0.015384615386982249,"STQ":0.0003505,"USD":1}}"#;

#[derive(Clone)]
pub struct UsersMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

#[derive(Clone)]
pub struct StoresMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

#[derive(Clone)]
pub struct OrdersMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

#[derive(Clone)]
pub struct BillingMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

#[derive(Clone)]
pub struct DeliveryMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

#[derive(Clone)]
pub struct WarehousesMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

#[derive(Clone)]
pub struct SagaMicroservice {
    pub url: String,
    pub client: Client,
}

#[derive(Clone)]
pub struct NotificationsMicroservice {
    pub database_url: String,
    pub url: String,
    pub client: Client,
}

#[derive(Clone)]
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
        let ref json: serde_json::Value = serde_json::from_str(
            r#"
            {
                "user_id": 1,
                "email": "user@mail.com"
            }"#,
        )
        .unwrap();

        self.client
            .delete(&format!("{}/emarsys/contact", self.url))
            .json(json)
            .send()?;

        let conn = PgConnection::establish(self.database_url.as_ref())?;
        let _ = diesel::sql_query("TRUNCATE TABLE user_roles;").execute(&conn)?;
        let _ = diesel::sql_query("INSERT INTO user_roles (user_id, name) VALUES (1, 'superuser')")
            .execute(&conn)?;

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
    pub fn clear_all_data(&self) -> Result<(), FailureError> {
        let conn = PgConnection::establish(self.database_url.as_ref())?;
        let _ = diesel::sql_query(
            "TRUNCATE TABLE
        payment_intent,
        accounts,
        customers,
        event_store,
        fees,
        international_billing_info,
        payment_intents_fees,
        payment_intents_invoices,
        proxy_companies_billing_info,
        russia_billing_info,
        store_billing_type,
        amounts_received,
        event_store,
        invoices,
        invoices_v2,
        merchants,
        order_exchange_rates,
        orders,
        orders_info,
        roles;",
        )
        .execute(&conn)?;
        let _ = diesel::sql_query("INSERT INTO roles (user_id, name) VALUES (1, 'superuser')")
            .execute(&conn)?;
        Ok(())
    }

    pub fn healthcheck(&self) -> Result<(), FailureError> {
        healthcheck(&self.client, &self.url).map_err(|e| {
            e.context("Healthcheck in billing microservice failed")
                .into()
        })
    }
}

impl DeliveryMicroservice {
    pub fn clear_all_data(&self) -> Result<(), FailureError> {
        let conn = PgConnection::establish(self.database_url.as_ref())?;
        let _ = diesel::sql_query("TRUNCATE TABLE shipping_rates, companies, companies_packages, packages, pickups, products, roles, user_addresses;")
            .execute(&conn)?;
        let _ = diesel::sql_query("INSERT INTO roles (user_id, name) VALUES (1, 'superuser')")
            .execute(&conn)?;
        Ok(())
    }

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

    pub fn clear_all_data(&self) -> Result<(), FailureError> {
        let conn = PgConnection::establish(self.database_url.as_ref())?;
        let _ = diesel::sql_query("TRUNCATE TABLE roles, stocks, warehouses;").execute(&conn)?;

        let _ = diesel::sql_query(
            "INSERT INTO roles (user_id, name, data) VALUES (1, 'superadmin', 'null')",
        )
        .execute(&conn)?;
        Ok(())
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
        // TODO: This is unsafe, rewrite using proper special characters escaping!
        let _ = diesel::sql_query(format!(
            "INSERT INTO currency_exchange (data) values ('{}')",
            CURRENCY_EXCHANGE_DATA
        ))
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
