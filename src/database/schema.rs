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
        disabledcommands -> Array<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    roles,
    servers,
);
