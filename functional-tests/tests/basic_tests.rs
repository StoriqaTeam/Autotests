extern crate failure;
extern crate functional_tests;

use std::collections::HashSet;

use failure::Error as FailureError;

use functional_tests::query::*;

use functional_tests::context::TestContext;

#[test]
pub fn change_password() {
    //setup
    let mut context = TestContext::new();
    //given
    let (user, token) = set_up_user(&mut context).expect("set_up_user failed");
    //when
    context.set_bearer(token);
    let response = context
        .request(change_password::default_change_password_input())
        .expect("change_password failed");
    //then
    assert!(response.success, "change password failed");
    context.set_bearer(response.token);
    let me = context
        .request(get_me::GetMeInput {})
        .expect("get_me failed")
        .expect("get_me returned nothing");
    assert_eq!(
        me.email, user.email,
        "change password returned wrong auth token"
    );
}

#[test]
pub fn reset_password() {
    //setup
    let mut context = TestContext::new();
    //given
    let (user, token) = set_up_user(&mut context).expect("set_up_user failed");
    context.set_bearer(token);
    let password_reset = context
        .request(request_password_reset::ResetRequest {
            email: user.email.clone(),
            ..request_password_reset::default_change_password_input()
        })
        .expect("request_password_reset failed");
    assert!(password_reset.success, "reset password failed");
    context.as_superadmin();
    let reset_token = context
        .request(get_existing_reset_token::ExistingResetTokenInput {
            user_id: user.raw_id,
            token_type: get_existing_reset_token::TokenTypeInput::PASSWORD_RESET,
        })
        .expect("get_existing_reset_token failed")
        .token;
    //when
    context.clear_bearer();
    let response = context
        .request(apply_password_reset::ResetApply {
            token: reset_token,
            ..apply_password_reset::default_apply_password_reset_input()
        })
        .expect("apply_password_reset failed");
    //then
    assert!(response.success, "reset password failed");
    context.set_bearer(response.token);
    let me = context
        .request(get_me::GetMeInput {})
        .expect("get_me failed")
        .expect("get_me returned nothing");
    assert_eq!(
        me.email, user.email,
        "reset password returned wrong auth token"
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

fn create_user_with_products_in_carts(
    context: &mut TestContext,
    email: &str,
    stores: &[&Store],
) -> Result<User, FailureError> {
    //create user
    let user = context
        .request(create_user::CreateUserInput {
            email: email.to_string(),
            ..create_user::default_create_user_input()
        })
        .expect("create_user failed for user");
    context
        .verify_user_email(&user.email)
        .expect("verify_user_email failed for user");;
    let token: String = context
        .request(get_jwt_by_email::CreateJWTEmailInput {
            email: user.email.clone(),
            ..get_jwt_by_email::default_create_jwt_email_input()
        })
        .expect("get_jwt_by_email failed for user")
        .token;
    context.set_bearer(token.clone());
    // add products to cart
    for store in stores {
        let _ = context.request(add_in_cart_v2::AddInCartInputV2 {
            product_id: store.product_1.product_1.raw_id,
            ..add_in_cart_v2::default_add_in_cart_v2_input()
        })?;
        let _ = context.request(add_in_cart_v2::AddInCartInputV2 {
            product_id: store.product_1.product_2.raw_id,
            ..add_in_cart_v2::default_add_in_cart_v2_input()
        })?;
        let _ = context.request(add_in_cart_v2::AddInCartInputV2 {
            product_id: store.product_2.product_1.raw_id,
            ..add_in_cart_v2::default_add_in_cart_v2_input()
        })?;
        let _ = context.request(add_in_cart_v2::AddInCartInputV2 {
            product_id: store.product_2.product_2.raw_id,
            ..add_in_cart_v2::default_add_in_cart_v2_input()
        })?;
    }

    Ok(User { token })
}

fn create_store_with_several_products(
    context: &mut TestContext,
    store_name: &str,
) -> Result<Store, FailureError> {
    //create user
    let owner = context.request(create_user::CreateUserInput {
        email: format!("{}-manger@mail.com", store_name),
        ..create_user::default_create_user_input()
    })?;
    context.verify_user_email(&owner.email)?;
    let token: String = context
        .request(get_jwt_by_email::CreateJWTEmailInput {
            email: owner.email.clone(),
            ..get_jwt_by_email::default_create_jwt_email_input()
        })?
        .token;
    //create store
    context.set_bearer(token.clone());
    let store = context.request(create_store::CreateStoreInput {
        user_id: owner.raw_id,
        slug: store_name.to_string(),
        ..create_store::default_create_store_input()
    })?;
    //publish store
    let _ = context.request(send_store_to_moderation::SendStoreToModerationInput {
        raw_id: store.raw_id,
    })?;
    context.as_superadmin();
    let _ = context.request(set_moderation_status_store::StoreModerateInput {
        id: store.id.clone(),
        status: set_moderation_status_store::Status::PUBLISHED,
    })?;
    //create categories
    context.as_superadmin();
    let category_level_1 = context.request(create_category::CreateCategoryInput {
        slug: Some(format!("{}-categor-slug-1", store_name)),
        ..create_category::default_create_category_input()
    })?;
    let category_level_2 = context.request(create_category::CreateCategoryInput {
        parent_id: category_level_1.raw_id,
        slug: Some(format!("{}-categor-slug-2", store_name)),
        ..create_category::default_create_category_input()
    })?;
    let category_level_3 = context.request(create_category::CreateCategoryInput {
        parent_id: category_level_2.raw_id,
        slug: Some(format!("{}-categor-slug-3", store_name)),
        ..create_category::default_create_category_input()
    })?;
    //create products
    context.set_bearer(token.clone());
    let product_1 = create_base_broduct_with_two_products(
        context,
        store_name,
        "product-1",
        store.raw_id,
        category_level_3.raw_id,
    )?;
    let product_2 = create_base_broduct_with_two_products(
        context,
        store_name,
        "product-2",
        store.raw_id,
        category_level_3.raw_id,
    )?;
    //publish products
    let _ = context
        .request(
            send_base_product_to_moderation::SendBaseProductToModerationInput {
                raw_id: product_1.base_product.raw_id,
            },
        )
        .expect("send_base_product_to_moderation failed to send to moderation");
    let _ = context
        .request(
            send_base_product_to_moderation::SendBaseProductToModerationInput {
                raw_id: product_2.base_product.raw_id,
            },
        )
        .expect("send_base_product_to_moderation failed to send to moderation");

    context.as_superadmin();
    let _ = context
        .request(
            set_moderation_status_base_product::BaseProductModerateInput {
                id: product_1.base_product.id.clone(),
                status: set_moderation_status_base_product::Status::PUBLISHED,
            },
        )
        .expect("set_moderation_status_base_product failed");
    context.as_superadmin();
    let _ = context
        .request(
            set_moderation_status_base_product::BaseProductModerateInput {
                id: product_2.base_product.id.clone(),
                status: set_moderation_status_base_product::Status::PUBLISHED,
            },
        )
        .expect("set_moderation_status_base_product failed");

    Ok(Store {
        store,
        product_1,
        product_2,
        token,
    })
}

fn create_base_broduct_with_two_products(
    context: &mut TestContext,
    store_name: &str,
    product_name: &str,
    store_raw_id: i64,
    category_raw_id: i64,
) -> Result<Products, FailureError> {
    let new_base_product = create_base_product::CreateBaseProductInput {
        store_id: store_raw_id,
        category_id: category_raw_id,
        slug: Some(format!("{}-{}-base-product", store_name, product_name)),
        ..create_base_product::default_create_base_product_input()
    };
    let base_product = context.request(new_base_product)?;

    let product_1 = context.request(create_product::CreateProductWithAttributesInput {
        product: create_product::NewProduct {
            base_product_id: Some(base_product.raw_id),
            vendor_code: format!("{}-{}-1", store_name, product_name),
            ..create_product::default_new_product()
        },
        ..create_product::default_create_product_input()
    })?;

    let product_2 = context.request(create_product::CreateProductWithAttributesInput {
        product: create_product::NewProduct {
            base_product_id: Some(base_product.raw_id),
            vendor_code: format!("{}-{}-2", store_name, product_name),
            ..create_product::default_new_product()
        },
        ..create_product::default_create_product_input()
    })?;

    Ok(Products {
        base_product,
        product_1,
        product_2,
    })
}

struct Store {
    store: create_store::RustCreateStoreCreateStore,
    product_1: Products,
    product_2: Products,
    token: String,
}

struct Products {
    base_product: create_base_product::RustCreateBaseProductCreateBaseProduct,
    product_1: create_product::RustCreateProductCreateProduct,
    product_2: create_product::RustCreateProductCreateProduct,
}

struct User {
    token: String,
}

#[test]
pub fn deactivate_store() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _category) = set_up_store(&mut context).expect("set_up_store failed");
    context.set_bearer(token.clone());
    //when
    let _ = context
        .request(deactivate_store::DeactivateStoreInput {
            id: store.id.clone(),
            ..deactivate_store::default_deactivate_store_input()
        })
        .expect("deactivate_store failed");
    //then
    let store = context
        .get_store(store.raw_id)
        .expect("get_store failed")
        .store;
    assert!(store.is_none(), "store should be deactivated")
}

