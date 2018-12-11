extern crate failure;
extern crate functional_tests;

use failure::Error as FailureError;

use functional_tests::query::*;

use functional_tests::context::TestContext;

#[test]
pub fn update_store() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    //when
    let updated_store = context
        .update_store(update_store::UpdateStoreInput {
            id: store.create_store.id,
            ..update_store::default_update_store_input()
        })
        .unwrap()
        .update_store;
    //then
    let expected_values = update_store::default_update_store_input();
    verify_update_store_values(updated_store, expected_values);
}

#[test]
pub fn update_store_does_not_update_rating() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    let initial_rating = store.create_store.rating;
    //when
    let updated_store = context
        .update_store(update_store::UpdateStoreInput {
            id: store.create_store.id,
            ..update_store::default_update_store_input()
        })
        .unwrap()
        .update_store;
    //then
    assert!((updated_store.rating - initial_rating).abs() < 0.001);
}

#[test]
pub fn delete_attribute() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let attribute = context
        .create_attribute(create_attribute::default_create_attribute_input())
        .unwrap()
        .create_attribute;
    //when
    let _ = context
        .delete_attribute(delete_attribute::DeleteAttributeInput {
            id: attribute.id,
            ..delete_attribute::default_delete_attribute_input()
        })
        .unwrap();
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
        .create_attribute(create_attribute::default_create_attribute_input())
        .unwrap()
        .create_attribute;
    //when
    let updated_attribute = context
        .update_attribute(update_attribute::UpdateAttributeInput {
            id: attribute.id,
            ..update_attribute::default_update_attribute_input()
        })
        .unwrap()
        .update_attribute;
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
        .create_category(create_category::default_create_category_input())
        .unwrap()
        .create_category;
    let attribute = context
        .create_attribute(create_attribute::default_create_attribute_input())
        .unwrap()
        .create_attribute;
    let _ = context
        .add_attribute_to_category(add_attribute_to_category::AddAttributeToCategoryInput {
            cat_id: category.raw_id,
            attr_id: attribute.raw_id,
            ..add_attribute_to_category::default_add_attribute_to_categoryinput()
        })
        .unwrap();
    //when
    let _ = context
        .delete_attribute_from_category(
            delete_attribute_from_category::DeleteAttributeFromCategory {
                cat_id: category.raw_id,
                attr_id: attribute.raw_id,
                ..delete_attribute_from_category::default_delete_attribute_from_category_input()
            },
        )
        .unwrap();
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
pub fn add_attribute_to_category() {
    //setup
    let mut context = TestContext::new();
    //given
    context.as_superadmin();
    let category = context
        .create_category(create_category::default_create_category_input())
        .unwrap()
        .create_category;
    let attribute = context
        .create_attribute(create_attribute::default_create_attribute_input())
        .unwrap()
        .create_attribute;
    //when
    let _ = context
        .add_attribute_to_category(add_attribute_to_category::AddAttributeToCategoryInput {
            cat_id: category.raw_id,
            attr_id: attribute.raw_id,
            ..add_attribute_to_category::default_add_attribute_to_categoryinput()
        })
        .unwrap();
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
        .create_category(create_category::default_create_category_input())
        .unwrap()
        .create_category;
    //when
    let _ = context
        .delete_category(delete_category::DeleteCategoryInput {
            cat_id: category.raw_id,
            ..delete_category::default_delete_category_input()
        })
        .unwrap()
        .delete_category;
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
        .create_category(create_category::default_create_category_input())
        .unwrap()
        .create_category;
    //when
    let updated_category = context
        .update_category(update_category::UpdateCategoryInput {
            id: category.id,
            ..update_category::default_update_category_input()
        })
        .unwrap()
        .update_category;
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
pub fn microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.microservice_healthcheck().unwrap();
}

