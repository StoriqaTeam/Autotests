use failure::Error as FailureError;

use functional_tests::context::TestContext;
use functional_tests::query::create_user_delivery_address_full::*;
use functional_tests::query::delete_user_delivery_address_full::*;
use functional_tests::query::get_user_delivery_addresses_full::*;
use functional_tests::query::update_user_delivery_address_full::*;
use functional_tests::query::*;

use common::*;

#[test]
fn create_user_address() {
    // setup
    let mut context = TestContext::new();

    // given
    let UserDeliveryAddress { token, .. } =
        set_up_user_address(&mut context).expect("Cannot get data from set_up_user_address");
    context.set_bearer(token);

    // when
    let get_delivery_address = context
        .request(GetUserDeliveryAddressesInput)
        .expect("Cannot get data from get_delivery_address")
        .expect("Empty data from get_delivery_address");

    // then
    let delivery_addresses = get_delivery_address
        .delivery_addresses_full
        .expect("Empty delivery_addresses_full from get_delivery_address");
    assert_eq!(delivery_addresses.len(), 1);
    let address = delivery_addresses.first().unwrap().address.clone();
    assert_eq!(address.value, Some("Value".to_string()));
    assert_eq!(address.country, Some("Country".to_string()));
    assert_eq!(address.country_code, Some("Country Code".to_string()));
    assert_eq!(
        address.administrative_area_level1,
        Some("Administrative Area Level 1".to_string())
    );
    assert_eq!(
        address.administrative_area_level2,
        Some("Administrative Area Level 2".to_string())
    );
    assert_eq!(address.locality, Some("Locality".to_string()));
    assert_eq!(address.political, Some("Political".to_string()));
    assert_eq!(address.postal_code, Some("Postal Code".to_string()));
    assert_eq!(address.route, Some("Route".to_string()));
    assert_eq!(address.street_number, Some("Street Number".to_string()));
    // assert_eq!(address.place_id, Some("Place ID".to_string()));
}

#[test]
fn update_user_address() {
    // setup
    let mut context = TestContext::new();

    // given
    let UserDeliveryAddress {
        delivery_address,
        token,
    } = set_up_user_address(&mut context).expect("Cannot get data from set_up_user_address");

    // when
    context.as_superadmin();
    context
        .request(UpdateUserDeliveryAddressFullInput {
            id: delivery_address.id,
            is_priority: Some(true),
            address_full: update_user_delivery_address_full::AddressInput {
                value: Some("New Value".to_string()),
                country: Some("New Country".to_string()),
                country_code: Some("New Country Code".to_string()),
                administrative_area_level1: Some("New Administrative Area Level 1".to_string()),
                administrative_area_level2: Some("New Administrative Area Level 2".to_string()),
                locality: Some("New Locality".to_string()),
                political: Some("New Political".to_string()),
                postal_code: Some("New Postal Code".to_string()),
                route: Some("New Route".to_string()),
                street_number: Some("New Street Number".to_string()),
                place_id: Some("New Place ID".to_string()),
                ..update_user_delivery_address_full::default_address_input()
            },
            ..default_update_user_delivery_address_full_input()
        })
        .expect("Cannot get data from update_user_delivery_address_full");
    context.set_bearer(token);
    let get_delivery_address = context
        .request(GetUserDeliveryAddressesInput)
        .expect("Cannot get data from get_delivery_address")
        .expect("Empty data from get_delivery_address");

    // then
    let delivery_addresses = get_delivery_address
        .delivery_addresses_full
        .expect("Empty delivery_addresses_full from get_delivery_address");
    assert_eq!(delivery_addresses.len(), 1);
    let address = delivery_addresses.first().unwrap().address.clone();
    assert_eq!(address.value, Some("New Value".to_string()));
    assert_eq!(address.country, Some("New Country".to_string()));
    assert_eq!(address.country_code, Some("New Country Code".to_string()));
    assert_eq!(
        address.administrative_area_level1,
        Some("New Administrative Area Level 1".to_string())
    );
    assert_eq!(
        address.administrative_area_level2,
        Some("New Administrative Area Level 2".to_string())
    );
    assert_eq!(address.locality, Some("New Locality".to_string()));
    assert_eq!(address.political, Some("New Political".to_string()));
    assert_eq!(address.postal_code, Some("New Postal Code".to_string()));
    assert_eq!(address.route, Some("New Route".to_string()));
    assert_eq!(address.street_number, Some("New Street Number".to_string()));
    // assert_eq!(address.place_id, Some("New Place ID".to_string()));
}

#[test]
fn delete_user_address() {
    // setup
    let mut context = TestContext::new();

    // given
    let UserDeliveryAddress {
        delivery_address,
        token,
    } = set_up_user_address(&mut context).expect("Cannot get data from set_up_user_address");

    // when
    context.as_superadmin();
    context
        .request(DeleteUserDeliveryAddressFullInput {
            id: delivery_address.raw_id,
        })
        .expect("Cannot get data from delete_user_delivery_address_full");
    let delete_delivery_address_twice = context.request(DeleteUserDeliveryAddressFullInput {
        id: delivery_address.raw_id,
    });
    context.set_bearer(token);
    let get_delivery_address = context
        .request(GetUserDeliveryAddressesInput)
        .expect("Cannot get data from get_delivery_address")
        .expect("Empty data from get_delivery_address");

    // then
    if delete_delivery_address_twice.is_ok() {
        panic!("Should not be able to delete the same delivery address twice");
    }
    if get_delivery_address.delivery_addresses_full.is_some()
        && get_delivery_address.delivery_addresses_full.unwrap().len() != 0
    {
        panic!("Should not be able to get deleted address");
    }
}

pub fn set_up_user_address(context: &mut TestContext) -> Result<UserDeliveryAddress, FailureError> {
    let (user, token) = set_up_user(context)?;
    context.as_superadmin();
    let delivery_address = context.request(NewUserDeliveryAddressFullInput {
        user_id: user.raw_id,
        ..default_create_user_delivery_address_full_input()
    })?;
    context.clear_bearer();
    Ok(UserDeliveryAddress {
        delivery_address,
        token,
    })
}

pub struct UserDeliveryAddress {
    delivery_address: create_user_delivery_address_full::GraphqlRequestOutput,
    token: String,
}
