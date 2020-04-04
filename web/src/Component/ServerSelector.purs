module Utter.Component.ServerSelector where

import Prelude

import Data.Array (mapWithIndex)
import Data.Maybe (Maybe(..))
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import Utter.Capability.Logger (class Logger, log)
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Component.Utils (cssClass)
import Utter.Data.Route (Route(..))
import Utter.Data.Guild (Guild)


type Input = { servers :: Array Guild, selected :: Int }

type State = Input

data Action
  = HandleInput Input
  | SelectServer Int

component
  :: ∀ q o m
   . Navigate m
  => Logger m
  => H.Component HH.HTML q Input o m
component = H.mkComponent
  { initialState: identity
  , render
  , eval: H.mkEval $ H.defaultEval { handleAction = handleAction }
  }
  where
    handleAction :: ∀ slots. Action -> H.HalogenM State Action slots o m Unit
    handleAction = case _ of
      HandleInput n -> do
        oldN <- H.get
        when (oldN /= n) $ H.put n
      SelectServer n -> do
        navigate $ EditPanel n

render :: ∀ slots m. State -> H.ComponentHTML Action slots m
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
    entry :: ∀ i. Int -> Guild -> HH.HTML i Action
    entry ix { id, icon, name } =
      HH.div [ cssClass (if (ix == selected) then "selected" else "")
             , HE.onClick \_ -> Just $ SelectServer ix
             ]
        [ HH.img [ cssClass "circle", HP.src $ getIcon id icon ]
        , HH.p_ [ HH.text name ]
        ]
