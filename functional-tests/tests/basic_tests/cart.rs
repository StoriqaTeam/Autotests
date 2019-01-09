use std::collections::HashSet;

use functional_tests::context::TestContext;
use functional_tests::defaults::*;
use functional_tests::query::*;

use common::*;

#[test]
fn delete_from_cart() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _created_store, _category, _base_product, created_product) =
        set_up_published_product(&mut context).expect("set_up_published_product failed");
    context.set_bearer(token);
    let _ = context
        .request(add_in_cart_v2::AddInCartInputV2 {
            product_id: created_product.raw_id,
            ..add_in_cart_v2::default_add_in_cart_v2_input()
        })
        .expect("add_in_cart_v2 failed");
    //when
    let _ = context
        .request(delete_from_cart::DeleteFromCartInput {
            product_id: created_product.raw_id,
            ..delete_from_cart::default_delete_from_cart_input()
        })
        .expect("delete_from_cart failed");
    //then
    let user_cart = context
        .request(get_cart_v2::default_get_cart_v2_input())
        .expect("get_cart_v2 failed for user_cart")
        .expect("Cannot user_cart");
    assert!(user_cart.get_product(created_product.raw_id).is_none());
}

#[test]
pub fn add_in_cart() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, _token, created_store, _category, _base_product, created_product) =
        set_up_published_product(&mut context).expect("set_up_published_product failed");
    let buyer = context
        .request(create_user::CreateUserInput {
            email: "buyer@email.com".to_string(),
            ..create_user::default_create_user_input()
        })
        .expect("create_user failed for buyer");
    context
        .verify_user_email(&buyer.email)
        .expect("verify_user_email failed for buyer");;
    let buyer_token: String = context
        .request(get_jwt_by_email::CreateJWTEmailInput {
            email: buyer.email,
            ..get_jwt_by_email::default_create_jwt_email_input()
        })
        .expect("get_jwt_by_email failed for buyer")
        .token;
    context.set_bearer(buyer_token);
    //when
    let _ = context
        .request(add_in_cart_v2::AddInCartInputV2 {
            product_id: created_product.raw_id,
            value: Some(10),
            ..add_in_cart_v2::default_add_in_cart_v2_input()
        })
        .expect("add_in_cart_v2 failed");
    //then
    let cart = context
        .request(get_cart_v2::default_get_cart_v2_input())
        .expect("get_cart_v2 failed for user_cart");
    assert!(cart.is_some(), "add_in_cart_v2 returned None");
    let mut cart = cart.expect("add_in_cart_v2 returned None");
    let store = cart.stores.edges.pop();
    assert!(store.is_some(), "cart returned no stores");
    let mut store = store.expect("cart returned no stores").node;
    assert_eq!(store.raw_id, created_store.raw_id);
    let product = store.products.pop();
    assert!(product.is_some(), "store returned no products");
    let product = product.expect("store returned no products");
    assert_eq!(product.raw_id, created_product.raw_id);
    assert_eq!(product.quantity, 10);
}

fn check_exists_delivery_method_in_cart(
    context: &mut TestContext,
    store_1: &Store,
    shipping_id: i64,
) {
    let user_cart = context
        .request(get_cart_v2::default_get_cart_v2_input())
        .expect("get_cart_v2 failed for user_cart")
        .expect("Cannot user_cart");

    let cart_stores = user_cart.stores;
    let cart_edges = cart_stores.edges;
    let cart_products = cart_edges
        .into_iter()
        .map(|edge| edge.node.products)
        .flatten();
    let find_product_id_with_delivery = store_1.product_1.product_1.raw_id;

    //then
    assert_eq!(
        cart_products
            .into_iter()
            .find(|product| product.raw_id == find_product_id_with_delivery)
            .map(|find_product| find_product
                .select_package
                .map(|package| package.shipping_id))
            .unwrap(),
        Some(shipping_id)
    );
}

