module Utter.Component.ServerSelector where

import Prelude

import Data.Array (mapWithIndex)
import Halogen.HTML as HH
import Halogen.HTML.Properties as HP
import Utter.Component.Utils (cssClass)
import Utter.Data.Server (Server)

component :: forall i p. Array Server -> Int -> HH.HTML i p
component servers selected =
  HH.div [ cssClass "card" ]
    [ HH.h2_ [ HH.text "Server Selector" ]
    , HH.div [ cssClass "horizontal-view small" ]
        (mapWithIndex entry servers)
    ]
  where
    getIcon :: String -> String -> String
    getIcon _ "" = "https://cdn.discordapp.com/embed/avatars/0.png"
    getIcon id hash = "https://cdn.discordapp.com/icons/" <> id <> "/" <> hash <> ".png"
    entry :: Int -> Server -> HH.HTML i p
    entry ix { id, icon, name } =
      HH.div [ cssClass (if (ix == selected) then "selected" else "") ]
        [ HH.img [ cssClass "circle", HP.src $ getIcon id icon ]
        , HH.p_ [ HH.text name ]
        ]
