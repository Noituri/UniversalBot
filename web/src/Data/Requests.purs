module Utter.Data.Requests where

type ReqGuildDetails =
  { token :: String
  , guild_id :: String
  , actions_from :: Int
  }