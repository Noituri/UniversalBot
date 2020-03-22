module Utter.Data.Route where

import Prelude hiding ((/))

import Data.Generic.Rep (class Generic)
import Data.Generic.Rep.Show (genericShow)
import Routing.Duplex (RouteDuplex', as, root, segment)
import Routing.Duplex.Generic (noArgs, sum)

data Route
  = Home
--   | Redirect Int
--   | Panel Int
--   | Commands

derive instance genericRoute :: Generic Route _
derive instance eqRoute :: Eq Route
derive instance ordRoute :: Ord Route

instance showRoute :: Show Route where
  show = genericShow

routeDuplex :: RouteDuplex' Route
routeDuplex = root $ sum
  { "Home": noArgs
  }