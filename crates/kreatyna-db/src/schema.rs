// @generated automatically by Diesel CLI.

diesel::table! {
    ingredient_units (ingredient_local_id, position) {
        ingredient_local_id -> BigInt,
        position -> BigInt,
        name -> Text,
        name_key -> Text,
        grams -> Nullable<Text>,
        millilitres -> Nullable<Text>,
    }
}

diesel::table! {
    ingredients (local_id) {
        local_id -> BigInt,
        remote_id -> Text,
        name -> Text,
        name_key -> Text,
        notes -> Nullable<Text>,
        basis -> Integer,
        calories -> Nullable<Text>,
        protein -> Nullable<Text>,
        carbohydrates -> Nullable<Text>,
        fat -> Nullable<Text>,
    }
}

diesel::table! {
    recipes (local_id) {
        local_id -> BigInt,
        remote_id -> Text,
        title -> Text,
        title_key -> Text,
    }
}

diesel::joinable!(ingredient_units -> ingredients (ingredient_local_id));

diesel::allow_tables_to_appear_in_same_query!(ingredient_units, ingredients, recipes,);
