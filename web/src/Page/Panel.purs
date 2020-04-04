module Utter.Page.Panel (component) where

import Prelude

import Control.Monad.Reader (class MonadAsk, lift)
import Data.Maybe (Maybe(..), isJust, isNothing)
import Data.Monoid (guard)
import Data.Symbol (SProxy(..))
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import Utter.Capability.Api (class Api, getGuilds)
import Utter.Capability.Logger (class Logger, log)
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Component.Container as Container
import Utter.Component.ItemsList as ItemsList
import Utter.Component.OptionsPanel as OptionsPanel
import Utter.Component.ServerSelector as ServerSelector
import Utter.Component.ServerSettings as ServerSettings
import Utter.Component.Utils (ChildSlot, cssClass)
import Utter.Component.Wrapper as Wrapper
import Utter.Data.Guild (Guild)
import Utter.Data.Requests (Stasus(..))
import Utter.Data.Route (Route(..))
import Utter.Data.User (User)
import Utter.Env (UserEnv)

type PageStasus =
  { guilds :: Stasus
  }

type State =
  { user :: Maybe User
  , selectedOption :: Int
  , guilds :: Array Guild
  , stasus :: PageStasus
  }

data Action
  = Initialize
  | Receive { | ( user :: Maybe User | ()) }
  | TryAgain
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
  => Api m
  => H.Component HH.HTML q {} o m
component = Wrapper.component $ H.mkComponent
  { initialState
  , render
  , eval: H.mkEval $ H.defaultEval
      { handleAction = handleAction
      , initialize = Just Initialize
      , receive = Just <<< Receive
      }
  }
  where
    initialState { user } =
      { user
      , selectedOption: 0
      , guilds: mempty
      , stasus: { guilds: Loading }
      }
    handleAction :: Action -> H.HalogenM State Action ChildSlots o m Unit
    handleAction = case _ of
      Initialize -> do
        { user } <- H.get
        case user of
          Nothing -> log "Waiting for authorization."
          Just { token } ->
            getGuilds token >>= case _ of
              Nothing -> H.modify_ \st -> st { stasus { guilds = Error "Could not retrieve your servers!" } }
              Just guilds -> H.modify_ \st -> st { guilds = guilds, stasus { guilds = Done } }
      Receive { user } -> do
        H.modify_ \st -> st { user = user }
        handleAction Initialize
      TryAgain -> do
        H.modify_ \st -> st { stasus { guilds = Loading } }
        handleAction Initialize
      HandleOptionMessage (OptionsPanel.SelectedOption option) ->
        H.modify_ \st -> st { selectedOption = option }
    render :: State -> H.ComponentHTML Action ChildSlots m
    render { user, selectedOption, guilds, stasus } =
      Container.component user "Panel" $ page
      where
        page = case stasus.guilds of
          Loading -> [ HH.h2_ [ HH.text "Loading..." ] ]
          Error err ->
            [ HH.h2_ [ HH.text err ]
            , HH.p [ cssClass "gradient-btn", HE.onClick \_ -> Just TryAgain ]
                [ HH.text "Try again!" ]
            ]
          Done -> guildsLoaded
        guildsLoaded = 
          [ HH.slot (SProxy :: _ "serverSelector") unit ServerSelector.component
              { servers: guilds
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
                        [ { name: "Ban", description: "User xxx has been banned by yyy!", details: Just "Banned for breaking 'z' rule." }
                        , { name: "Ban", description: "User xyx has been banned by yxy!", details: Just "Banned for breaking 'w' rule." }
                        ]
                    } absurd
              1 -> HH.slot (SProxy :: _ "serverSettings") unit ServerSettings.component
                    { prefix: "!"
                    , mutedRole: "12312312312312"
                    , modLogsChannel: "11112311332"
                    } absurd
              _ -> HH.text ""
          , HH.div_
              [ HH.p [ cssClass "gradient-btn red" ]
                  [ HH.text "Logout" ]
              ]
          ]