#[test]
pub fn set_delivery_method_in_cart() {
    //setup
    let mut context = TestContext::new();
    //given
    let store_1 = create_store_with_several_products(&mut context, "store-1")
        .expect("create_store_with_several_products failed to create store 1");
    context.set_bearer(store_1.token.clone());

    let warehouse_payload = create_warehouse::CreateWarehouseInput {
        name: Some("Warehouse".to_string()),
        store_id: store_1.store.raw_id,
        address_full: create_warehouse::AddressInput {
            country: Some("Russian Federation".to_string()),
            country_code: Some("RUS".to_string()),
            ..create_warehouse::default_address_input()
        },
        ..create_warehouse::default_create_warehouse_input()
    };
    let _warehouse = context
        .request(warehouse_payload)
        .expect("Cannot get data from create_warehouse");

    context.as_superadmin();

    let company_payload = create_delivery_company::NewCompanyInput {
        name: "Test company".to_string(),
        label: "TEST".to_string(),
        description: Some("Test description".to_string()),
        deliveries_from: default_deliveries_from(),
        logo: "test loge URL".to_string(),
        ..create_delivery_company::default_create_company_input()
    };

    let new_company = context
        .request(company_payload.clone())
        .expect("Cannot get data from create_delivery_company");

    let new_package = context
        .request(create_package::NewPackagesInput {
            name: "Initial name".to_string(),
            deliveries_to: default_deliveries_to(),
            ..create_package::default_create_package_input()
        })
        .expect("Cannot get data from create_package");

    let company_package = context
        .request(add_package_to_company::NewCompaniesPackagesInput {
            company_id: new_company.raw_id,
            package_id: new_package.raw_id,
            ..add_package_to_company::default_add_package_to_company_input()
        })
        .expect("Cannot get data from add_package_to_company");
    println!("company_package: {:#?}", company_package);
    context.set_bearer(store_1.token.clone());

    let upsert_shipping_payload = upsert_shipping::NewShippingInput {
        store_id: store_1.store.raw_id,
        base_product_id: store_1.product_1.base_product.raw_id,
        local: vec![upsert_shipping::NewLocalShippingProductsInput {
            company_package_id: company_package.raw_id,
            price: Some(10.0),
        }],
        ..upsert_shipping::default_upsert_shipping_input()
    };
    let _upsert_shipping = context
        .request(upsert_shipping_payload)
        .expect("Cannot get data from upsert_shipping");
    println!("_upsert_shipping: {:#?}", _upsert_shipping);
    let buyer_1 =
        create_user_with_products_in_carts(&mut context, "buyer-1@examplemail.com", &[&store_1])
            .expect("create_user_with_products_in_carts failed to create buyer 1");
    context.set_bearer(buyer_1.token);

    //when
    let available_shipping_package = context
        .request(
            get_available_shipping_for_user::GetAvailableShippingForUserInput {
                user_country_code: "RUS".to_string(),
                base_product_id: store_1.product_1.base_product.raw_id,
            },
        )
        .expect("Cannot get data from available_shipping_for_user")
        .packages
        .pop()
        .expect("Available package not exists");
    println!(
        "available_shipping_package: {:#?}",
        available_shipping_package
    );
    let _delivery_payload =
        context.request(set_delivery_method_in_cart::SetDeliveryMethodInCartInput {
            product_id: store_1.product_1.product_1.raw_id,
            shipping_id: available_shipping_package.shipping_id,
            ..set_delivery_method_in_cart::default_set_delivery_method_in_cart_input()
        });
    //then
    check_exists_delivery_method_in_cart(
        &mut context,
        &store_1,
        available_shipping_package.shipping_id,
    );
}

