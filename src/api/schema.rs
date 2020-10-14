#![allow(non_snake_case)]
table! {
    CreationInformation (ID) {
        ID -> Integer,
        UUID -> Text,
        CreatorUserUUID -> Text,
        CreationTime -> Text,
        LastUpdatedByUserUUID -> Text,
        LastUpdatedTime -> Text,
    }
}

table! {
    Lists (ID) {
        ID -> Integer,
        UUID -> Text,
        Title -> Text,
        Description -> Nullable<Text>,
        ColorHex -> Nullable<Text>,
        TaskUUIDs -> Nullable<Text>,
        ParentListUUID -> Nullable<Text>,
        SubListUUIDs -> Nullable<Text>,
        SharedWithUserUUIDs -> Nullable<Text>,
        CreationInformationUUID -> Text,
    }
}

table! {
    Tags (ID) {
        ID -> Integer,
        UUID -> Text,
        Title -> Text,
        CreationInformationUUID -> Text,
    }
}

table! {
    Tasks (ID) {
        ID -> Integer,
        UUID -> Text,
        Content -> Text,
        Priority -> Integer,
        TagUUIDs -> Nullable<Text>,
        IsComplete -> Bool,
        CreationInformationUUID -> Text,
    }
}

table! {
    Users (ID) {
        ID -> Integer,
        UUID -> Text,
        Username -> Text,
        PasswordHash -> Text,
        Nickname -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    CreationInformation,
    Lists,
    Tags,
    Tasks,
    Users,
);
