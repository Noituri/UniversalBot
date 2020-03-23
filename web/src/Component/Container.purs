module Utter.Component.Container (component) where

import Prelude

import Data.Array ((:))
import Data.Maybe (Maybe)
import Halogen.HTML as HH
import Halogen.HTML.Properties as HP
import Utter.Component.Navbar as Navbar
import Utter.Component.Utils (cssClass)
import Utter.Data.User (User)

component :: forall i p. Maybe User -> String -> Array (HH.HTML i p) -> HH.HTML i p
component user title inner =
  HH.div [ cssClass "container" ] ((Navbar.component user) : titleHTML : inner)
  where
    titleHTML = HH.title_ [ HH.text $ "Utter - " <> title ]