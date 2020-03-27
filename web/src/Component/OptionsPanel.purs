module Utter.Component.OptionsPanel (component, Slot, Message(..)) where
  
import Prelude

import Data.Array (mapWithIndex)
import Data.Maybe (Maybe(..))
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Utter.Capability.Logger (class Logger, log)
import Utter.Capability.Navigate (class Navigate)
import Utter.Component.Utils (cssClass, maybeElem)

type Slot a = ∀ q. H.Slot q Message a

type Input =
  { title :: Maybe String
  , options :: Array String
  , selected :: Int
  }

type State = Input

data Message = SelectedOption Int

data Action
  = HandleInput Input
  | SelectOption Int

component
  :: ∀ q m
   . Navigate m
  => Logger m
  => H.Component HH.HTML q Input Message m
component = H.mkComponent
  { initialState: identity
  , render
  , eval: H.mkEval $ H.defaultEval
      { handleAction = handleAction
      , receive = Just <<< HandleInput
      }
  }
  where
    handleAction :: ∀ slots. Action -> H.HalogenM State Action slots Message m Unit
    handleAction = case _ of
      HandleInput n -> do
        oldN <- H.get
        when (oldN /= n) $ H.put n
      SelectOption n -> do
        H.raise $ SelectedOption n

render :: ∀ slots m. State -> H.ComponentHTML Action slots m
render { title, options, selected } =
  HH.div [ cssClass "card" ]
    [ maybeElem title \text -> HH.h2_ [ HH.text text ]
    , HH.div [ cssClass "horizontal-view" ]
        (mapWithIndex option options)
    ]
  where
    option :: ∀ i. Int -> String -> HH.HTML i Action
    option ix icon =
      HH.div [ cssClass $ "panel-option" <> (if (ix == selected)  then" selected" else "")
             , HE.onClick \_ -> Just $ SelectOption ix
             ]
        [ HH.i [ cssClass $ "option-el fas " <> icon ] [] ]