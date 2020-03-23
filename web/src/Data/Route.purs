module Utter.Data.Route where

import Prelude hiding ((/))

import Data.Either (note)
import Data.Generic.Rep (class Generic)
import Data.Generic.Rep.Show (genericShow)
import Data.Int (decimal, fromString, toStringAs)
import Routing.Duplex (RouteDuplex', as, root, segment)
import Routing.Duplex.Generic (noArgs, sum)
import Routing.Duplex.Generic.Syntax ((/))

data Route
  = Home
  | Panel
  | EditPanel Int
  | NotFound
--   | Redirect Int
--   | Commands

derive instance genericRoute :: Generic Route _
derive instance eqRoute :: Eq Route
derive instance ordRoute :: Ord Route

instance showRoute :: Show Route where
  show = genericShow

routeDuplex :: RouteDuplex' Route
routeDuplex = root $ sum
  { "Home": noArgs
  , "Panel": "panel" / noArgs
  , "EditPanel": "panel" / int segment
  , "NotFound": "not-found" / noArgs
  }

int :: RouteDuplex' String -> RouteDuplex' Int
int = as (toStringAs decimal) (fromString >>> note "Expected an integer value")