#[test]
pub fn clear_delivery_method_in_carts_users() {
    //setup
    let mut context = TestContext::new();
    //given
    let store_1 = create_store_with_several_products(&mut context, "store-1")
        .expect("create_store_with_several_products failed to create store 1");
    context.set_bearer(store_1.token.clone());

    let warehouse_payload = create_warehouse::CreateWarehouseInput {
        name: Some("Warehouse".to_string()),
        store_id: store_1.store.raw_id,
        address_full: create_warehouse::AddressInput {
            country: Some("Russian Federation".to_string()),
            country_code: Some("RUS".to_string()),
            ..create_warehouse::default_address_input()
        },
        ..create_warehouse::default_create_warehouse_input()
    };
    let _warehouse = context
        .request(warehouse_payload)
        .expect("Cannot get data from create_warehouse");

    context.as_superadmin();

    let company_payload = create_delivery_company::NewCompanyInput {
        name: "Test company".to_string(),
        label: "TEST".to_string(),
        description: Some("Test description".to_string()),
        deliveries_from: default_deliveries_from(),
        logo: "test loge URL".to_string(),
        ..create_delivery_company::default_create_company_input()
    };

    let new_company = context
        .request(company_payload.clone())
        .expect("Cannot get data from create_delivery_company");

    let new_package = context
        .request(create_package::NewPackagesInput {
            name: "Initial name".to_string(),
            deliveries_to: default_deliveries_to(),
            ..create_package::default_create_package_input()
        })
        .expect("Cannot get data from create_package");

    let company_package = context
        .request(add_package_to_company::NewCompaniesPackagesInput {
            company_id: new_company.raw_id,
            package_id: new_package.raw_id,
            ..add_package_to_company::default_add_package_to_company_input()
        })
        .expect("Cannot get data from add_package_to_company");
    println!("company_package: {:#?}", company_package);

    context.set_bearer(store_1.token.clone());

    let upsert_shipping_payload = upsert_shipping::NewShippingInput {
        store_id: store_1.store.raw_id,
        base_product_id: store_1.product_1.base_product.raw_id,
        local: vec![upsert_shipping::NewLocalShippingProductsInput {
            company_package_id: company_package.raw_id,
            price: Some(10.0),
        }],
        ..upsert_shipping::default_upsert_shipping_input()
    };
    let _upsert_shipping = context
        .request(upsert_shipping_payload)
        .expect("Cannot get data from upsert_shipping");
    println!("_upsert_shipping: {:#?}", _upsert_shipping);
    let buyer_1 =
        create_user_with_products_in_carts(&mut context, "buyer-1@examplemail.com", &[&store_1])
            .expect("create_user_with_products_in_carts failed to create buyer 1");

    context.set_bearer(buyer_1.token.clone());

    //when
    let available_shipping_package = context
        .request(
            get_available_shipping_for_user::GetAvailableShippingForUserInput {
                user_country_code: "RUS".to_string(),
                base_product_id: store_1.product_1.base_product.raw_id,
            },
        )
        .expect("Cannot get data from available_shipping_for_user")
        .packages
        .pop()
        .expect("Available package not exists");
    println!(
        "available_shipping_package: {:#?}",
        available_shipping_package
    );
    let _delivery_payload =
        context.request(set_delivery_method_in_cart::SetDeliveryMethodInCartInput {
            product_id: store_1.product_1.product_1.raw_id,
            shipping_id: available_shipping_package.shipping_id,
            ..set_delivery_method_in_cart::default_set_delivery_method_in_cart_input()
        });

    check_exists_delivery_method_in_cart(
        &mut context,
        &store_1,
        available_shipping_package.shipping_id,
    );

    context.set_bearer(store_1.token.clone());

    // single update delivery
    let upsert_shipping_payload = upsert_shipping::NewShippingInput {
        store_id: store_1.store.raw_id,
        base_product_id: store_1.product_1.base_product.raw_id,
        local: vec![upsert_shipping::NewLocalShippingProductsInput {
            company_package_id: company_package.raw_id,
            price: Some(100.0),
        }],
        ..upsert_shipping::default_upsert_shipping_input()
    };

    let _update_upsert_shipping = context
        .request(upsert_shipping_payload)
        .expect("Cannot get data from update upsert_shipping");
    println!("_update_upsert_shipping: {:#?}", _update_upsert_shipping);

    context.set_bearer(buyer_1.token.clone());

    let user_cart = context
        .request(get_cart_v2::default_get_cart_v2_input())
        .expect("get_cart_v2 failed for user_cart")
        .expect("Cannot user_cart");

    let cart_stores = user_cart.stores;
    let cart_edges = cart_stores.edges;
    let cart_products = cart_edges
        .into_iter()
        .map(|edge| edge.node.products)
        .flatten();
    let find_product_id_with_delivery = store_1.product_1.product_1.raw_id;

    //then
    assert!(
        cart_products
            .into_iter()
            .find(|product| product.raw_id == find_product_id_with_delivery)
            .map(|find_product| find_product
                .select_package
                .map(|package| package.shipping_id))
            .expect("product was not found in cart_products")
            .is_none(),
        "Delivery method should be none"
    );
}

