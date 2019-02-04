use functional_tests::context::{Microservice, TestContext};

#[test]
pub fn users_microservice_healthcheck() {
    //given
    let context = TestContext::new_without_clear_data();
    //then
    let _ = context
        .microservice_healthcheck(Microservice::Users)
        .unwrap();
}

#[test]
pub fn stores_microservice_healthcheck() {
    //given
    let context = TestContext::new_without_clear_data();
    //then
    let _ = context
        .microservice_healthcheck(Microservice::Stores)
        .unwrap();
}

#[test]
pub fn orders_microservice_healthcheck() {
    //given
    let context = TestContext::new_without_clear_data();
    //then
    let _ = context
        .microservice_healthcheck(Microservice::Orders)
        .unwrap();
}

#[test]
pub fn warehouses_microservice_healthcheck() {
    //given
    let context = TestContext::new_without_clear_data();
    //then
    let _ = context
        .microservice_healthcheck(Microservice::Warehouses)
        .unwrap();
}

#[test]
pub fn billing_microservice_healthcheck() {
    //given
    let context = TestContext::new_without_clear_data();
    //then
    let _ = context
        .microservice_healthcheck(Microservice::Billing)
        .unwrap();
}

#[test]
pub fn notifications_microservice_healthcheck() {
    //given
    let context = TestContext::new_without_clear_data();
    //then
    let _ = context
        .microservice_healthcheck(Microservice::Notifications)
        .unwrap();
}

#[test]
pub fn delivery_microservice_healthcheck() {
    //given
    let context = TestContext::new_without_clear_data();
    //then
    let _ = context
        .microservice_healthcheck(Microservice::Delivery)
        .unwrap();
}

#[test]
pub fn saga_microservice_healthcheck() {
    //given
    let context = TestContext::new_without_clear_data();
    //then
    let _ = context
        .microservice_healthcheck(Microservice::Saga)
        .unwrap();
}

#[test]
pub fn gateway_microservice_healthcheck() {
    //given
    let context = TestContext::new_without_clear_data();
    //then
    let _ = context
        .microservice_healthcheck(Microservice::Gateway)
        .unwrap();
}
