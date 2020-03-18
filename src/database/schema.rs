table! {
    actions (id) {
        id -> Int4,
        server_id -> Int4,
        action_type -> Int4,
        issuer -> Varchar,
        target -> Nullable<Varchar>,
        message -> Varchar,
        creation_date -> Timestamp,
    }
}

table! {
    commands (id) {
        id -> Int4,
        server_id -> Int4,
        command_name -> Varchar,
        disabled_channels -> Array<Text>,
    }
}

table! {
    roles (id) {
        id -> Int4,
        server_id -> Int4,
        role_id -> Varchar,
        perms -> Array<Text>,
    }
}

table! {
    servers (id) {
        id -> Int4,
        guildid -> Varchar,
        prefix -> Varchar,
        enabledmodules -> Array<Text>,
    }
}

table! {
    special_entities (id) {
        id -> Int4,
        server_id -> Int4,
        entity_type -> Int4,
        entity_id -> Varchar,
    }
}

table! {
    temp_operations (id) {
        id -> Int4,
        server_id -> Int4,
        action_type -> Int4,
        target_id -> Varchar,
        end_date -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    actions,
    commands,
    roles,
    servers,
    special_entities,
    temp_operations,
);