#[test]
pub fn publish_base_product() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("set_up_base_product failed");
    assert_eq!(
        base_product.status,
        create_base_product::Status::DRAFT,
        "Initial status should be draft"
    );
    context.set_bearer(token);
    let _ = context
        .request(
            send_base_product_to_moderation::SendBaseProductToModerationInput {
                raw_id: base_product.raw_id,
            },
        )
        .expect("send_base_product_to_moderation failed to send to moderation");
    //when
    context.as_superadmin();
    let _ = context
        .request(
            set_moderation_status_base_product::BaseProductModerateInput {
                id: base_product.id,
                status: set_moderation_status_base_product::Status::PUBLISHED,
            },
        )
        .expect("set_moderation_status_base_product failed");
    //then
    let updated_base_product = context
        .get_base_product(base_product.raw_id)
        .unwrap()
        .base_product
        .unwrap();
    assert_eq!(
        updated_base_product.status,
        get_base_product::Status::PUBLISHED,
        "final status should be published"
    );
}

#[test]
pub fn send_base_product_to_moderation() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("set_up_base_product failed");
    assert_eq!(
        base_product.status,
        create_base_product::Status::DRAFT,
        "Initial status should be draft"
    );
    context.set_bearer(token);
    //when
    let _ = context
        .request(
            send_base_product_to_moderation::SendBaseProductToModerationInput {
                raw_id: base_product.raw_id,
            },
        )
        .expect("send_base_product_to_moderation failed to send to moderation");
    //then
    let updated_base_product = context
        .get_base_product(base_product.raw_id)
        .unwrap()
        .base_product
        .unwrap();
    assert_eq!(
        updated_base_product.status,
        get_base_product::Status::MODERATION,
        "final status should be moderation"
    );
}

#[test]
pub fn publish_store() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _category) = set_up_store(&mut context).unwrap();
    assert_eq!(
        store.status,
        create_store::Status::DRAFT,
        "Initial status should be draft"
    );
    context.set_bearer(token);
    let _ = context
        .request(send_store_to_moderation::SendStoreToModerationInput {
            raw_id: store.raw_id,
        })
        .expect("send_store_to_moderation failed to send to moderation");
    //when
    context.as_superadmin();
    let _ = context
        .request(set_moderation_status_store::StoreModerateInput {
            id: store.id,
            status: set_moderation_status_store::Status::PUBLISHED,
        })
        .expect("set_moderation_status_store failed");
    //then
    let updated_store = context.get_store(store.raw_id).unwrap().store.unwrap();
    assert_eq!(
        updated_store.status,
        get_store::Status::PUBLISHED,
        "final status should be published"
    );
}

#[test]
pub fn send_store_to_moderation() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _category) = set_up_store(&mut context).unwrap();
    assert_eq!(
        store.status,
        create_store::Status::DRAFT,
        "Initial status should be draft"
    );
    context.set_bearer(token);
    //when
    let _ = context
        .request(send_store_to_moderation::SendStoreToModerationInput {
            raw_id: store.raw_id,
        })
        .expect("send_store_to_moderation failed to send to moderation");
    //then
    let updated_store = context.get_store(store.raw_id).unwrap().store.unwrap();
    assert_eq!(
        updated_store.status,
        get_store::Status::MODERATION,
        "final status should be moderation"
    );
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

#[test]
pub fn verify_email() {
    //setup
    let mut context = TestContext::new();
    //given
    let user = context
        .request(create_user::default_create_user_input())
        .expect("createUser failed");
    context.as_superadmin();
    let verification_token = context
        .request(get_existing_reset_token::ExistingResetTokenInput {
            user_id: user.raw_id,
            token_type: get_existing_reset_token::TokenTypeInput::EMAIL_VERIFY,
        })
        .expect("get_existing_reset_token failed")
        .token;
    context.clear_bearer();
    //when
    let verification = context
        .request(verify_email::VerifyEmailApply {
            token: verification_token.clone(),
            ..verify_email::default_verify_email_input()
        })
        .expect("verify_email failed");
    //then
    assert_eq!(verification.success, true);
    assert_eq!(verification.email, user.email);
    context.set_bearer(verification.token);

    let me = context
        .request(get_me::GetMeInput {})
        .expect("get_me failed")
        .expect("get_me returned nothing");
    assert_eq!(me.emarsys_id.is_some(), true);

    //only verified user can create store
    let store = context
        .request(create_store::CreateStoreInput {
            user_id: user.raw_id,
            ..create_store::default_create_store_input()
        })
        .expect("create_store failed");
    assert_eq!(store.user_id, user.raw_id);
}

#[test]
pub fn deactivate_user() {
    //setup
    let mut context = TestContext::new();
    //given
    let user = context
        .request(create_user::default_create_user_input())
        .expect("createUser failed");
    //when
    context.as_superadmin();
    let deactivated_user = context
        .request(deactivate_user::DeactivateUserInput {
            id: user.id,
            ..deactivate_user::default_deactivate_user_input()
        })
        .expect("deactivate_user failed");
    //then
    assert_eq!(deactivated_user.is_active, false);
}

#[test]
pub fn update_user() {
    //setup
    let mut context = TestContext::new();
    //given
    let user = context
        .request(create_user::default_create_user_input())
        .expect("createUser failed");
    context.verify_user_email(&user.email).unwrap();
    let token: String = context
        .request(get_jwt_by_email::default_create_jwt_email_input())
        .expect("get_jwt_by_email faile")
        .token;
    context.set_bearer(token);
    //when
    let updated_user = context
        .request(update_user::UpdateUserInput {
            id: user.id,
            is_active: Some(false),
            ..update_user::default_update_user_input()
        })
        .expect("update_user failed");
    //then
    let expected_values = update_user::default_update_user_input();
    assert_eq!(updated_user.is_active, false);
    assert_eq!(
        updated_user.phone.expect("updated_user.phone is none"),
        expected_values.phone.unwrap()
    );
    assert_eq!(
        updated_user
            .first_name
            .expect("updated_user.first_name is none"),
        expected_values.first_name.unwrap()
    );
    assert_eq!(
        updated_user
            .last_name
            .expect("updated_user.last_name is none"),
        expected_values.last_name.unwrap()
    );
    assert_eq!(
        updated_user
            .middle_name
            .expect("updated_user.middle_name is none"),
        expected_values.middle_name.unwrap()
    );
    assert_eq!(
        updated_user
            .birthdate
            .expect("updated_user.birthdate is none"),
        expected_values.birthdate.unwrap()
    );
    assert_eq!(
        updated_user.avatar.expect("updated_user.avatar is none"),
        expected_values.avatar.unwrap()
    );
}

#[test]
pub fn deactivate_base_product() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    //when
    let _ = context
        .request(deactivate_base_product::DeactivateBaseProductInput {
            id: base_product.id,
            ..deactivate_base_product::default_deactivate_base_product_input()
        })
        .expect("deactivate_base_product failed");
    //then
    let deactivated_base_product = context
        .get_base_product(base_product.raw_id)
        .unwrap()
        .base_product;
    assert!(deactivated_base_product.is_none());
}

