table! {
    servers (id) {
        id -> Int4,
        guildid -> Varchar,
        enabledmodules -> Array<Text>,
        enabledcommands -> Array<Text>,
    }
}
