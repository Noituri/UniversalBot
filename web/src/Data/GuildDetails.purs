module Utter.Data.GuildDetails where

import Utter.Data.Action (Action)

type GuildDetails =
 { guild_id :: String
 , actions :: Array Action
 , prefix :: String
 , muted_role :: String
 , mod_logs_channel :: String
 }