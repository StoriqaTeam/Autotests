use functional_tests::context::TestContext;
use functional_tests::query::*;
use functional_tests::query::{
    create_store_subscription::{self, *},
    get_store_subscription::{self, *},
    update_store_subscription::*,
};

use common::*;

#[test]
pub fn create_store_subscription() {
    //given
    let mut context = TestContext::new();
    let (_user, token, store, _category) = set_up_store(&mut context).expect("set_up_user failed");
    //when
    context.set_bearer(token);
    let created_store_subsciption = context
        .request(CreateStoreSubscriptionInput {
            store_id: store.raw_id,
            currency: create_store_subscription::Currency::EUR,
            ..default_create_store_subscription_input()
        })
        .expect("Create store subscription failed");
    let retrieved_store_subsciption = context
        .request(GetStoreSubscriptionInput {
            store_id: store.raw_id,
        })
        .expect("Get store subscription failed")
        .expect("Get store subscription returned none");
    //then
    assert_eq!(
        created_store_subsciption.store_id, store.raw_id,
        "store_id in created_store_subsciption"
    );
    assert_eq!(
        created_store_subsciption.currency,
        create_store_subscription::Currency::EUR,
        "currency in created_store_subsciption"
    );
    assert_eq_f64!(created_store_subsciption.value, 0.03, 3);
    assert!(
        created_store_subsciption.wallet_address.is_none(),
        "wallet_address in created_store_subsciption"
    );
    assert!(
        created_store_subsciption.trial_start_date.is_none(),
        "trial_start_date in created_store_subsciption"
    );
    assert!(
        created_store_subsciption.trial_end_date.is_none(),
        "trial_start_date in created_store_subsciption"
    );

    assert_eq!(
        retrieved_store_subsciption.store_id, store.raw_id,
        "store_id in retrieved_store_subsciption"
    );
    assert_eq!(
        retrieved_store_subsciption.currency,
        get_store_subscription::Currency::EUR,
        "currency in retrieved_store_subsciption"
    );
    assert_eq_f64!(retrieved_store_subsciption.value, 0.03, 3);
    assert!(
        retrieved_store_subsciption.wallet_address.is_none(),
        "wallet_address in retrieved_store_subsciption"
    );
    assert!(
        retrieved_store_subsciption.trial_start_date.is_none(),
        "trial_start_date in retrieved_store_subsciption"
    );
    assert!(
        retrieved_store_subsciption.trial_end_date.is_none(),
        "trial_start_date in retrieved_store_subsciption"
    );
}

#[test]
pub fn update_store_subscription() {
    //given
    let mut context = TestContext::new();
    let (_user, token, store, _category) = set_up_store(&mut context).expect("set_up_user failed");
    context.set_bearer(token);
    let _created_store_subsciption = context
        .request(CreateStoreSubscriptionInput {
            store_id: store.raw_id,
            currency: create_store_subscription::Currency::EUR,
            ..default_create_store_subscription_input()
        })
        .expect("Create store subscription failed");
    //when
    let updated_store_subsciption = context
        .request(UpdateStoreSubscriptionInput {
            store_id: store.raw_id,
            currency: Some(update_store_subscription::Currency::STQ),
            ..default_update_store_subscription_input()
        })
        .expect("Update store subscription failed");
    let retrieved_store_subsciption = context
        .request(GetStoreSubscriptionInput {
            store_id: store.raw_id,
        })
        .expect("Get store subscription failed")
        .expect("Get store subscription returned none");
    //then
    assert_eq!(
        updated_store_subsciption.store_id, store.raw_id,
        "store_id in updated_store_subsciption"
    );
    assert_eq!(
        updated_store_subsciption.currency,
        update_store_subscription::Currency::STQ,
        "currency in updated_store_subsciption"
    );
    assert_eq_f64!(updated_store_subsciption.value, 1.0, 1);
    assert!(
        updated_store_subsciption.wallet_address.is_some(),
        "wallet_address in updated_store_subsciption"
    );
    assert!(
        updated_store_subsciption.trial_start_date.is_none(),
        "trial_start_date in updated_store_subsciption"
    );
    assert!(
        updated_store_subsciption.trial_end_date.is_none(),
        "trial_start_date in updated_store_subsciption"
    );

    assert_eq!(
        retrieved_store_subsciption.store_id, store.raw_id,
        "store_id in retrieved_store_subsciption"
    );
    assert_eq!(
        retrieved_store_subsciption.currency,
        get_store_subscription::Currency::STQ,
        "currency in retrieved_store_subsciption"
    );
    assert_eq_f64!(retrieved_store_subsciption.value, 1.0, 1);
    assert!(
        retrieved_store_subsciption.wallet_address.is_some(),
        "wallet_address in retrieved_store_subsciption"
    );
    assert!(
        retrieved_store_subsciption.trial_start_date.is_none(),
        "trial_start_date in retrieved_store_subsciption"
    );
    assert!(
        retrieved_store_subsciption.trial_end_date.is_none(),
        "trial_start_date in retrieved_store_subsciption"
    );
}
