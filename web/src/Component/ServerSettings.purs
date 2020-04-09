module Utter.Component.ServerSettings (component, Slot, Message(..), Input) where

import Prelude

import Data.Maybe (Maybe(..))
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import Utter.Capability.Navigate (class Navigate)
import Utter.Component.Utils (cssClass)

type Slot a = ∀ q. H.Slot q Message a

data Message = SaveSettings Input

type Input =
  { prefix :: String
  , mutedRole :: String
  , modLogsChannel :: String
  }

type State = Input

data Action
  = HandleInput Input
  | HandlePrefix String
  | HandleMutedRole String
  | HandleModLogsChannel String
  | HandleSave

component
  :: ∀ q m
   . Navigate m
  => H.Component HH.HTML q Input Message m
component = H.mkComponent
  { initialState
  , render
  , eval: H.mkEval $ H.defaultEval { handleAction = handleAction }
  }
  where
    initialState i = i
    handleAction :: ∀ slots. Action -> H.HalogenM State Action slots Message m Unit
    handleAction = case _ of
      HandleInput n -> do
        st <- H.get
        when (st /= n) $ H.put n
      HandlePrefix prefix ->
        H.modify_ \st -> st { prefix = prefix }
      HandleMutedRole role ->
        H.modify_ \st -> st { mutedRole = role }
      HandleModLogsChannel channel ->
        H.modify_ \st -> st { modLogsChannel = channel }
      HandleSave -> do
        st <- H.get
        H.raise $ SaveSettings st

render :: ∀ slots m. State -> H.ComponentHTML Action slots m
render { prefix, mutedRole, modLogsChannel } =
  HH.div [ cssClass "card" ]
    [ HH.h2_ [ HH.text "Settings" ]
    , HH.div [ cssClass "settings-container" ]
        [ HH.h4_ [ HH.text "Prefix" ]
        , HH.input [ cssClass "input-field", HP.placeholder "Bot Prefix", HP.value prefix, HE.onValueInput (Just <<< HandlePrefix) ]
        , HH.h4_ [ HH.text "Muted Role" ]
        , HH.input [ cssClass "input-field", HP.placeholder "Muted Role", HP.value mutedRole, HE.onValueInput (Just <<< HandleMutedRole) ]
        , HH.h4_ [ HH.text "Mod-logs Channel" ]
        , HH.input [ cssClass "input-field", HP.placeholder "Mod-logs Channel", HP.value modLogsChannel, HE.onValueInput (Just <<< HandleModLogsChannel) ]
        , HH.p [ cssClass "top-margin gradient-btn", HE.onClick \_ -> Just HandleSave ]
            [ HH.text "Save" ]
        ]
    ]