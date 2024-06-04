// @generated automatically by Diesel CLI.

diesel::table! {
    env_vars (id) {
        id -> Nullable<Integer>,
        key -> Text,
        value -> Text,
    }
}
