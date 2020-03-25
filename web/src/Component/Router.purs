module Utter.Component.Router where

import Prelude

import Control.Monad.Reader (class MonadAsk)
import Data.Either (hush)
import Data.Maybe (Maybe(..), fromMaybe, isJust)
import Data.Symbol (SProxy(..))
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML as HH
import Routing.Hash (getHash)
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Capability.Logger (class Logger)
import Utter.Component.Utils (ChildSlot)
import Utter.Component.Wrapper as Wrapper
import Utter.Data.Route (Route(..), routeDuplex)
import Utter.Data.User (User)
import Utter.Env (UserEnv)
import Routing.Duplex as RD
import Utter.Page.Home as Home
import Utter.Page.Panel as Panel
import Utter.Page.NotFound as NotFound

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
  , notFound :: ChildSlot Unit
  )

component
  :: forall m r
   . MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => Navigate m
  => Logger m
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
      initialRoute <- hush <<< (RD.parse routeDuplex) <$> H.liftEffect getHash
      navigate $ fromMaybe NotFound initialRoute

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
        HH.slot (SProxy :: _ "panel") unit Panel.component {} absurd
          # authorize user
      EditPanel guild ->
        HH.slot (SProxy :: _ "panel") unit Panel.component {} absurd
          # authorize user
      NotFound ->
        HH.slot (SProxy :: _ "notFound") unit NotFound.component {} absurd
    Nothing ->
      HH.div_ [ HH.text "Page not found!" ]