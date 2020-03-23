module Utter.Component.Container (component) where

import Prelude

import Data.Array ((:))
import Halogen.HTML as HH
import Halogen.HTML.Properties as HP
import Utter.Component.Utils (cssClass)

component :: forall i p. String -> Array (HH.HTML i p) -> HH.HTML i p
component title inner =
  HH.div [ cssClass "container" ] (titleHTML : inner)
  where
    titleHTML = HH.title_ [ HH.text $ "Utter - " <> title ]