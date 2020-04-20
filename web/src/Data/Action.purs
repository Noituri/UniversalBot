module Utter.Data.Action where
  
type GuildAction =
  { action_type :: Int
  , issuer :: String
  , target :: String
  , message :: String
  , creation_date :: String
  }