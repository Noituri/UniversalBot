module Utter.Data.Action where
  
type Action =
  { action_type :: Int
  , issuer :: String
  , target :: String
  , message :: String
  , creation_date :: String
  }