table! {
    servers (id) {
        id -> Int4,
        guildid -> Varchar,
        ownerid -> Varchar,
        enabledmodules -> Array<Text>,
        enabledcommands -> Array<Text>,
    }
}
