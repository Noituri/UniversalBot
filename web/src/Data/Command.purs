module Utter.Data.Command where

type Command =
  { kind :: Int
  , name :: String
  , description :: String
  }

type CommandCategory =
  { name :: String
  , icon :: String
  }