module Utter.Component.ServerSelector (component, Slot, Message(..)) where

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
import Utter.Data.Guild (Guild)
import Utter.Data.Route (Route(..))

type Slot a = ∀ q. H.Slot q Message a

type Input = { servers :: Array Guild, selected :: Int }

type State = Input

data Message = SelectedServer Int

data Action
  = HandleInput Input
  | SelectServer Int


component
  :: ∀ q m
   . Navigate m
  => Logger m
  => H.Component HH.HTML q Input Message m
component = H.mkComponent
  { initialState: identity
  , render
  , eval: H.mkEval $ H.defaultEval { handleAction = handleAction }
  }
  where
    handleAction :: ∀ slots. Action -> H.HalogenM State Action slots Message m Unit
    handleAction = case _ of
      HandleInput n -> do
        oldN <- H.get
        when (oldN /= n) $ H.put n
      SelectServer n -> do
        st <- H.get
        H.modify_ \_ -> st { selected = n }
        H.raise $ SelectedServer n

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
    entry ix { id, icon, name, access } =
      HH.div [ cssClass (if (ix == selected) then "selected" else if not access then " noaccess" else "")
             , HE.onClick \_ -> Just $ SelectServer ix
             ]
        [ HH.img [ cssClass "circle", HP.src $ getIcon id icon ]
        , HH.p_ [ HH.text name ]
        ]
