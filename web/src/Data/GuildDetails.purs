module Utter.Data.GuildDetails where

import Utter.Data.Action (GuildAction)

type GuildDetails =
 { guild_id :: String
 , actions :: Array GuildAction
 , prefix :: String
 , muted_role :: String
 , mod_logs_channel :: String
 }