#[test]
fn set_selection_in_cart() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _created_store, _category, _base_product, created_product) =
        set_up_published_product(&mut context).expect("set_up_published_product failed");
    context.set_bearer(token);
    let _ = context
        .request(add_in_cart_v2::AddInCartInputV2 {
            product_id: created_product.raw_id,
            ..add_in_cart_v2::default_add_in_cart_v2_input()
        })
        .expect("add_in_cart_v2 failed");
    //then
    check_changed_selection(&mut context, created_product.raw_id, false);
    check_changed_selection(&mut context, created_product.raw_id, true);
}

fn check_changed_selection(context: &mut TestContext, product_id: i64, selected: bool) {
    //when
    context
        .request(set_selection_in_cart::SetSelectionInCartInput {
            product_id: product_id,
            value: selected,
            ..set_selection_in_cart::default_set_selection_in_cart_input()
        })
        .expect("set_quantity_in_cart failed");
    //then
    let user_cart = context
        .request(get_cart_v2::default_get_cart_v2_input())
        .expect("get_cart_v2 failed for user_cart")
        .expect("Cannot user_cart");
    assert_eq!(
        user_cart
            .get_product(product_id)
            .expect("product not found in cart")
            .selected,
        selected
    );
}

#[test]
pub fn delete_products_from_all_carts_during_various_scenarios() {
    //setup
    let mut context = TestContext::new();
    //given
    let store_1 = create_store_with_several_products(&mut context, "store-1")
        .expect("create_store_with_several_products failed to create store 1");
    let store_2 = create_store_with_several_products(&mut context, "store-2")
        .expect("create_store_with_several_products failed to create store 2");
    let buyer_1 =
        create_user_with_products_in_carts(&mut context, "buyer-1@mail.com", &[&store_1, &store_2])
            .expect("create_user_with_products_in_carts failed to create buyer 1");
    let buyer_2 =
        create_user_with_products_in_carts(&mut context, "buyer-2@mail.com", &[&store_1, &store_2])
            .expect("create_user_with_products_in_carts failed to create buyer 2");
    //then
    check_delete_product_from_carts_when_product_is_deactivated(
        &mut context,
        &store_1,
        &store_2,
        &buyer_1,
        &buyer_2,
    );
    check_delete_products_from_carts_when_base_product_is_deactivated(
        &mut context,
        &store_1,
        &store_2,
        &buyer_1,
        &buyer_2,
    );
    check_delete_products_from_carts_when_store_is_deactivated(
        &mut context,
        &store_1,
        &store_2,
        &buyer_1,
        &buyer_2,
    );
    check_delete_products_from_carts_when_base_product_status_is_changed(
        &mut context,
        &store_1,
        &store_2,
        &buyer_1,
        &buyer_2,
    );
    check_delete_products_from_carts_when_store_status_is_changed(
        &mut context,
        &store_1,
        &store_2,
        &buyer_1,
        &buyer_2,
    );
}

