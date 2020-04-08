module Utter.Component.Router where

import Prelude

import Control.Monad.Reader (class MonadAsk)
import Data.Either (hush)
import Data.String (null)
import Data.Maybe (Maybe(..), fromMaybe, isJust)
import Data.Symbol (SProxy(..))
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML as HH
import Routing.Duplex as RD
import Routing.Hash (getHash)
import Utter.Capability.Logger (class Logger)
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Capability.Api (class Api)
import Utter.Component.Utils (ChildSlot)
import Utter.Component.Wrapper as Wrapper
import Utter.Data.Route (Route(..), routeDuplex)
import Utter.Data.User (User)
import Utter.Env (UserEnv)
import Utter.Page.Home as Home
import Utter.Page.NotFound as NotFound
import Utter.Page.Panel as Panel
import Utter.Page.Commands as Commands
import Utter.Page.LoginRedirect as LoginRedirect

type State =
  { route :: Maybe Route
  , user :: Maybe User
  }

data Query a = Navigate Route a

data Action
  = Initialize
  | Receive { | ( user :: Maybe User | ()) }


type ChildSlots =
  ( home :: ChildSlot Unit
  , panel :: ChildSlot Unit
  , commands :: ChildSlot Unit
  , notFound :: ChildSlot Unit
  , loginRedirect :: ChildSlot Unit
  )

component
  :: forall m r
   . MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => Navigate m
  => Logger m
  => Api m
  => H.Component HH.HTML Query {} Void m
component = Wrapper.component $ H.mkComponent
  { initialState: \{ user } -> { route: Nothing, user }
  , render
  , eval: H.mkEval $ H.defaultEval
      { handleQuery = handleQuery
      , handleAction = handleAction
      , receive = Just <<< Receive
      , initialize = Just Initialize
      }
  }
  where
    handleAction :: Action -> H.HalogenM State Action ChildSlots Void m Unit
    handleAction = case _ of
      Initialize -> do
        hashRoute <- H.lift $ H.liftEffect getHash
        if null hashRoute
        then navigate Home
        else navigate $ fromMaybe NotFound (hush $ RD.parse routeDuplex hashRoute)
      Receive { user } ->
        H.modify_ _ { user = user }
    handleQuery :: forall a. Query a -> H.HalogenM State Action ChildSlots Void m (Maybe a)
    handleQuery = case _ of
      Navigate dest a -> do
        { route, user } <- H.get
        when (route /= Just dest) do
          case (isJust user && false) of -- TODO: && dest `elem` [ Redirect ]
            false -> H.modify_ _ { route = Just dest }
            _ -> pure unit
        pure (Just a)

    authorize :: Maybe User -> H.ComponentHTML Action ChildSlots m -> H.ComponentHTML Action ChildSlots m
    authorize user html = case user of
      Nothing ->
        HH.slot (SProxy :: _ "home") unit Home.component {} absurd -- TODO: Redirect to discord login
      Just _ ->
        html

    render :: State -> H.ComponentHTML Action ChildSlots m
    render { route, user } = case route of
      Just r -> case r of
        Home ->
          HH.slot (SProxy :: _ "home") unit Home.component {} absurd
        Panel ->
          HH.slot (SProxy :: _ "panel") unit Panel.component { selectedGuild: 0 } absurd
            # authorize user
        EditPanel guild ->
          HH.slot (SProxy :: _ "panel") unit Panel.component { selectedGuild: guild } absurd
            # authorize user
        Commands category ->
          HH.slot (SProxy :: _ "commands") unit Commands.component { category } absurd
        Redirect code ->
          HH.slot (SProxy :: _ "loginRedirect") unit LoginRedirect.component { code } absurd
        NotFound ->
          HH.slot (SProxy :: _ "notFound") unit NotFound.component {} absurd
      Nothing ->
        HH.div_ [ HH.text "Page not found!" ]