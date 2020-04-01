module Utter.Page.Commands where

import Prelude

import Control.Monad.Reader (class MonadAsk)
import Data.Array ((!!), filter)
import Data.Maybe (Maybe(..))
import Data.String (Pattern(..), contains, null)
import Data.Symbol (SProxy(..))
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import Utter.Capability.Logger (class Logger)
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Component.Container as Container
import Utter.Component.ItemsList as ItemsList
import Utter.Component.OptionsPanel as OptionsPanel
import Utter.Component.Utils (ChildSlot, cssClass)
import Utter.Component.Wrapper as Wrapper
import Utter.Data.Command (Command, CommandCategory)
import Utter.Data.ListEntry (ListEntry)
import Utter.Data.Route (Route(..))
import Utter.Data.User (User)
import Utter.Env (UserEnv)

type Input = { category :: Int }

type State =
  { user :: Maybe User
  , selectedCategory :: Int
  , search :: String
  }

data Action
  = Receive { user :: Maybe User, category :: Int }
  | HandleOptionsMessage OptionsPanel.Message
  | HandleSearch String

type ChildSlots =
  ( optionsPanel :: OptionsPanel.Slot Unit
  , itemsList :: ChildSlot Unit
  )

commands :: Array Command
commands =
  [ { kind: 1, name: ".help", description: "Shows infromations about commands", details: Nothing }
  , { kind: 1, name: ".prefix", description: "Changes prefix", details: Nothing }
  , { kind: 2, name: ".ban", description: "Bans users from your server", details: Nothing }
  ]

categories :: Array CommandCategory
categories =
  [ { name: "All Commands", icon: "fa-border-all" }
  , { name: "Main Commands", icon: "fa-star" }
  , { name: "Moderation Commands", icon: "fa-gavel" }
  , { name: "Ticket Commands", icon: "fa-ticket-alt" }
  ]

component
  :: ∀ q o m r
   . MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => Navigate m
  => Logger m
  => H.Component HH.HTML q Input o m
component = Wrapper.component $ H.mkComponent
  { initialState
  , render
  , eval: H.mkEval $ H.defaultEval
      { handleAction = handleAction
      , receive = Just <<< Receive
      }
  }
  where
    initialState { user, category } =
      { user
      , selectedCategory: category
      , search: ""
      }
    handleAction :: Action -> H.HalogenM State Action ChildSlots o m Unit
    handleAction = case _ of
      Receive { user, category } -> do
        H.modify_ \st -> st { user = user, selectedCategory = category }
      HandleOptionsMessage (OptionsPanel.SelectedOption option) -> do
        H.modify_ \st -> st { search = "" }
        navigate $ Commands option
      HandleSearch v -> do
        navigate $ Commands 0
        H.modify_ \st -> st { search = v }
    commandToListEntry :: Command -> ListEntry
    commandToListEntry c = { name: c.name, description: c.description, details: c.details }
    render :: State -> H.ComponentHTML Action ChildSlots m
    render { user, selectedCategory, search } =
      Container.component user "Commands" $
        [ HH.slot (SProxy :: _ "optionsPanel") unit OptionsPanel.component
            { title: _.name <$> (categories !! selectedCategory)
            , options: (\c -> c.icon) <$> categories
            , selected: selectedCategory
            } (Just <<< HandleOptionsMessage)
        , HH.div [ cssClass "card" ]
            [ HH.h2_ [ HH.text "Search" ]
            , HH.input
                [ cssClass "input-field"
                , HP.placeholder "Search"
                , HP.value search
                , HE.onValueInput (Just <<< HandleSearch)
                ]
            ]
        , HH.slot (SProxy :: _ "itemsList") unit ItemsList.component
            { title: Nothing
            , entries: case selectedCategory of
                        0 | null search -> commandToListEntry <$> commands
                        0 -> commandToListEntry <$> filter (\c -> contains (Pattern search) c.name) commands
                        _ ->  commandToListEntry <$> filter (\c -> c.kind == selectedCategory) commands
            } absurd
        ]