fn check_delete_products_from_carts_when_store_status_is_changed(
    context: &mut TestContext,
    _store_1: &Store,
    store_2: &Store,
    buyer_1: &User,
    buyer_2: &User,
) {
    //when
    context.as_superadmin();
    let _ = context
        .request(set_moderation_status_store::StoreModerateInput {
            id: store_2.store.id.clone(),
            status: set_moderation_status_store::Status::DRAFT,
        })
        .expect("set_moderation_status_store failed");
    //then
    let desired_products_in_cart: HashSet<i64> = HashSet::new(); //totaly empty
                                                                 //first buyer
    assert_eq!(
        user_has_products_in_cart(context, buyer_1.token.clone()),
        desired_products_in_cart,
        "products mismatch for buyer_1"
    );
    //second buyer
    assert_eq!(
        user_has_products_in_cart(context, buyer_2.token.clone()),
        desired_products_in_cart,
        "products mismatch for buyer_2"
    );
}

fn check_delete_products_from_carts_when_base_product_status_is_changed(
    context: &mut TestContext,
    _store_1: &Store,
    store_2: &Store,
    buyer_1: &User,
    buyer_2: &User,
) {
    //when
    context.as_superadmin();
    let _ = context
        .request(
            set_moderation_status_base_product::BaseProductModerateInput {
                id: store_2.product_1.base_product.id.clone(),
                status: set_moderation_status_base_product::Status::DRAFT,
            },
        )
        .expect("set_moderation_status_base_product failed");
    //then
    let desired_products_in_cart: HashSet<i64> = vec![
        //without store_1.product_1.product_1
        //without store_1.product_1.product_2
        //without store_1.product_2.product_1
        //without store_1.product_2.product_2
        //without store_2.product_1.product_1
        //without store_2.product_1.product_2.
        store_2.product_2.product_1.raw_id,
        store_2.product_2.product_2.raw_id,
    ]
    .into_iter()
    .collect();
    //first buyer
    assert_eq!(
        user_has_products_in_cart(context, buyer_1.token.clone()),
        desired_products_in_cart,
        "products mismatch for buyer_1"
    );
    //second buyer
    assert_eq!(
        user_has_products_in_cart(context, buyer_2.token.clone()),
        desired_products_in_cart,
        "products mismatch for buyer_2"
    );
}

fn check_delete_products_from_carts_when_store_is_deactivated(
    context: &mut TestContext,
    store_1: &Store,
    store_2: &Store,
    buyer_1: &User,
    buyer_2: &User,
) {
    //when
    context.set_bearer(store_1.token.clone());
    let _ = context
        .request(deactivate_store::DeactivateStoreInput {
            id: store_1.store.id.clone(),
            ..deactivate_store::default_deactivate_store_input()
        })
        .expect("deactivate_store failed");
    //then
    let desired_products_in_cart: HashSet<i64> = vec![
        //without store_1.product_1.product_1
        //without store_1.product_1.product_2
        //without store_1.product_2.product_1
        //without store_1.product_2.product_2
        store_2.product_1.product_1.raw_id,
        store_2.product_1.product_2.raw_id,
        store_2.product_2.product_1.raw_id,
        store_2.product_2.product_2.raw_id,
    ]
    .into_iter()
    .collect();
    //first buyer
    assert_eq!(
        user_has_products_in_cart(context, buyer_1.token.clone()),
        desired_products_in_cart,
        "products mismatch for buyer_1"
    );
    //second buyer
    assert_eq!(
        user_has_products_in_cart(context, buyer_2.token.clone()),
        desired_products_in_cart,
        "products mismatch for buyer_2"
    );
}

