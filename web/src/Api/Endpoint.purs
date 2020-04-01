module Utter.Api.Endpoint where

import Prelude

data Endpoint
  = ExchangeCode

rootAddress :: Int -> String
rootAddress port =
  if true -- if debug
  then "http://localhost:" <> show port
  else "NOT IMPLEMENTED"

endpointUrl :: Endpoint -> String
endpointUrl = case _ of
  ExchangeCode -> (rootAddress 8080) <> "/login"