module Utter.Component.ServerSettings where

import Prelude

import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import Utter.Capability.Navigate (class Navigate)
import Utter.Component.Utils (cssClass)

type Input =
  { prefix :: String
  , mutedRole :: String
  , modLogsChannel :: String
  }

type State = Input

data Action
  = HandleInput Input

component
  :: ∀ q o m
   . Navigate m
  => H.Component HH.HTML q Input o m
component = H.mkComponent
  { initialState
  , render
  , eval: H.mkEval $ H.defaultEval { handleAction = handleAction }
  }
  where
    initialState i = i
    handleAction :: ∀ slots. Action -> H.HalogenM State Action slots o m Unit
    handleAction = case _ of
      HandleInput n -> do
        st <- H.get
        when (st /= n) $ H.put n

render :: ∀ slots m. State -> H.ComponentHTML Action slots m
render { prefix, mutedRole, modLogsChannel } =
  HH.div [ cssClass "card" ]
    [ HH.h2_ [ HH.text "Settings" ]
    , HH.div [ cssClass "settings-container" ]
        [ HH.h4_ [ HH.text "Prefix" ]
        , HH.input [ cssClass "input-field", HP.placeholder "Bot Prefix", HP.value prefix ]
        , HH.h4_ [ HH.text "Muted Role" ]
        , HH.input [ cssClass "input-field", HP.placeholder "Muted Role", HP.value mutedRole ]
        , HH.h4_ [ HH.text "Mod-logs Channel" ]
        , HH.input [ cssClass "input-field", HP.placeholder "Mod-logs Channel", HP.value modLogsChannel ]
        , HH.p [ cssClass "top-margin gradient-btn" ]
            [ HH.text "Save" ]
        ]
    ]