#[test]
pub fn update_base_product() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    //when
    let updated_base_product = context
        .request(update_base_product::UpdateBaseProductInput {
            id: base_product.id,
            length_cm: Some(20),
            width_cm: Some(30),
            height_cm: Some(40),
            weight_g: Some(2000),
            ..update_base_product::default_update_base_product_input()
        })
        .expect("update_base_product failed");
    //then
    let updated_base_product = context
        .get_base_product(updated_base_product.raw_id)
        .unwrap()
        .base_product
        .unwrap();
    let expected_values = update_base_product::default_update_base_product_input();
    assert!((updated_base_product.rating - expected_values.rating.unwrap()).abs() < 0.001);
    assert_eq!(updated_base_product.slug, expected_values.slug.unwrap());
    assert_eq!(updated_base_product.length_cm, Some(20));
    assert_eq!(updated_base_product.width_cm, Some(30));
    assert_eq!(updated_base_product.height_cm, Some(40));
    assert_eq!(updated_base_product.weight_g, Some(2000));
    assert_eq!(
        updated_base_product.name[0].text,
        expected_values.name.unwrap()[0].text
    );
    assert_eq!(
        updated_base_product.short_description[0].text,
        expected_values.short_description.unwrap()[0].text
    );
    assert_eq!(
        updated_base_product
            .long_description
            .expect("updated_base_product.long_description is none")[0]
            .text,
        expected_values.long_description.unwrap()[0].text
    );
    assert_eq!(
        updated_base_product
            .seo_title
            .expect("updated_base_product.seo_title is none")[0]
            .text,
        expected_values.seo_title.unwrap()[0].text
    );
    assert_eq!(
        updated_base_product
            .seo_description
            .expect("updated_base_product.seo_description is none")[0]
            .text,
        expected_values.seo_description.unwrap()[0].text
    );
}

#[test]
#[ignore]
pub fn update_base_product_does_not_update_rating() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    let initial_rating = base_product.rating;
    //when
    let updated_base_product = context
        .request(update_base_product::UpdateBaseProductInput {
            id: base_product.id,
            ..update_base_product::default_update_base_product_input()
        })
        .expect("update_base_product failed");
    //then
    let updated_base_product = context
        .get_base_product(updated_base_product.raw_id)
        .unwrap()
        .base_product
        .unwrap();
    assert!((updated_base_product.rating - initial_rating).abs() < 0.001);
}

#[test]
pub fn create_base_product_with_variants() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, category) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    //when
    let base_product = context.request(create_base_product_with_variants::NewBaseProductWithVariantsInput{
        store_id: store.raw_id,
        category_id: category.raw_id,
        ..create_base_product_with_variants::default_create_base_product_with_variants_input()
    }).expect("create_base_product_with_variants failed");
    //then
    let base_product = context
        .get_base_product(base_product.raw_id)
        .unwrap()
        .base_product
        .unwrap();
    assert_eq!(
        base_product
            .products
            .as_ref()
            .expect("base_product.products is none")
            .edges
            .len(),
        1
    );
    let variant = base_product.products.unwrap().edges.pop().unwrap().node;
    assert_eq!(variant.discount, Some(0.3));
    assert_eq!(variant.photo_main, Some("photo".to_string()));
    assert_eq!(variant.vendor_code, "vendor_code".to_string());
    assert_eq!(variant.cashback, Some(0.1));
    assert_eq!(variant.price, 100.0);
    assert_eq!(variant.pre_order, false);
    assert_eq!(variant.pre_order_days, 100);
    assert_eq!(
        variant
            .additional_photos
            .as_ref()
            .expect("variant.additional_photos is none")
            .len(),
        2
    );
    assert_eq!(
        variant
            .additional_photos
            .as_ref()
            .expect("variant.additional_photos is none")[0],
        "additional_photo_1".to_string()
    );
    assert_eq!(
        variant
            .additional_photos
            .as_ref()
            .expect("variant.additional_photos is none")[1],
        "additional_photo_2".to_string()
    );
}

#[test]
pub fn update_store() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    //when
    let updated_store = context
        .request(update_store::UpdateStoreInput {
            id: store.id,
            ..update_store::default_update_store_input()
        })
        .expect("update_store failed");
    //then
    let updated_store = context
        .get_store(updated_store.raw_id)
        .unwrap()
        .store
        .unwrap();
    let expected_values = update_store::default_update_store_input();
    verify_update_store_values(updated_store, expected_values);
}

#[test]
#[ignore]
pub fn update_store_does_not_update_rating() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    let initial_rating = store.rating;
    //when
    let updated_store = context
        .request(update_store::UpdateStoreInput {
            id: store.id,
            ..update_store::default_update_store_input()
        })
        .expect("update_store failed");
    //then
    let updated_store = context
        .get_store(updated_store.raw_id)
        .unwrap()
        .store
        .unwrap();
    assert!((updated_store.rating - initial_rating).abs() < 0.001);
}

#[test]
pub fn delete_attribute() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    //when
    let _ = context
        .request(delete_attribute::DeleteAttributeInput {
            id: attribute.id,
            ..delete_attribute::default_delete_attribute_input()
        })
        .expect("delete_attribute failed");
    //then
    let all_attribute = context.get_attributes().unwrap().attributes.unwrap();
    assert!(all_attribute.is_empty());
}

#[test]
pub fn update_attribute() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    //when
    let updated_attribute = context
        .request(update_attribute::UpdateAttributeInput {
            id: attribute.id,
            ..update_attribute::default_update_attribute_input()
        })
        .expect("update_attribute failed");
    //then
    assert_eq!(updated_attribute.name[0].text, "Update category");
}

#[test]
pub fn delete_attribute_from_category() {
    //setup
    let mut context = TestContext::new();
    //given
    context.as_superadmin();
    let category = context
        .request(create_category::default_create_category_input())
        .expect("create_category failed");
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    let _ = context
        .request(add_attribute_to_category::AddAttributeToCategoryInput {
            cat_id: category.raw_id,
            attr_id: attribute.raw_id,
            ..add_attribute_to_category::default_add_attribute_to_category_input()
        })
        .expect("add_attribute_to_category failed");
    //when
    let _ = context
        .request(
            delete_attribute_from_category::DeleteAttributeFromCategory {
                cat_id: category.raw_id,
                attr_id: attribute.raw_id,
                ..delete_attribute_from_category::default_delete_attribute_from_category_input()
            },
        )
        .expect("delete_attribute_from_category failed");
    //then
    let changed_category_attributes = context
        .get_categories()
        .unwrap()
        .categories
        .unwrap()
        .children
        .into_iter()
        .filter(|cat| cat.id == category.id)
        .next()
        .unwrap()
        .get_attributes;
    assert!(changed_category_attributes.is_empty());
}

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

#[test]
pub fn add_attribute_to_category() {
    //setup
    let mut context = TestContext::new();
    //given
    context.as_superadmin();
    let category = context
        .request(create_category::default_create_category_input())
        .expect("create_category failed");
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    //when
    let _ = context
        .request(add_attribute_to_category::AddAttributeToCategoryInput {
            cat_id: category.raw_id,
            attr_id: attribute.raw_id,
            ..add_attribute_to_category::default_add_attribute_to_category_input()
        })
        .expect("add_attribute_to_category failed");
    //then
    let changed_category_attributes = context
        .get_categories()
        .unwrap()
        .categories
        .unwrap()
        .children
        .into_iter()
        .filter(|cat| cat.id == category.id)
        .next()
        .unwrap()
        .get_attributes;
    assert_eq!(changed_category_attributes.len(), 1);
    assert!(changed_category_attributes
        .iter()
        .filter(|attr| attr.id == attribute.id)
        .next()
        .is_some());
}

#[test]
pub fn delete_category() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let category = context
        .request(create_category::default_create_category_input())
        .expect("create_category failed");
    //when
    let _ = context
        .request(delete_category::DeleteCategoryInput {
            cat_id: category.raw_id,
            ..delete_category::default_delete_category_input()
        })
        .expect("delete_category failed");
    //then
    let existing_categories = context
        .get_categories()
        .unwrap()
        .categories
        .unwrap()
        .children;
    assert!(existing_categories.is_empty());
}

#[test]
pub fn update_category() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let category = context
        .request(create_category::default_create_category_input())
        .expect("create_category failed");
    //when
    let updated_category = context
        .request(update_category::UpdateCategoryInput {
            id: category.id,
            ..update_category::default_update_category_input()
        })
        .expect("update_category failed");
    //then
    let expected_values = update_category::default_update_category_input();
    assert_eq!(updated_category.slug, expected_values.slug.unwrap());
    assert_eq!(
        updated_category.meta_field.unwrap(),
        expected_values.meta_field.unwrap()
    );
    assert_eq!(
        updated_category.parent_id.unwrap(),
        expected_values.parent_id.unwrap()
    );
    assert_eq!(updated_category.level, expected_values.level.unwrap());
}

