module Utter.Api.Endpoint where

import Prelude

data Endpoint
  = ExchangeCodeEndpoint
  | GuildsEndpoint
  | GuildDetailsEndpoint
  | ModifyGuildEndpoint

rootAddress :: Int -> String
rootAddress debugPort =
  if true -- if debug
  then "http://localhost:" <> show debugPort
  else "NOT IMPLEMENTED"

endpointUrl :: Endpoint -> String
endpointUrl = case _ of
  ExchangeCodeEndpoint -> (rootAddress 8080) <> "/login"
  GuildsEndpoint -> (rootAddress 8090) <> "/guilds"
  GuildDetailsEndpoint -> (rootAddress 8100) <> "/guild-details"
  ModifyGuildEndpoint -> (rootAddress 8110) <> "/modify-guild"