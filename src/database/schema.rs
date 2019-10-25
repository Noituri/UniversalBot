table! {
    servers (id) {
        id -> Int4,
        guildid -> Varchar,
        prefix -> Varchar,
        enabledmodules -> Array<Text>,
        disabledcommands -> Array<Text>,
    }
}
