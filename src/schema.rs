// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
    }
}

diesel::table! {
    projects_progress (id) {
        id -> Integer,
        project_id -> Integer,
        step -> Integer,
        sub_step -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    projects_progress,
);
