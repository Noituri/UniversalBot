module Utter.Capability.Navigate (class Navigate, navigate) where
import Prelude

import Control.Monad.Trans.Class (lift)
import Utter.Data.Route (Route)
import Halogen as H

class Monad m <= Navigate m where
  navigate :: Route -> m Unit

instance navigateHalogenM :: Navigate m => Navigate (H.HalogenM st act slots msg m) where
  navigate = lift <<< navigate