#[test]
pub fn create_base_product() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, category) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    let new_base_product = create_base_product::CreateBaseProductInput {
        store_id: store.raw_id,
        category_id: category.raw_id,
        ..create_base_product::default_create_base_product_input()
    };
    //when
    let base_product = context
        .request(new_base_product)
        .expect("create_base_product failed");
    //then
    assert_eq!(
        base_product.slug,
        create_base_product::default_create_base_product_input()
            .slug
            .unwrap()
    );
}

#[test]
pub fn delete_attribute_value() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    let new_value = context
        .request(create_attribute_value::CreateAttributeValueInput {
            raw_attribute_id: attribute.raw_id,
            ..create_attribute_value::default_create_attribute_value_input()
        })
        .expect("create_attribute_value failed");
    //when
    let _ = context
        .request(delete_attribute_value::DeleteAttributeValueInput {
            raw_id: new_value.raw_id,
            ..delete_attribute_value::default_delete_attribute_value_input()
        })
        .expect("delete_attribute_value failed");
    //then
    let changed_attribute = context
        .get_attributes()
        .unwrap()
        .attributes
        .into_iter()
        .flatten()
        .filter(|a| a.raw_id == attribute.raw_id)
        .next()
        .unwrap();
    assert!(changed_attribute.values.unwrap().is_empty());
}

#[test]
pub fn update_attribute_value() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    let new_value = context
        .request(create_attribute_value::CreateAttributeValueInput {
            raw_attribute_id: attribute.raw_id,
            ..create_attribute_value::default_create_attribute_value_input()
        })
        .expect("create_attribute_value failed");
    //when
    let updated = context
        .request(update_attribute_value::UpdateAttributeValueInput {
            raw_id: new_value.raw_id,
            raw_attribute_id: attribute.raw_id,
            ..update_attribute_value::default_create_attribute_value_input()
        })
        .expect("update_attribute_value failed");
    //then
    assert_eq!(
        Some(updated.code),
        update_attribute_value::default_create_attribute_value_input().code
    );
}

#[test]
pub fn add_values_to_attribute() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    //when
    let new_value = context
        .request(create_attribute_value::CreateAttributeValueInput {
            raw_attribute_id: attribute.raw_id,
            ..create_attribute_value::default_create_attribute_value_input()
        })
        .expect("create_attribute_value failed");
    //then
    assert_eq!(new_value.attr_raw_id, attribute.raw_id);
    assert_eq!(new_value.attribute.unwrap().raw_id, attribute.raw_id);
}

#[test]
pub fn create_attribute_with_values() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //then
    let _attribute = context
        .request(create_attribute::CreateAttributeInput {
            values: Some(vec![
                create_attribute::CreateAttributeValueWithAttributeInput {
                    code: "attribute_code".to_string(),
                    translations: Some(vec![create_attribute::TranslationInput {
                        lang: create_attribute::Language::EN,
                        text: "attribute code".to_string(),
                    }]),
                },
            ]),
            ..create_attribute::default_create_attribute_input()
        })
        .expect("create_attribute failed");
}

#[test]
pub fn create_attribute() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //then
    let _attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
}

#[test]
pub fn create_subcategories() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let category_level_1 = context
        .request(create_category::default_create_category_input())
        .expect("create_category failed");
    let category_level_2 = context
        .request(create_category::CreateCategoryInput {
            parent_id: category_level_1.raw_id,
            slug: Some("category-slug-1".to_string()),
            ..create_category::default_create_category_input()
        })
        .expect("create_category failed");
    //when
    let category_level_3 = context
        .request(create_category::CreateCategoryInput {
            parent_id: category_level_2.raw_id,
            slug: Some("category-slug-2".to_string()),
            ..create_category::default_create_category_input()
        })
        .expect("create_category failed");
    //then
    assert_eq!(category_level_3.level, 3);
}

#[test]
pub fn create_category() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let new_category = create_category::default_create_category_input();
    //when
    let category = context
        .request(new_category)
        .expect("create_category failed");
    //then
    assert_eq!(
        Some(category.slug),
        create_category::default_create_category_input().slug
    );
    let existing_categories = context
        .get_categories()
        .unwrap()
        .categories
        .unwrap()
        .children;
    assert_eq!(existing_categories.len(), 1);
}

#[test]
pub fn create_store() {
    //setup
    let mut context = TestContext::new();
    //given
    let user = context
        .request(create_user::default_create_user_input())
        .expect("createUser failed");
    context.verify_user_email(&user.email).unwrap();
    let token: String = context
        .request(get_jwt_by_email::default_create_jwt_email_input())
        .expect("get_jwt_by_email failed")
        .token;
    context.set_bearer(token);
    //when
    let store = context
        .request(create_store::CreateStoreInput {
            user_id: user.raw_id,
            ..create_store::default_create_store_input()
        })
        .expect("create_store failed");
    //then
    assert_eq!(store.user_id, user.raw_id);
}

#[test]
pub fn create_user() {
    //setup
    let context = TestContext::new();
    //given
    let new_user = create_user::default_create_user_input();
    //when
    let user = context.request(new_user).expect("createUser failed");
    //then
    assert_eq!(user.email, create_user::default_create_user_input().email);
}

#[test]
pub fn delete_user() {
    //setup
    let mut context = TestContext::new();
    //given
    let new_user = create_user::default_create_user_input();
    let user = context.request(new_user).expect("createUser failed");
    //when
    context.as_superadmin();
    let delete_result = context.delete_user(user.raw_id);
    //then
    assert!(delete_result.is_ok())
}

#[test]
pub fn create_user_with_additional_data() {
    //setup
    let context = TestContext::new();
    //given
    let referal = context
        .request(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..create_user::default_create_user_input()
        })
        .expect("createUser failed");

    let new_user = create_user::CreateUserInput {
        additional_data: Some(create_user::NewUserAdditionalDataInput {
            country: Some("MM".to_string()),
            referal: Some(referal.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks: Some(vec![create_user::UtmMarkInput {
                key: "source".to_string(),
                value: "word_of_mouth".to_string(),
            }]),
        }),
        ..create_user::default_create_user_input()
    };
    //when
    let user = context.request(new_user).expect("createUser failed");
    //then
    assert_eq!(user.email, create_user::default_create_user_input().email);
    assert_eq!(user.referal.expect("user.referal is none"), referal.raw_id);
    assert_eq!(
        user.country.expect("user.country is none").alpha3,
        "MMR".to_string()
    );
    assert_eq!(
        user.referer.expect("user.referer is none"),
        "localhost".to_string()
    );
    assert_eq!(
        &user.utm_marks.as_ref().expect("user.utm_marks is none")[0].key,
        "source"
    );
    assert_eq!(
        &user.utm_marks.as_ref().expect("user.utm_marks is none")[0].value,
        "word_of_mouth"
    );
}

#[test]
pub fn create_user_via_facebook() {
    //setup
    let mut context = TestContext::new();
    //given
    let facebook_jwt = get_jwt_by_provider::facebook_create_jwt_provider_input();
    //when
    let token = context
        .request(facebook_jwt)
        .expect("get_jwt_by_provider facebook failed")
        .token;
    //then
    context.set_bearer(token);
    let me = context
        .request(get_me::GetMeInput {})
        .expect("get_me failed")
        .expect("get_me returned nothing");
    assert_eq!(me.email, create_user::default_create_user_input().email);
}

#[test]
pub fn create_user_via_google() {
    //setup
    let mut context = TestContext::new();
    //given
    let google_jwt = get_jwt_by_provider::google_create_jwt_provider_input();
    //when
    let token = context
        .request(google_jwt)
        .expect("get_jwt_by_provider google failed")
        .token;
    //then
    context.set_bearer(token);
    let me = context
        .request(get_me::GetMeInput {})
        .expect("get_me failed")
        .expect("get_me returned nothing");
    assert_eq!(me.email, create_user::default_create_user_input().email);
}

