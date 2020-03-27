module Utter.Page.Panel (component) where

import Prelude

import Control.Monad.Reader (class MonadAsk)
import Data.Maybe (Maybe(..), isJust, isNothing)
import Data.Monoid (guard)
import Data.Symbol (SProxy(..))
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import Utter.Capability.Logger (class Logger, log)
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Component.Container as Container
import Utter.Component.ItemsList as ItemsList
import Utter.Component.OptionsPanel as OptionsPanel
import Utter.Component.ServerSelector as ServerSelector
import Utter.Component.ServerSettings as ServerSettings
import Utter.Component.Utils (ChildSlot, cssClass)
import Utter.Component.Wrapper as Wrapper
import Utter.Data.User (User)
import Utter.Env (UserEnv)

type State =
  { user:: Maybe User
  , selectedOption :: Int
  }

data Action
  = Receive { | ( user :: Maybe User | ()) }
  | HandleOptionMessage OptionsPanel.Message

type ChildSlots =
  ( serverSelector :: ChildSlot Unit
  , optionsPanel :: OptionsPanel.Slot Unit
  , itemsList :: ChildSlot Unit
  , serverSettings :: ChildSlot Unit
  )

component
  :: ∀ q o m r
   . MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => Navigate m
  => Logger m
  => H.Component HH.HTML q {} o m
component = Wrapper.component $ H.mkComponent
  { initialState
  , render
  , eval: H.mkEval $ H.defaultEval
      { handleAction = handleAction
      , receive = Just <<< Receive
      }
  }
  where
    initialState { user } =
      { user
      , selectedOption: 0
      }
    handleAction :: Action -> H.HalogenM State Action ChildSlots o m Unit
    handleAction = case _ of
      Receive { user } ->
        H.modify_ \st -> st { user = user }
      HandleOptionMessage (OptionsPanel.SelectedOption option) ->
        -- log ("Selected: " <> show option)
        H.modify_ \st -> st { selectedOption = option }
    render :: State -> H.ComponentHTML Action ChildSlots m
    render { user, selectedOption } =
      Container.component user "Panel" $
        [ HH.slot (SProxy :: _ "serverSelector") unit ServerSelector.component
            { servers: [ { id: "1", icon: "", name: "Test1" }
                       , { id: "2", icon: "", name: "Test2" }
                       ]
            , selected: 0
            } absurd
        , HH.slot (SProxy :: _ "optionsPanel") unit OptionsPanel.component
            { title: Nothing
            , options: [ "fa-newspaper", "fa-wrench" ]
            , selected: selectedOption
            } (Just <<< HandleOptionMessage)
        , case selectedOption of
            0 -> HH.slot (SProxy :: _ "itemsList") unit ItemsList.component
                  { title: Just "Actions"
                  , entries:
                      [ { name: "Ban", description: "User xxx has been banned by yyy!", details: "Banned for breaking 'z' rule." }
                      , { name: "Ban", description: "User xyx has been banned by yxy!", details: "Banned for breaking 'w' rule." }
                      ]
                  } absurd
            1 -> HH.slot (SProxy :: _ "serverSettings") unit ServerSettings.component
                  { prefix: "!"
                  , mutedRole: "12312312312312"
                  , modLogsChannel: "11112311332"
                  } absurd
            _ -> HH.text ""
        ]
