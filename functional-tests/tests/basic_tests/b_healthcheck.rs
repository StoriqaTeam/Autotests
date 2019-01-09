use functional_tests::context::TestContext;

#[test]
pub fn users_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.users_microservice_healthcheck().unwrap();
}

#[test]
pub fn stores_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.stores_microservice_healthcheck().unwrap();
}

#[test]
pub fn orders_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.orders_microservice_healthcheck().unwrap();
}

#[test]
pub fn warehouses_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.warehouses_microservice_healthcheck().unwrap();
}

#[test]
pub fn billing_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.billing_microservice_healthcheck().unwrap();
}

#[test]
pub fn notifications_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.notifications_microservice_healthcheck().unwrap();
}

#[test]
pub fn delivery_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.delivery_microservice_healthcheck().unwrap();
}

#[test]
pub fn saga_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.saga_microservice_healthcheck().unwrap();
}

#[test]
pub fn gateway_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.gateway_microservice_healthcheck().unwrap();
}