#[test]
pub fn create_user_via_facebook_with_additional_data() {
    //setup
    let mut context = TestContext::new();
    //given
    let referal = context
        .request(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..create_user::default_create_user_input()
        })
        .expect("createUser failed");

    let facebook_jwt = get_jwt_by_provider::CreateJWTProviderInput {
        additional_data: Some(get_jwt_by_provider::NewUserAdditionalDataInput {
            referal: Some(referal.raw_id),
            ..get_jwt_by_provider::new_user_additional_data_input()
        }),
        ..get_jwt_by_provider::facebook_create_jwt_provider_input()
    };
    //when
    let token = context
        .request(facebook_jwt)
        .expect("get_jwt_by_provider facebook failed")
        .token;
    //then
    context.set_bearer(token);
    let user = context
        .request(get_me::GetMeInput {})
        .expect("get_me failed")
        .expect("get_me returned nothing");
    verify_additional_data(user, referal.raw_id);
}

#[test]
pub fn create_user_via_google_with_additional_data() {
    //setup
    let mut context = TestContext::new();
    //given
    let referal = context
        .request(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..create_user::default_create_user_input()
        })
        .expect("createUser failed");

    let google_jwt = get_jwt_by_provider::CreateJWTProviderInput {
        additional_data: Some(get_jwt_by_provider::NewUserAdditionalDataInput {
            referal: Some(referal.raw_id),
            ..get_jwt_by_provider::new_user_additional_data_input()
        }),
        ..get_jwt_by_provider::google_create_jwt_provider_input()
    };
    //when
    let token = context
        .request(google_jwt)
        .expect("get_jwt_by_provider google failed")
        .token;
    //then
    context.set_bearer(token);
    let user = context
        .request(get_me::GetMeInput {})
        .expect("get_me failed")
        .expect("get_me returned nothing");
    verify_additional_data(user, referal.raw_id);
}

fn verify_additional_data(user: get_me::RustGetMeMe, referal_id: i64) {
    assert_eq!(user.email, create_user::default_create_user_input().email);
    assert_eq!(user.referal.expect("user.referal is none"), referal_id);
    assert_eq!(
        user.country.expect("user.country is none").alpha3,
        "MMR".to_string()
    );
    assert_eq!(
        user.referer.expect("user.referer is none"),
        "localhost".to_string()
    );
    assert_eq!(
        &user.utm_marks.as_ref().expect("user.utm_marks is none")[0].key,
        "source"
    );
    assert_eq!(
        &user.utm_marks.as_ref().expect("user.utm_marks is none")[0].value,
        "word_of_mouth"
    );
}

#[test]
pub fn delete_store() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    //when
    context.as_superadmin();
    let delete_result = context.delete_store(store.raw_id);
    //then
    assert!(delete_result.is_ok())
}

#[test]
pub fn update_store_in_status_draft() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _) =
        set_up_store(&mut context).expect("Cannot get data from set_up_store");
    context.set_bearer(token);

    //when
    let update_result = context.request(update_store::UpdateStoreInput {
        id: store.id.clone(),
        email: Some("example@example.com".to_string()),
        ..update_store::default_update_store_input()
    });

    let update_store = update_result.expect("Cannot get update store");

    //then
    assert_eq!(update_store.email, Some("example@example.com".to_string()));
    assert_eq!(update_store.id, store.id);
    assert_eq!(update_store.status, update_store::Status::DRAFT)
}

#[test]
pub fn update_base_product_in_status_draft() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    //when
    let update_base_product_payload = update_base_product::UpdateBaseProductInput {
        id: base_product.id.clone(),
        slug: Some(format!("{}-{}", base_product.slug, "plus")),
        ..update_base_product::default_update_base_product_input()
    };

    let update_base_product = context
        .request(update_base_product_payload)
        .expect("Cannot get update base_product");

    //then
    assert_eq!(update_base_product.id, base_product.id);
    assert_eq!(
        update_base_product.slug,
        format!("{}-{}", base_product.slug, "plus")
    );
    assert_eq!(
        update_base_product.status,
        update_base_product::Status::DRAFT
    );
}

#[test]
pub fn create_product_without_attributes() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    //when
    let product_payload = create_product::CreateProductWithAttributesInput {
        product: create_product::NewProduct {
            base_product_id: Some(base_product.raw_id),
            ..create_product::default_new_product()
        },
        ..create_product::default_create_product_input()
    };

    let new_product = context
        .request(product_payload)
        .expect("Cannot get data from create_product");

    //then
    assert_eq!(base_product.status, create_base_product::Status::DRAFT);
    assert_eq!(new_product.base_product_id, base_product.raw_id);
}

#[test]
pub fn deactivate_product() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    let product_payload = create_product::CreateProductWithAttributesInput {
        product: create_product::NewProduct {
            base_product_id: Some(base_product.raw_id),
            ..create_product::default_new_product()
        },
        ..create_product::default_create_product_input()
    };

    let new_product = context
        .request(product_payload)
        .expect("Cannot get data from create_product");

    let deactivate_product_payload = deactivate_product::DeactivateProductInput {
        id: new_product.id.clone(),
        ..deactivate_product::default_deactivate_product_input()
    };

    //when
    let product = context
        .request(deactivate_product_payload)
        .expect("Cannot get data from deactivate_product");
    //then
    assert_eq!(new_product.id, product.id);
    assert_eq!(product.is_active, false);
}

#[test]
pub fn create_product_without_base_product() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    //when
    let product_payload = create_product::CreateProductWithAttributesInput {
        product: create_product::default_new_product(),
        ..create_product::default_create_product_input()
    };

    let new_product = context.request(product_payload);

    //then
    assert_eq!(base_product.status, create_base_product::Status::DRAFT);
    assert!(new_product.is_err());
}

#[test]
#[ignore]
pub fn update_product_without_attributes() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    let product_payload = create_product::CreateProductWithAttributesInput {
        product: create_product::NewProduct {
            base_product_id: Some(base_product.raw_id),
            ..create_product::default_new_product()
        },
        ..create_product::default_create_product_input()
    };

    let new_product = context
        .request(product_payload)
        .expect("Cannot get data from create_product");

    //when
    let update_product_payload = update_product::UpdateProductWithAttributesInput {
        id: new_product.id.clone(),
        product: Some(update_product::UpdateProduct {
            price: Some(15f64),
            pre_order: Some(true),
            pre_order_days: Some(15),
            ..update_product::default_update_product_input()
        }),
        ..update_product::default_update_product_with_attributes_input()
    };

    let update_product = context
        .request(update_product_payload)
        .expect("Cannot get update product");
    //then
    assert_eq!(base_product.status, create_base_product::Status::DRAFT);
    assert_eq!(update_product.id, new_product.id);
    assert_eq!(update_product.pre_order, true);
    assert_eq!(update_product.pre_order_days, 15);
}

#[test]
pub fn create_delivery_company() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let company_payload = create_delivery_company::NewCompanyInput {
        name: "Test company".to_string(),
        label: "TEST".to_string(),
        description: Some("Test description".to_string()),
        deliveries_from: vec!["RUS".to_string()],
        logo: "test loge URL".to_string(),
        ..create_delivery_company::default_create_company_input()
    };
    //when
    let create_company = context
        .request(company_payload.clone())
        .expect("Cannot get data from create_delivery_company");

    let rus_country = create_company
        .deliveries_from
        .iter()
        .flat_map(|root| {
            root.children
                .iter()
                .flat_map(|region| region.children.iter())
        })
        .find(|c| c.alpha3 == "RUS".to_string());
    //then
    assert_eq!(
        rus_country.map(|c| c.alpha3.clone()),
        Some("RUS".to_string())
    );
    assert_eq!(create_company.label, company_payload.label);
    assert_eq!(create_company.name, company_payload.name);
    assert_eq!(create_company.description, company_payload.description);
    assert_eq!(create_company.logo, company_payload.logo);
}

#[test]
pub fn update_delivery_company() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let company_payload = create_delivery_company::default_create_company_input();
    let create_company = context
        .request(company_payload)
        .expect("Cannot get data from create_delivery_company");
    //when
    let update_company_payload = update_delivery_company::UpdateCompanyInput {
        id: create_company.id.clone(),
        name: Some("Test company plus update".to_string()),
        ..update_delivery_company::default_update_company_input()
    };
    let update_company = context
        .request(update_company_payload)
        .expect("Cannot get data from update_delivery_company");
    //then
    assert_eq!(update_company.id, create_company.id);
    assert_eq!(update_company.name, "Test company plus update".to_string());
}