#[test]
pub fn create_base_product() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, category) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    let new_base_product = create_base_product::CreateBaseProductInput {
        store_id: store.create_store.raw_id,
        category_id: category.create_category.raw_id,
        ..create_base_product::default_create_base_product_input()
    };
    //when
    let base_product = context
        .create_base_product(new_base_product)
        .unwrap()
        .create_base_product;
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
        .create_attribute(create_attribute::default_create_attribute_input())
        .unwrap()
        .create_attribute;
    let new_value = context
        .create_attribute_value(create_attribute_value::CreateAttributeValueInput {
            raw_attribute_id: attribute.raw_id,
            ..create_attribute_value::default_create_attribute_value_input()
        })
        .unwrap()
        .create_attribute_value;
    //when
    let _ = context
        .delete_attribute_value(delete_attribute_value::DeleteAttributeValueInput {
            raw_id: new_value.raw_id,
            ..delete_attribute_value::default_delete_attribute_value_input()
        })
        .unwrap()
        .delete_attribute_value;
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
        .create_attribute(create_attribute::default_create_attribute_input())
        .unwrap()
        .create_attribute;
    let new_value = context
        .create_attribute_value(create_attribute_value::CreateAttributeValueInput {
            raw_attribute_id: attribute.raw_id,
            ..create_attribute_value::default_create_attribute_value_input()
        })
        .unwrap()
        .create_attribute_value;
    //when
    let updated = context
        .update_attribute_value(update_attribute_value::UpdateAttributeValueInput {
            raw_id: new_value.raw_id,
            raw_attribute_id: attribute.raw_id,
            ..update_attribute_value::default_create_attribute_value_input()
        })
        .unwrap()
        .update_attribute_value;
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
        .create_attribute(create_attribute::default_create_attribute_input())
        .unwrap()
        .create_attribute;
    //when
    let new_value = context
        .create_attribute_value(create_attribute_value::CreateAttributeValueInput {
            raw_attribute_id: attribute.raw_id,
            ..create_attribute_value::default_create_attribute_value_input()
        })
        .unwrap()
        .create_attribute_value;
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
        .create_attribute(create_attribute::CreateAttributeInput {
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
        .unwrap()
        .create_attribute;
}

#[test]
pub fn create_attribute() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //then
    let _attribute = context
        .create_attribute(create_attribute::default_create_attribute_input())
        .unwrap()
        .create_attribute;
}

#[test]
pub fn create_subcategories() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let category_level_1 = context
        .create_category(create_category::default_create_category_input())
        .unwrap()
        .create_category;
    let category_level_2 = context
        .create_category(create_category::CreateCategoryInput {
            parent_id: category_level_1.raw_id,
            slug: Some("category-slug-1".to_string()),
            ..create_category::default_create_category_input()
        })
        .unwrap()
        .create_category;
    //when
    let category_level_3 = context
        .create_category(create_category::CreateCategoryInput {
            parent_id: category_level_2.raw_id,
            slug: Some("category-slug-2".to_string()),
            ..create_category::default_create_category_input()
        })
        .unwrap()
        .create_category;
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
        .create_category(new_category)
        .unwrap()
        .create_category;
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
        .create_user(create_user::default_create_user_input())
        .unwrap()
        .create_user;
    context.verify_email(&user.email).unwrap();
    let token: String = context
        .get_jwt_by_email(get_jwt_by_email::default_create_jwt_email_input())
        .unwrap()
        .get_jwt_by_email
        .token;
    context.set_bearer(token);
    //when
    let store = context
        .create_store(create_store::CreateStoreInput {
            user_id: user.raw_id,
            ..create_store::default_create_store_input()
        })
        .unwrap()
        .create_store;
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
    let user = context.create_user(new_user).unwrap().create_user;
    //then
    assert_eq!(user.email, create_user::default_create_user_input().email);
}

#[test]
pub fn delete_user() {
    //setup
    let mut context = TestContext::new();
    //given
    let new_user = create_user::default_create_user_input();
    let user = context.create_user(new_user).unwrap().create_user;
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
        .create_user(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..create_user::default_create_user_input()
        })
        .unwrap();

    let new_user = create_user::CreateUserInput {
        additional_data: Some(create_user::NewUserAdditionalDataInput {
            country: Some("MMR".to_string()),
            referal: Some(referal.create_user.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks: Some(vec![create_user::UtmMarkInput {
                key: "source".to_string(),
                value: "word_of_mouth".to_string(),
            }]),
        }),
        ..create_user::default_create_user_input()
    };
    //when
    let user = context.create_user(new_user).unwrap().create_user;
    //then
    assert_eq!(user.email, create_user::default_create_user_input().email);
    assert_eq!(user.referal.unwrap(), referal.create_user.raw_id);
    assert_eq!(user.country.unwrap(), "MMR".to_string());
    assert_eq!(user.referer.unwrap(), "localhost".to_string());
    assert_eq!(&user.utm_marks.as_ref().unwrap()[0].key, "source");
    assert_eq!(&user.utm_marks.as_ref().unwrap()[0].value, "word_of_mouth");
}

