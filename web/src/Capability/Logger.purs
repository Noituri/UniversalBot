module Utter.Capability.Logger (class Logger, log) where

import Prelude

import Control.Monad.Trans.Class (lift)
import Halogen as H

class Monad m <= Logger m where
  log :: String -> m Unit

instance loggerHalogenM :: Logger m => Logger (H.HalogenM st act slots msg m) where
  log = lift <<< log