#[test]
pub fn delete_delivery_company() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let company_payload = create_delivery_company::default_create_company_input();
    let create_company = context
        .request(company_payload)
        .expect("Cannot get data from create_delivery_company");
    //when
    let delete_company = context
        .delete_delivery_company(create_company.raw_id)
        .expect("Cannot get data from delete_delivery_company")
        .delete_company;
    //then
    assert_eq!(create_company.raw_id, delete_company.raw_id);
}

#[test]
pub fn get_categories_with_products() {
    //setup
    let mut context = TestContext::new();

    //given
    let (_user, _token, _store, category, _base_product) =
        set_up_base_product(&mut context).expect("set_up_base_product failed");
    context.as_superadmin();
    let category_level_1 = context
        .request(create_category::CreateCategoryInput {
            parent_id: 0,
            slug: Some("category-level-1".to_string()),
            ..create_category::default_create_category_input()
        })
        .expect("Cannot create category category_level_1");
    let category_level_2 = context
        .request(create_category::CreateCategoryInput {
            parent_id: category_level_1.raw_id,
            slug: Some("category-level-2".to_string()),
            ..create_category::default_create_category_input()
        })
        .expect("Cannot create category category_level_2");
    let category_level_3 = context
        .request(create_category::CreateCategoryInput {
            parent_id: category_level_2.raw_id,
            slug: Some("category-level-3".to_string()),
            ..create_category::default_create_category_input()
        })
        .expect("Cannot create category category_level_3");

    //when
    let mut categories = context
        .get_categories_with_products()
        .unwrap()
        .categories_with_products
        .into_iter()
        .flat_map(|root| {
            root.children.into_iter().flat_map(|category1| {
                category1
                    .children
                    .into_iter()
                    .flat_map(|category2| category2.children.into_iter())
            })
        });
    //then
    let exists_category = categories.find(|value| value.id == category.id);
    println!("exists_category: {:#?}", exists_category);

    assert_eq!(exists_category.map(|c| c.id), Some(category.id));
    assert!(categories
        .find(|value| value.id == category_level_3.id)
        .is_none());
}

fn verify_update_store_values(
    updated_store: get_store::RustGetStoreStore,
    expected_values: update_store::UpdateStoreInput,
) {
    assert_eq!(
        updated_store.name[0].text,
        expected_values.name.unwrap()[0].text
    );
    assert_eq!(
        updated_store.short_description[0].text,
        expected_values.short_description.unwrap()[0].text
    );
    assert_eq!(
        updated_store
            .long_description
            .expect("update_store.long_description is none")[0]
            .text,
        expected_values.long_description.unwrap()[0].text
    );
    assert_eq!(updated_store.slug, expected_values.slug.unwrap());
    assert_eq!(
        updated_store.cover.expect("update_store.cover is none"),
        expected_values.cover.unwrap()
    );
    assert_eq!(
        updated_store.logo.expect("update_store.logo is none"),
        expected_values.logo.unwrap()
    );
    assert_eq!(
        updated_store.phone.expect("update_store.phone is none"),
        expected_values.phone.unwrap()
    );
    assert_eq!(
        updated_store.email.expect("update_store.email is none"),
        expected_values.email.unwrap()
    );
    assert_eq!(
        updated_store
            .instagram_url
            .expect("update_store.instagram_url is none"),
        expected_values.instagram_url.unwrap()
    );
    assert_eq!(
        updated_store
            .twitter_url
            .expect("update_store.twitter_url is none"),
        expected_values.twitter_url.unwrap()
    );
    assert_eq!(
        updated_store
            .facebook_url
            .expect("update_store.facebook_url is none"),
        expected_values.facebook_url.unwrap()
    );
    assert_eq!(
        updated_store.slogan.expect("update_store.slogan is none"),
        expected_values.slogan.unwrap()
    );
    assert!((updated_store.rating - expected_values.rating.unwrap()).abs() < 0.001);

    assert_eq!(
        updated_store
            .address_full
            .value
            .expect("update_store.address_full.value is none"),
        expected_values.address_full.value.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .country
            .expect("update_store.address_full.country is none"),
        expected_values.address_full.country.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .country_code
            .expect("update_store.address_full.country_code is none"),
        expected_values.address_full.country_code.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .administrative_area_level1
            .expect("update_store.address_full.administrative_area_level1 is none"),
        expected_values
            .address_full
            .administrative_area_level1
            .unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .administrative_area_level2
            .expect("update_store.address_full.administrative_area_level2 is none"),
        expected_values
            .address_full
            .administrative_area_level2
            .unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .locality
            .expect("update_store.address_full.locality is none"),
        expected_values.address_full.locality.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .political
            .expect("update_store.address_full.political is none"),
        expected_values.address_full.political.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .postal_code
            .expect("update_store.address_full.postal_code is none"),
        expected_values.address_full.postal_code.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .route
            .expect("update_store.address_full.route is none"),
        expected_values.address_full.route.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .street_number
            .expect("update_store.address_full.street_number is none"),
        expected_values.address_full.street_number.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .place_id
            .expect("update_store.address_full.place_id is none"),
        expected_values.address_full.place_id.unwrap()
    );
}

#[test]
fn create_update_delete_warehouse() {
    let mut context = TestContext::new();

    let (_user, token, store, new_warehouse) =
        set_up_warehouse(&mut context).expect("Cannot get data from create_warehouse");
    let id = new_warehouse.id;

    assert_eq!(new_warehouse.name, Some("Initial name".to_string()));
    assert_eq!(new_warehouse.store_id, store.raw_id);

    let updated_warehouse = update_warehouse(
        &mut context,
        token.clone(),
        update_warehouse::UpdateWarehouseInput {
            id: id.clone(),
            address_full: update_warehouse::AddressInput {
                country: Some("Russian Federation".to_string()),
                administrative_area_level1: Some("Moscow Region".to_string()),
                administrative_area_level2: Some("Moscow".to_string()),
                ..update_warehouse::default_address_input()
            },
            location: Some(update_warehouse::GeoPointInput {
                x: 42.0,
                y: 90.0,
                ..update_warehouse::default_geo_point_input()
            }),
            name: Some("New name".to_string()),
            ..update_warehouse::default_update_warehouse_input()
        },
    )
    .expect("Cannot get data from update_warehouse")
    .expect("Empty data from update_warehouse");

    assert_eq!(updated_warehouse.name, Some("New name".to_string()));
    assert_eq!(
        updated_warehouse.address_full.country,
        Some("Russian Federation".to_string())
    );
    assert_eq!(
        updated_warehouse.address_full.administrative_area_level1,
        Some("Moscow Region".to_string())
    );
    assert_eq!(
        updated_warehouse.address_full.administrative_area_level2,
        Some("Moscow".to_string())
    );
    let location = updated_warehouse
        .location
        .expect("Cannot get location data from update_warehouse");
    assert_eq!(location.x, 42.0);
    assert_eq!(location.y, 90.0);

    let deleted_warehouse_id = delete_warehouse(&mut context, token.clone(), id.clone())
        .expect("Cannot get data from delete_warehouse")
        .expect("Empty data from delete_warehouse");

    assert_eq!(deleted_warehouse_id, id);

    // negative cases
    let update_deleted_warehouse = update_warehouse(
        &mut context,
        token.clone(),
        update_warehouse::UpdateWarehouseInput {
            id: id.clone(),
            address_full: update_warehouse::AddressInput {
                country: Some("Russian Federation".to_string()),
                administrative_area_level1: Some("Moscow Region".to_string()),
                administrative_area_level2: Some("Moscow".to_string()),
                ..update_warehouse::default_address_input()
            },
            location: Some(update_warehouse::GeoPointInput {
                x: 42.0,
                y: 90.0,
                ..update_warehouse::default_geo_point_input()
            }),
            name: Some("New name".to_string()),
            ..update_warehouse::default_update_warehouse_input()
        },
    );
    if update_deleted_warehouse.is_ok() && update_deleted_warehouse.unwrap().is_some() {
        panic!("Should not be able to update deleted warehouse");
    }

    let deleted_twice = delete_warehouse(&mut context, token.clone(), id.clone());
    if deleted_twice.is_ok() && deleted_twice.unwrap() != None {
        panic!("Should not be able to delete the same warehouse twice");
    }
}

