module Utter.Component.FeatureCard (component) where

import Halogen.HTML as HH
import Utter.Component.Utils (cssClass)

component :: forall i p. String -> String -> HH.HTML i p
component title desc =
    HH.div [ cssClass "card" ]
        [ HH.h3_
            [ HH.text title ]
        , HH.p_
            [ HH.text desc ]
        ]