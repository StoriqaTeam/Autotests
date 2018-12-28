use functional_tests::context::TestContext;
use functional_tests::query::*;

use basic_tests::set_up_published_product;

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
    println!("\n{:#?}\n", user_cart);
    assert_eq!(
        user_cart
            .get_product(created_product.raw_id)
            .expect("product not found in cart")
            .quantity,
        13
    );
}