#[test]
#[ignore]
pub fn create_user_via_facebook() {
    //setup
    let context = TestContext::new();
    //given
    let facebook_jwt = get_jwt_by_provider::facebook_create_jwt_provider_input();
    //when
    let user = context.create_user_jwt(facebook_jwt);
    //then
    assert!(user.is_ok());
}

#[test]
#[ignore]
pub fn create_user_via_google() {
    //setup
    let context = TestContext::new();
    //given
    let google_jwt = get_jwt_by_provider::google_create_jwt_provider_input();
    //when
    let user = context.create_user_jwt(google_jwt);
    //then
    assert!(user.is_ok());
}

#[test]
#[ignore]
pub fn create_user_via_facebook_with_additional_data() {
    //setup
    let context = TestContext::new();
    //given
    let referal = context
        .create_user(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..create_user::default_create_user_input()
        })
        .unwrap();

    let facebook_jwt = get_jwt_by_provider::CreateJWTProviderInput {
        additional_data: Some(get_jwt_by_provider::NewUserAdditionalDataInput {
            country: Some("MMR".to_string()),
            referal: Some(referal.create_user.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks: Some(vec![get_jwt_by_provider::UtmMarkInput {
                key: "source".to_string(),
                value: "word_of_mouth".to_string(),
            }]),
        }),
        ..get_jwt_by_provider::facebook_create_jwt_provider_input()
    };
    //when
    let user = context.create_user_jwt(facebook_jwt);
    //then
    assert!(user.is_ok());
}

#[test]
#[ignore]
pub fn create_user_via_google_with_additional_data() {
    //setup
    let context = TestContext::new();
    //given
    let referal = context
        .create_user(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..create_user::default_create_user_input()
        })
        .unwrap();

    let google_jwt = get_jwt_by_provider::CreateJWTProviderInput {
        additional_data: Some(get_jwt_by_provider::NewUserAdditionalDataInput {
            country: Some("MMR".to_string()),
            referal: Some(referal.create_user.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks: Some(vec![get_jwt_by_provider::UtmMarkInput {
                key: "source".to_string(),
                value: "word_of_mouth".to_string(),
            }]),
        }),
        ..get_jwt_by_provider::google_create_jwt_provider_input()
    };
    //when
    let user = context.create_user_jwt(google_jwt);
    //then
    assert!(user.is_ok());
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
    let delete_result = context.delete_store(store.create_store.raw_id);
    //then
    assert!(delete_result.is_ok())
}

fn set_up_store(
    context: &mut TestContext,
) -> Result<
    (
        create_user::ResponseData,
        String,
        create_store::ResponseData,
        create_category::ResponseData,
    ),
    FailureError,
> {
    let user = context.create_user(create_user::default_create_user_input())?;
    context.verify_email(&user.create_user.email).unwrap();
    let token: String = context
        .get_jwt_by_email(get_jwt_by_email::default_create_jwt_email_input())?
        .get_jwt_by_email
        .token;
    context.set_bearer(token.clone());
    let store = context.create_store(create_store::CreateStoreInput {
        user_id: user.create_user.raw_id,
        ..create_store::default_create_store_input()
    })?;
    context.as_superadmin();
    let category_level_1 = context
        .create_category(create_category::default_create_category_input())?
        .create_category;
    let category_level_2 = context
        .create_category(create_category::CreateCategoryInput {
            parent_id: category_level_1.raw_id,
            slug: Some("category-slug-1".to_string()),
            ..create_category::default_create_category_input()
        })?
        .create_category;
    let category_level_3 = context.create_category(create_category::CreateCategoryInput {
        parent_id: category_level_2.raw_id,
        slug: Some("category-slug-2".to_string()),
        ..create_category::default_create_category_input()
    })?;
    context.clear_bearer();
    Ok((user, token, store, category_level_3))
}

fn verify_update_store_values(
    updated_store: update_store::RustUpdateStoreUpdateStore,
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
