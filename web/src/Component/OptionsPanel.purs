module Utter.Component.OptionsPanel where
  
import Prelude

import Data.Array (mapWithIndex)
import Data.Maybe (Maybe, isJust)
import Halogen.HTML as HH
import Utter.Component.Utils (cssClass, maybeElem)

type Option = String

component :: forall i p. Maybe String -> Array Option -> Int -> HH.HTML i p
component title options selected =
  HH.div [ cssClass "card" ]
    [ maybeElem title \text -> HH.h2_ [ HH.text text ]
    , HH.div [ cssClass "horizontal-view" ]
        (mapWithIndex option options)
    ]
  where
    option :: Int -> Option -> HH.HTML i p
    option ix icon =
      HH.div [ cssClass $ "panel-option" <> (if (ix == selected)  then" selected" else "") ]
        [ HH.i [ cssClass $ "option-el fas " <> icon ] [] ]