use functional_tests::context::TestContext;
use functional_tests::query::*;

use basic_tests::set_up_published_product;

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