fn check_delete_products_from_carts_when_base_product_is_deactivated(
    context: &mut TestContext,
    store_1: &Store,
    store_2: &Store,
    buyer_1: &User,
    buyer_2: &User,
) {
    //when
    context.set_bearer(store_1.token.clone());
    let _ = context
        .request(deactivate_base_product::DeactivateBaseProductInput {
            id: store_1.product_1.base_product.id.clone(),
            ..deactivate_base_product::default_deactivate_base_product_input()
        })
        .expect("deactivate_base_product failed");
    //then
    let desired_products_in_cart: HashSet<i64> = vec![
        //without store_1.product_1.product_1
        //without store_1.product_1.product_2
        store_1.product_2.product_1.raw_id,
        store_1.product_2.product_2.raw_id,
        store_2.product_1.product_1.raw_id,
        store_2.product_1.product_2.raw_id,
        store_2.product_2.product_1.raw_id,
        store_2.product_2.product_2.raw_id,
    ]
    .into_iter()
    .collect();
    //first buyer
    assert_eq!(
        user_has_products_in_cart(context, buyer_1.token.clone()),
        desired_products_in_cart,
        "products mismatch for buyer_1"
    );
    //second buyer
    assert_eq!(
        user_has_products_in_cart(context, buyer_2.token.clone()),
        desired_products_in_cart,
        "products mismatch for buyer_2"
    );
}

fn check_delete_product_from_carts_when_product_is_deactivated(
    context: &mut TestContext,
    store_1: &Store,
    store_2: &Store,
    buyer_1: &User,
    buyer_2: &User,
) {
    //when
    context.set_bearer(store_1.token.clone());
    let _ = context
        .request(deactivate_product::DeactivateProductInput {
            id: store_1.product_1.product_1.id.clone(),
            ..deactivate_product::default_deactivate_product_input()
        })
        .expect("deactivate_product failed");
    //then
    let desired_products_in_cart: HashSet<i64> = vec![
        //without store_1.product_1.product_1
        store_1.product_1.product_2.raw_id,
        store_1.product_2.product_1.raw_id,
        store_1.product_2.product_2.raw_id,
        store_2.product_1.product_1.raw_id,
        store_2.product_1.product_2.raw_id,
        store_2.product_2.product_1.raw_id,
        store_2.product_2.product_2.raw_id,
    ]
    .into_iter()
    .collect();
    //first buyer
    assert_eq!(
        user_has_products_in_cart(context, buyer_1.token.clone()),
        desired_products_in_cart,
        "products mismatch for buyer_1"
    );
    //second buyer
    assert_eq!(
        user_has_products_in_cart(context, buyer_2.token.clone()),
        desired_products_in_cart,
        "products mismatch for buyer_2"
    );
}

#[test]
fn set_quantity_in_cart() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _created_store, _category, _base_product, created_product) =
        set_up_published_product(&mut context).expect("set_up_published_product failed");
    context.set_bearer(token);
    let _ = context
        .request(add_in_cart_v2::AddInCartInputV2 {
            product_id: created_product.raw_id,
            ..add_in_cart_v2::default_add_in_cart_v2_input()
        })
        .expect("add_in_cart_v2 failed");
    //when
    context
        .request(set_quantity_in_cart::SetQuantityInCartInput {
            product_id: created_product.raw_id,
            value: 13,
            ..set_quantity_in_cart::default_set_quantity_in_cart_input()
        })
        .expect("set_quantity_in_cart failed");
    //then
    let user_cart = context
        .request(get_cart_v2::default_get_cart_v2_input())
        .expect("get_cart_v2 failed for user_cart")
        .expect("Cannot user_cart");
    assert_eq!(
        user_cart
            .get_product(created_product.raw_id)
            .expect("product not found in cart")
            .quantity,
        13
    );
}

fn user_has_products_in_cart(context: &mut TestContext, user_token: String) -> HashSet<i64> {
    context.set_bearer(user_token);
    let user_cart = context
        .request(get_cart_v2::default_get_cart_v2_input())
        .expect("get_cart_v2 failed for user_cart");
    user_cart
        .expect("get_cart_v2 returned None for user")
        .stores
        .edges
        .into_iter()
        .flat_map(|e| e.node.products)
        .map(|product| product.raw_id)
        .collect()
}