#[test]
fn create_update_delete_package() {
    let mut context = TestContext::new();
    let new_package = create_package(
        &mut context,
        create_package::NewPackagesInput {
            name: "Initial name".to_string(),
            deliveries_to: vec!["RUS".to_string(), "USA".to_string()],
            ..create_package::default_graphql_request_input()
        },
    )
    .expect("Cannot get data from create_package");

    let get_package = context
        .request(get_package::GetPackageInput {
            id: new_package.raw_id,
        })
        .expect("Cannot get data from get_package")
        .expect("Empty data from get_package");

    assert_eq!(get_package.name, "Initial name".to_string());
    assert_eq!(get_package.max_size, 1000);
    assert_eq!(get_package.min_size, 100);
    assert_eq!(get_package.max_weight, 3000);
    assert_eq!(get_package.min_weight, 300);

    // deliveries
    assert_eq!(get_package.deliveries_to.len(), 1);

    let xal = get_package
        .deliveries_to
        .first()
        .expect("Cannot get delivery info");
    assert_eq!(xal.level, 0);
    assert_eq!(xal.label, "All".to_string());
    assert_eq!(xal.alpha3, "XAL".to_string());
    assert_eq!(xal.children.len(), 2);

    let xeu = xal
        .children
        .iter()
        .find(|d| d.label == "Europe".to_string())
        .expect("Cannot get Europe delivery info");
    assert_eq!(xeu.level, 1);
    assert_eq!(xeu.alpha3, "XEU".to_string());
    assert_eq!(xeu.children.len(), 1);

    let xna = xal
        .children
        .iter()
        .find(|d| d.label == "North America".to_string())
        .expect("Cannot get North America delivery info");
    assert_eq!(xna.level, 1);
    assert_eq!(xna.alpha3, "XNA".to_string());
    assert_eq!(xna.children.len(), 1);

    let rus = xeu
        .children
        .iter()
        .find(|d| d.label == "Russian Federation".to_string())
        .expect("Cannot get Russian Federation delivery info");
    assert_eq!(rus.level, 2);
    assert_eq!(rus.alpha2, "RU".to_string());
    assert_eq!(rus.alpha3, "RUS".to_string());

    let usa = xna
        .children
        .iter()
        .find(|d| d.label == "United States of America".to_string())
        .expect("Cannot get United States of America delivery info");
    assert_eq!(usa.level, 2);
    assert_eq!(usa.alpha2, "US".to_string());
    assert_eq!(usa.alpha3, "USA".to_string());

    let _updated_package = update_package(
        &mut context,
        update_package::UpdatePackagesInput {
            id: new_package.id,
            name: Some("New name".to_string()),
            max_size: Some(1001),
            min_size: Some(101),
            max_weight: Some(3001),
            min_weight: Some(301),
            ..update_package::default_graphql_request_input()
        },
    )
    .expect("Cannot get data from update_package");

    let get_package = context
        .request(get_package::GetPackageInput {
            id: new_package.raw_id,
        })
        .expect("Cannot get data from get_package")
        .expect("Empty data from get_package");

    assert_eq!(get_package.name, "New name".to_string());
    assert_eq!(get_package.max_size, 1001);
    assert_eq!(get_package.min_size, 101);
    assert_eq!(get_package.max_weight, 3001);
    assert_eq!(get_package.min_weight, 301);

    delete_package(&mut context, new_package.raw_id).expect("Cannot get deleted package");

    // negative cases
    let get_package = context.request(get_package::GetPackageInput {
        id: new_package.raw_id,
    });
    if get_package.is_ok() && get_package.unwrap().is_some() {
        panic!("Should not be able to get deleted package");
    }

    if delete_package(&mut context, new_package.raw_id).is_ok() {
        panic!("Should not be able to delete the same package twice");
    }
}

#[test]
fn create_delete_company_package() {
    let mut context = TestContext::new();
    context.as_superadmin();

    let new_package = create_package(
        &mut context,
        create_package::NewPackagesInput {
            name: "Initial name".to_string(),
            deliveries_to: vec!["RUS".to_string(), "USA".to_string()],
            ..create_package::default_graphql_request_input()
        },
    )
    .expect("Cannot get data from create_package");

    let new_company = context
        .request(create_delivery_company::NewCompanyInput {
            name: "Test company".to_string(),
            label: "TEST".to_string(),
            description: Some("Test description".to_string()),
            deliveries_from: vec!["RUS".to_string()],
            logo: "test loge URL".to_string(),
            ..create_delivery_company::default_create_company_input()
        })
        .expect("Cannot get data from create_delivery_company");

    let company_package = add_package_to_company(
        &mut context,
        add_package_to_company::NewCompaniesPackagesInput {
            company_id: new_company.raw_id,
            package_id: new_package.raw_id,
            ..add_package_to_company::default_graphql_request_input()
        },
    )
    .expect("Cannot get data from add_package_to_company");

    let company = company_package
        .company
        .expect("Cannot get company data from add_package_to_company");
    let package = company_package
        .package
        .expect("Empty data from add_package_to_company");

    assert_eq!(company.label, new_company.label);
    assert_eq!(company.name, new_company.name);
    assert_eq!(package.name, new_package.name);

    let company_package = context
        .request(get_company_package::GetCompanyPackageInput {
            id: company_package.raw_id,
        })
        .expect("Cannot get data from get_company_package")
        .expect("Empty data from get_company_package");
    assert_eq!(company_package.company_id, new_company.raw_id);
    assert_eq!(company_package.package_id, new_package.raw_id);

    delete_company_package(&mut context, new_company.raw_id, new_package.raw_id)
        .expect("Cannot get data from delete_company_package");

    // negative cases
    let deleted_company_package = context.request(get_company_package::GetCompanyPackageInput {
        id: company_package.raw_id,
    });
    if deleted_company_package.is_ok() && deleted_company_package.unwrap().is_some() {
        panic!("Should not be able to get deleted company package");
    }

    if delete_company_package(&mut context, new_company.raw_id, new_package.raw_id).is_ok() {
        panic!("Should not be able to delete the same company package twice");
    }
}

