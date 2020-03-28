module Utter.Component.ItemsList where

import Prelude

import Data.Array (elem, mapWithIndex, snoc, deleteBy, (:))
import Data.Maybe (Maybe(..), fromMaybe, isJust)
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Utter.Capability.Logger (class Logger, log)
import Utter.Component.Utils (cssClass, isMaybeTrue, maybeElem, whenElem)
import Utter.Data.ListEntry (ListEntry)

type Input r = { title :: Maybe String, entries :: Array ListEntry | r }

type State = Input ( opened :: Array Int )

data Action
  = HandleInput (Input ())
  | ToggleDetails Int

component
  :: ∀ q o m
   . Logger m
  => H.Component HH.HTML q (Input ()) o m
component = H.mkComponent
  { initialState
  , render
  , eval: H.mkEval $ H.defaultEval
    { handleAction = handleAction
    , receive = Just <<< HandleInput
    }
  }
  where
    initialState { title, entries } =
      { title
      , entries
      , opened: mempty
      }
    handleAction :: ∀ slots. Action -> H.HalogenM State Action slots o m Unit
    handleAction = case _ of
      HandleInput { title, entries } -> do
        oldN <- H.get
        when ((isMaybeTrue $ (/=) <$> oldN.title <*> title) || (oldN.entries /= entries)) $
          H.put oldN { title = title, entries = entries }
      ToggleDetails n -> do
        st <- H.get
        if n `elem` st.opened
        then H.modify_ _ { opened = deleteBy (\v i -> v == i) n st.opened }
        else H.modify_ _ { opened = n : st.opened }

render :: ∀ slots m. State -> H.ComponentHTML Action slots m
render { title, entries, opened } =
  HH.div [ cssClass "card" ]
    [ maybeElem title \text -> HH.h2_ [ HH.text text ]
    , HH.div_
        (mapWithIndex entry entries)
    ]
  where
    entry :: ∀ i. Int -> ListEntry -> HH.HTML i Action
    entry ix { name, description, details } =
      HH.div [ cssClass "list-entry", HE.onClick \_ -> Just $ ToggleDetails ix ]
        [ HH.div [ cssClass "list-text" ]
            [ HH.h3_ [ HH.text name ]
            , HH.h5_ [ HH.text description ]
            ]
        , maybeElem details \text ->
            HH.div_
              [ HH.h6_ [ HH.text "Click to see details" ]
              , whenElem (ix `elem` opened) \_ ->
                HH.div [ cssClass "list-entry list-entry-details" ]
                  [ HH.div [ cssClass "list-text" ]
                      [ HH.h5_ [ HH.text text ] ]
                  ]
              ]
        ]