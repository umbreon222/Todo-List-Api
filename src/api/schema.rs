table! {
    creation_information (id) {
        id -> Integer,
        uuid -> Text,
        creator_user_uuid -> Text,
        creation_time -> Text,
        last_updated_by_user_uuid -> Text,
        last_updated_time -> Text,
    }
}

table! {
    lists (id) {
        id -> Integer,
        uuid -> Text,
        title -> Text,
        description -> Nullable<Text>,
        color_hex -> Nullable<Text>,
        task_uuids -> Nullable<Text>,
        parent_list_uuid -> Nullable<Text>,
        sub_list_uuids -> Nullable<Text>,
        shared_with_user_uuids -> Nullable<Text>,
        creation_information_uuid -> Text,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        uuid -> Text,
        content -> Text,
        priority -> Integer,
        tags -> Nullable<Text>,
        is_complete -> Bool,
        creation_information_uuid -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        uuid -> Text,
        username -> Text,
        password_hash -> Text,
        nickname -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    creation_information,
    lists,
    tasks,
    users,
);