#[test]
fn upsert_shipping() {
    let mut context = TestContext::new();

    let (_user, token, store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");

    context.set_bearer(token);
    let warehouse_payload = create_warehouse::CreateWarehouseInput {
        name: Some("Warehouse".to_string()),
        store_id: store.raw_id,
        address_full: create_warehouse::AddressInput {
            country: Some("Russian Federation".to_string()),
            country_code: Some("RUS".to_string()),
            ..create_warehouse::default_address_input()
        },
        ..create_warehouse::default_graphql_request_input()
    };
    let _warehouse = context
        .request(warehouse_payload)
        .expect("Cannot get data from create_warehouse");

    context.as_superadmin();
    let upsert_shipping_payload = upsert_shipping::NewShippingInput {
        store_id: store.raw_id,
        base_product_id: base_product.raw_id,
        ..upsert_shipping::default_graphql_request_input()
    };
    let _upsert_shipping = context
        .request(upsert_shipping_payload)
        .expect("Cannot get data from upsert_shipping");
}

fn add_package_to_company(
    context: &mut TestContext,
    payload: add_package_to_company::NewCompaniesPackagesInput,
) -> Result<add_package_to_company::GraphqlRequestOutput, FailureError> {
    context.as_superadmin();
    let company_package = context.request(payload)?;
    context.clear_bearer();
    Ok(company_package)
}

fn delete_company_package(
    context: &mut TestContext,
    company_id: i64,
    package_id: i64,
) -> Result<delete_company_package::GraphqlRequestOutput, FailureError> {
    context.as_superadmin();
    let company_package = context.request(delete_company_package::DeleteCompanyPackageInput {
        company_id,
        package_id,
    })?;
    context.clear_bearer();
    Ok(company_package)
}

fn create_package(
    context: &mut TestContext,
    payload: create_package::NewPackagesInput,
) -> Result<create_package::RustCreatePackageCreatePackage, FailureError> {
    context.as_superadmin();

    context.request(payload)
}

fn update_package(
    context: &mut TestContext,
    payload: update_package::UpdatePackagesInput,
) -> Result<update_package::RustUpdatePackageUpdatePackage, FailureError> {
    context.as_superadmin();

    context.request(payload)
}

fn delete_package(
    context: &mut TestContext,
    id: i64,
) -> Result<delete_package::RustDeletePackageDeletePackage, FailureError> {
    context.as_superadmin();

    context.request(delete_package::DeletePackagesInput { id })
}

fn set_up_warehouse(
    context: &mut TestContext,
) -> Result<
    (
        create_user::RustCreateUserCreateUser,
        String,
        get_store::RustGetStoreStore,
        create_warehouse::RustCreateWarehouseCreateWarehouse,
    ),
    FailureError,
> {
    let (user, token, store, _) = set_up_store(context)?;
    context.set_bearer(token.clone());
    let store = context
        .get_store(store.raw_id)?
        .store
        .expect("Cannot get data from get_store");
    let warehouse_payload = create_warehouse::CreateWarehouseInput {
        name: Some("Initial name".to_string()),
        store_id: store.raw_id,
        ..create_warehouse::default_graphql_request_input()
    };
    let warehouse = context.request(warehouse_payload)?;
    context.clear_bearer();
    Ok((user, token, store, warehouse))
}

fn update_warehouse(
    context: &mut TestContext,
    token: String,
    input: update_warehouse::UpdateWarehouseInput,
) -> Result<(Option<get_store::RustGetStoreStoreWarehouses>), FailureError> {
    context.set_bearer(token);
    let updated_warehouse = context.request(input)?;
    if updated_warehouse.is_none() {
        return Ok(None);
    }
    let updated_warehouse = updated_warehouse.unwrap();
    let store = context
        .get_store(updated_warehouse.store_id)?
        .store
        .expect("Cannot get data from get_store");
    context.clear_bearer();
    Ok(store
        .warehouses
        .into_iter()
        .find(|x| x.id == updated_warehouse.id))
}

fn delete_warehouse(
    context: &mut TestContext,
    token: String,
    id: String,
) -> Result<Option<String>, FailureError> {
    context.set_bearer(token);
    let deleted_warehouse =
        context.request(delete_warehouse::DeleteWarehouseInput { id: id.clone() })?;
    context.clear_bearer();
    Ok(deleted_warehouse.map(|w| w.id))
}

fn set_up_user(
    context: &mut TestContext,
) -> Result<(create_user::RustCreateUserCreateUser, String), FailureError> {
    let user = context.request(create_user::default_create_user_input())?;
    context.verify_user_email(&user.email)?;
    let token: String = context
        .request(get_jwt_by_email::default_create_jwt_email_input())?
        .token;
    Ok((user, token))
}

fn set_up_store(
    context: &mut TestContext,
) -> Result<
    (
        create_user::RustCreateUserCreateUser,
        String,
        create_store::RustCreateStoreCreateStore,
        create_category::RustCreateCategoryCreateCategory,
    ),
    FailureError,
> {
    let (user, token) = set_up_user(context)?;
    context.set_bearer(token.clone());
    let store = context.request(create_store::CreateStoreInput {
        user_id: user.raw_id,
        ..create_store::default_create_store_input()
    })?;
    context.as_superadmin();
    let category_level_1 = context.request(create_category::default_create_category_input())?;
    let category_level_2 = context.request(create_category::CreateCategoryInput {
        parent_id: category_level_1.raw_id,
        slug: Some("category-slug-1".to_string()),
        ..create_category::default_create_category_input()
    })?;
    let category_level_3 = context.request(create_category::CreateCategoryInput {
        parent_id: category_level_2.raw_id,
        slug: Some("category-slug-2".to_string()),
        ..create_category::default_create_category_input()
    })?;
    context.clear_bearer();
    Ok((user, token, store, category_level_3))
}

fn set_up_published_store(
    context: &mut TestContext,
) -> Result<
    (
        create_user::RustCreateUserCreateUser,
        String,
        create_store::RustCreateStoreCreateStore,
        create_category::RustCreateCategoryCreateCategory,
    ),
    FailureError,
> {
    let (user, token) = set_up_user(context)?;
    context.set_bearer(token.clone());
    //create
    let store = context.request(create_store::CreateStoreInput {
        user_id: user.raw_id,
        ..create_store::default_create_store_input()
    })?;

    //publish
    context.set_bearer(token.clone());
    let _ = context.request(send_store_to_moderation::SendStoreToModerationInput {
        raw_id: store.raw_id,
    })?;
    context.as_superadmin();
    let _ = context.request(set_moderation_status_store::StoreModerateInput {
        id: store.id.clone(),
        status: set_moderation_status_store::Status::PUBLISHED,
    })?;
    //create categories
    context.as_superadmin();
    let category_level_1 = context.request(create_category::default_create_category_input())?;
    let category_level_2 = context.request(create_category::CreateCategoryInput {
        parent_id: category_level_1.raw_id,
        slug: Some("category-slug-1".to_string()),
        ..create_category::default_create_category_input()
    })?;
    let category_level_3 = context.request(create_category::CreateCategoryInput {
        parent_id: category_level_2.raw_id,
        slug: Some("category-slug-2".to_string()),
        ..create_category::default_create_category_input()
    })?;
    context.clear_bearer();
    Ok((user, token, store, category_level_3))
}

fn set_up_base_product(
    context: &mut TestContext,
) -> Result<
    (
        create_user::RustCreateUserCreateUser,
        String,
        create_store::RustCreateStoreCreateStore,
        create_category::RustCreateCategoryCreateCategory,
        create_base_product::RustCreateBaseProductCreateBaseProduct,
    ),
    FailureError,
> {
    let (user, token, store, category) =
        set_up_store(context).expect("Cannot get data from set_up_store");
    context.set_bearer(token.clone());

    let new_base_product = create_base_product::CreateBaseProductInput {
        store_id: store.raw_id,
        category_id: category.raw_id,
        ..create_base_product::default_create_base_product_input()
    };
    let base_product = context.request(new_base_product)?;
    context.clear_bearer();

    Ok((user, token, store, category, base_product))
}

fn set_up_published_base_product(
    context: &mut TestContext,
) -> Result<
    (
        create_user::RustCreateUserCreateUser,
        String,
        create_store::RustCreateStoreCreateStore,
        create_category::RustCreateCategoryCreateCategory,
        create_base_product::RustCreateBaseProductCreateBaseProduct,
    ),
    FailureError,
> {
    let (user, token, store, category) =
        set_up_published_store(context).expect("Cannot get data from set_up_published_store");
    context.set_bearer(token.clone());

    let new_base_product = create_base_product::CreateBaseProductInput {
        store_id: store.raw_id,
        category_id: category.raw_id,
        ..create_base_product::default_create_base_product_input()
    };
    let base_product = context.request(new_base_product)?;

    context.set_bearer(token.clone());
    let _ = context
        .request(
            send_base_product_to_moderation::SendBaseProductToModerationInput {
                raw_id: base_product.raw_id,
            },
        )
        .expect("send_base_product_to_moderation failed to send to moderation");
    context.as_superadmin();
    let _ = context
        .request(
            set_moderation_status_base_product::BaseProductModerateInput {
                id: base_product.id.clone(),
                status: set_moderation_status_base_product::Status::PUBLISHED,
            },
        )
        .expect("set_moderation_status_base_product failed");
    context.clear_bearer();

    Ok((user, token, store, category, base_product))
}

fn set_up_published_product(
    context: &mut TestContext,
) -> Result<
    (
        create_user::RustCreateUserCreateUser,
        String,
        create_store::RustCreateStoreCreateStore,
        create_category::RustCreateCategoryCreateCategory,
        create_base_product::RustCreateBaseProductCreateBaseProduct,
        create_product::RustCreateProductCreateProduct,
    ),
    FailureError,
> {
    let (user, token, store, category, base_product) =
        set_up_published_base_product(context).expect("set_up_published_base_product failed");

    context.set_bearer(token.clone());
    let product_payload = create_product::CreateProductWithAttributesInput {
        product: create_product::NewProduct {
            base_product_id: Some(base_product.raw_id),
            ..create_product::default_new_product()
        },
        ..create_product::default_create_product_input()
    };

    let new_product = context
        .request(product_payload)
        .expect("Cannot get data from create_product");

    context.clear_bearer();
    Ok((user, token, store, category, base_product, new_product))
}
