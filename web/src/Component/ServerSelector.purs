module Utter.Component.ServerSelector where

import Prelude

import Data.Array (mapWithIndex)
import Data.Maybe (Maybe(..))
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Properties as HP
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Component.Utils (cssClass)
import Utter.Data.Route (Route(..))
import Utter.Data.Server (Server)


type Input = { servers :: Array Server, selected :: Int }

type State = Input

data Action
  = HandleInput Input
  | SelectServer Int

component
  :: forall q o m
   . MonadAff m
  => Navigate m
  => H.Component HH.HTML q Input o m
component = H.mkComponent
  { initialState: identity
  , render
  , eval: H.mkEval $ H.defaultEval
      { handleAction = handleAction
      , receive = Just <<< HandleInput
      }
  }
  where
    handleAction :: forall slots. Action -> H.HalogenM State Action slots o m Unit
    handleAction = case _ of
      HandleInput n -> do
        oldN <- H.get
        when (oldN /= n) $ H.put n
      SelectServer n -> do
        navigate $ EditPanel n

render :: forall slots m. State -> H.ComponentHTML Action slots m
render { servers, selected } =
  HH.div [ cssClass "card" ]
    [ HH.h2_ [ HH.text "Server Selector" ]
    , HH.div [ cssClass "horizontal-view small" ]
        (mapWithIndex entry servers)
    ]
  where
    getIcon :: String -> String -> String
    getIcon _ "" = "https://cdn.discordapp.com/embed/avatars/0.png"
    getIcon id hash = "https://cdn.discordapp.com/icons/" <> id <> "/" <> hash <> ".png"
    entry :: forall i p. Int -> Server -> HH.HTML i p
    entry ix { id, icon, name } =
      HH.div [ cssClass (if (ix == selected) then "selected" else "") ]
        [ HH.img [ cssClass "circle", HP.src $ getIcon id icon ]
        , HH.p_ [ HH.text name ]
        ]
