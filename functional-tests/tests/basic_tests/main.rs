extern crate failure;
extern crate functional_tests;

mod basic_tests;

use functional_tests::context::TestContext;

#[test]
pub fn a_users_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.users_microservice_healthcheck().unwrap();
}

#[test]
pub fn a_stores_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.stores_microservice_healthcheck().unwrap();
}

#[test]
pub fn a_orders_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.orders_microservice_healthcheck().unwrap();
}

#[test]
pub fn a_warehouses_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.warehouses_microservice_healthcheck().unwrap();
}

#[test]
pub fn a_billing_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.billing_microservice_healthcheck().unwrap();
}

#[test]
pub fn a_notifications_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.notifications_microservice_healthcheck().unwrap();
}

#[test]
pub fn a_delivery_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.delivery_microservice_healthcheck().unwrap();
}

#[test]
pub fn a_saga_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.saga_microservice_healthcheck().unwrap();
}

#[test]
pub fn a_gateway_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.gateway_microservice_healthcheck().unwrap();
}
