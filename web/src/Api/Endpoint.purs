module Utter.Api.Endpoint where

import Prelude

data Endpoint
  = ExchangeCode
  | Guilds
  | GuildDetails
  | ModifyGuild

rootAddress :: Int -> String
rootAddress debugPort =
  if true -- if debug
  then "http://localhost:" <> show debugPort
  else "NOT IMPLEMENTED"

endpointUrl :: Endpoint -> String
endpointUrl = case _ of
  ExchangeCode -> (rootAddress 8080) <> "/login"
  Guilds -> (rootAddress 8090) <> "/guilds"
  GuildDetails -> (rootAddress 8100) <> "/guild-details"
  ModifyGuild -> (rootAddress 8110) <> "/modify-guild"