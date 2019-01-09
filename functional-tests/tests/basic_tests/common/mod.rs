use failure::Error as FailureError;
use functional_tests::context::TestContext;
use functional_tests::query::*;

pub mod models;
pub use self::models::*;

pub fn set_up_user(
    context: &mut TestContext,
) -> Result<(create_user::RustCreateUserCreateUser, String), FailureError> {
    let user = context.request(create_user::default_create_user_input())?;
    context.verify_user_email(&user.email)?;
    let token: String = context
        .request(get_jwt_by_email::default_create_jwt_email_input())?
        .token;
    Ok((user, token))
}

pub fn create_user_with_products_in_carts(
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

pub fn create_store_with_several_products(
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

pub fn create_base_broduct_with_two_products(
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

pub fn set_up_store(
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

pub fn set_up_base_product(
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

pub fn set_up_base_product_with_attributes(
    context: &mut TestContext,
) -> Result<
    (
        create_user::RustCreateUserCreateUser,
        String,
        create_store::RustCreateStoreCreateStore,
        create_category::RustCreateCategoryCreateCategory,
        create_base_product::RustCreateBaseProductCreateBaseProduct,
        create_attribute::RustCreateAttributeCreateAttribute,
        create_custom_attribute::RustCreateCustomAttributeCreateCustomAttribute,
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

    context.as_superadmin();
    let new_attribute = create_attribute::CreateAttributeInput {
        name: vec![create_attribute::TranslationInput {
            lang: create_attribute::Language::EN,
            text: "Color".to_string(),
        }],
        values: Some(vec![
            create_attribute::create_attribute_value("RED", "Red", "Красный"),
            create_attribute::create_attribute_value("BLUE", "Blue", "Синий"),
            create_attribute::create_attribute_value("GREEN", "Green", "Зелёный"),
        ]),
        ..create_attribute::default_create_attribute_input()
    };
    let attribute = context
        .request(new_attribute)
        .expect("Cannot get data from create_attribute");

    let new_custom_attribute = create_custom_attribute::NewCustomAttributeInput {
        attribute_id: attribute.raw_id,
        base_product_id: base_product.raw_id,
        ..create_custom_attribute::default_create_custom_attribute_input()
    };
    let custom_attribute = context
        .request(new_custom_attribute)
        .expect("Cannot get data from create_custom_attribute");
    context.clear_bearer();

    Ok((
        user,
        token,
        store,
        category,
        base_product,
        attribute,
        custom_attribute,
    ))
}

pub fn set_up_published_store(
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

pub fn set_up_published_base_product(
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

pub fn set_up_published_product(
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
