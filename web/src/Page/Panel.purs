module Utter.Page.Panel (component) where

import Prelude

import Control.Monad.Reader (class MonadAsk, lift)
import Data.Array ((!!))
import Data.Maybe (Maybe(..), isJust, isNothing)
import Data.Monoid (guard)
import Data.Symbol (SProxy(..))
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import Utter.Capability.Api (class Api, getGuildDetails, getGuilds)
import Utter.Capability.Logger (class Logger, log)
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Component.Container as Container
import Utter.Component.ItemsList as ItemsList
import Utter.Component.OptionsPanel as OptionsPanel
import Utter.Component.ServerSelector as ServerSelector
import Utter.Component.ServerSettings as ServerSettings
import Utter.Component.Utils (ChildSlot, cssClass, maybeElem, whenElem)
import Utter.Component.Wrapper as Wrapper
import Utter.Data.Action (GuildAction)
import Utter.Data.Guild (Guild)
import Utter.Data.GuildDetails (GuildDetails)
import Utter.Data.ListEntry (ListEntry)
import Utter.Data.Requests (Stasus(..))
import Utter.Data.Route (Route(..))
import Utter.Data.User (User)
import Utter.Env (UserEnv)

type Input = { selectedGuild :: Int }

type PageStasus =
  { guilds :: Stasus
  }

type State =
  { user :: Maybe User
  , selectedOption :: Int
  , guilds :: Array Guild
  , stasus :: PageStasus
  , selectedGuild :: Int
  , guildDetails :: Maybe GuildDetails
  }

data Action
  = Initialize
  | Receive { user :: Maybe User, selectedGuild :: Int }
  | TryAgain
  | GetDetails String
  | HandleOptionMessage OptionsPanel.Message
  | HandleServerMessage ServerSelector.Message

type ChildSlots =
  ( serverSelector :: ServerSelector.Slot Unit
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
  => H.Component HH.HTML q Input o m
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
    initialState { user, selectedGuild } =
      { user
      , selectedOption: 0
      , guilds: mempty
      , stasus: { guilds: Loading }
      , selectedGuild
      , guildDetails: Nothing
      }
    handleAction :: Action -> H.HalogenM State Action ChildSlots o m Unit
    handleAction = case _ of
      Initialize -> do
        { user } <- H.get
        case user of
          Nothing -> log "Waiting for authorization."
          Just { token } ->
            getGuilds token >>= case _ of
              Nothing ->
                H.modify_ \st -> st { stasus { guilds = Error "Could not retrieve your servers!" } }
              Just guilds -> do
                H.modify_ \st -> st { guilds = guilds, stasus { guilds = Done } }
                handleAction $ GetDetails token
      Receive { user, selectedGuild } -> do
        H.modify_ \st -> st { user = user, selectedGuild = selectedGuild }
        handleAction Initialize
      TryAgain -> do
        H.modify_ \st -> st { stasus { guilds = Loading } }
        handleAction Initialize
      GetDetails token -> do
        { guilds, selectedGuild } <- H.get
        case guilds !! selectedGuild of
          Nothing ->
            H.modify_ \st -> st { stasus { guilds = Error "Server does not exist!" } }
          Just { id } ->
            getGuildDetails { token, guild_id: id, actions_from: 0  } >>= case _ of
              Nothing ->
                H.modify_ \st -> st { stasus { guilds = Error "Could not retrieve guild details!" } }
              Just details ->
                H.modify_ \st -> st { guildDetails = Just details }
      HandleOptionMessage (OptionsPanel.SelectedOption option) ->
        H.modify_ \st -> st { selectedOption = option }
      HandleServerMessage (ServerSelector.SelectedServer server) -> do
        H.modify_ \st -> st { selectedGuild = server }
        navigate $ EditPanel server
    render :: State -> H.ComponentHTML Action ChildSlots m
    render st@{ user, selectedOption, selectedGuild, guilds, stasus } =
      Container.component user "Panel" $ page
      where
        logoutBtn =
          HH.div_
            [ HH.p [ cssClass "gradient-btn red" ]
                [ HH.text "Logout" ]
            ]
        page = case stasus.guilds of
          Loading -> [ HH.h2_ [ HH.text "Loading..." ]
                     , logoutBtn
                     ]
          Error err ->
            [ HH.h2_ [ HH.text err ]
            , HH.p [ cssClass "gradient-btn", HE.onClick \_ -> Just TryAgain ]
                [ HH.text "Try again!" ]
            , logoutBtn
            ]
          Done -> guildsLoaded
        guildPanel =
          [ HH.slot (SProxy :: _ "optionsPanel") unit OptionsPanel.component
              { title: Nothing
              , options: [ "fa-newspaper", "fa-wrench" ]
              , selected: selectedOption
              } (Just <<< HandleOptionMessage)
          , case selectedOption of
              0 -> HH.slot (SProxy :: _ "itemsList") unit ItemsList.component
                    { title: Just "Actions"
                    , entries: case st.guildDetails of
                        Just g ->
                          actionsToItemsEntry g.actions
                        Nothing ->
                          []
                    } absurd
              1 -> HH.slot (SProxy :: _ "serverSettings") unit ServerSettings.component
                    { prefix: "!"
                    , mutedRole: "12312312312312"
                    , modLogsChannel: "11112311332"
                    } absurd
              _ -> HH.text ""
          ]
        guildsLoaded = 
          [ HH.slot (SProxy :: _ "serverSelector") unit ServerSelector.component
              { servers: guilds
              , selected: selectedGuild
              } (Just <<< HandleServerMessage)
          , maybeElem (guilds !! selectedGuild) \g ->
              if g.access then
                HH.div [ cssClass "guild-panel-container" ] guildPanel
              else
                HH.h2_ [ HH.text "You don't have access to this server." ]
          , whenElem (isNothing $ guilds !! selectedGuild) \_ ->
              HH.h2_ [ HH.text "Bot does not exist." ]
          , logoutBtn
          ]

actionsToItemsEntry :: Array GuildAction -> Array ListEntry
actionsToItemsEntry actions = (\a -> { name: "GET TYPE!!!", description: a.message, details: Just "CREATE DETAILS!!!" }) <$> actions