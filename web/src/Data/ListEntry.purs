module Utter.Data.ListEntry where

import Data.Maybe (Maybe)

type ListEntry =
  { name :: String
  , description :: String
  , details :: Maybe String
  }