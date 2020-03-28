module Utter.Data.Command where

import Data.Maybe (Maybe)

type Command =
  { kind :: Int
  , name :: String
  , description :: String
  , details :: Maybe String
  }

type CommandCategory =
  { name :: String
  , icon :: String
  }