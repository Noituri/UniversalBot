table! {
    servers (id) {
        id -> Int4,
        guildid -> Varchar,
        ownerid -> Varchar,
        enabledmodules -> Nullable<Array<Text>>,
        enabledcommands -> Nullable<Array<Text>>,
    }
}
