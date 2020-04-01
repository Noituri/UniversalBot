module Utter.Capability.Api where

import Prelude

import Data.Maybe (Maybe)
import Halogen (lift)
import Halogen as H
import Utter.Data.User (User)

class Monad m <= Api m where
  signin :: String -> m (Maybe User)

instance loggerHalogenM :: Api m => Api (H.HalogenM st act slots msg m) where
  signin = lift <<< signin