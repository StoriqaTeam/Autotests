extern crate functional_tests;

use functional_tests::query::{
    create_attribute, create_attribute_value, create_category, create_store, create_user,
    delete_attribute_value, get_jwt_by_email, get_jwt_by_provider, update_attribute_value,
};

use functional_tests::context::TestContext;

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
        }).unwrap()
        .create_attribute_value;
    //when
    let _ = context
        .delete_attribute_value(delete_attribute_value::DeleteAttributeValueInput {
            raw_id: new_value.raw_id,
            ..delete_attribute_value::default_delete_attribute_value_input()
        }).unwrap()
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
        }).unwrap()
        .create_attribute_value;
    //when
    let updated = context
        .update_attribute_value(update_attribute_value::UpdateAttributeValueInput {
            raw_id: new_value.raw_id,
            raw_attribute_id: attribute.raw_id,
            ..update_attribute_value::default_create_attribute_value_input()
        }).unwrap()
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
        }).unwrap()
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
        }).unwrap()
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
        }).unwrap()
        .create_category;
    //when
    let category_level_3 = context
        .create_category(create_category::CreateCategoryInput {
            parent_id: category_level_2.raw_id,
            slug: Some("category-slug-2".to_string()),
            ..create_category::default_create_category_input()
        }).unwrap()
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
        }).unwrap()
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
pub fn create_user_with_additional_data() {
    //setup
    let context = TestContext::new();
    //given
    let referal = context
        .create_user(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..create_user::default_create_user_input()
        }).unwrap();

    let new_user = create_user::CreateUserInput {
        additional_data: Some(create_user::NewUserAdditionalDataInput {
            country: Some("MMR".to_string()),
            referal: Some(referal.create_user.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks: Some(vec![create_user::UtmMark {
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
        }).unwrap();

    let facebook_jwt = get_jwt_by_provider::CreateJWTProviderInput {
        additional_data: Some(get_jwt_by_provider::NewUserAdditionalDataInput {
            country: Some("MMR".to_string()),
            referal: Some(referal.create_user.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks: Some(vec![get_jwt_by_provider::UtmMark {
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
        }).unwrap();

    let google_jwt = get_jwt_by_provider::CreateJWTProviderInput {
        additional_data: Some(get_jwt_by_provider::NewUserAdditionalDataInput {
            country: Some("MMR".to_string()),
            referal: Some(referal.create_user.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks: Some(vec![get_jwt_by_provider::UtmMark {
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
