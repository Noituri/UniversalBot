module Utter.Component.Router where

import Prelude

import Control.Monad.Reader (class MonadAsk)
import Data.Either (hush)
import Data.Maybe (Maybe(..), fromMaybe, isJust)
import Data.Symbol (SProxy(..))
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import Routing.Hash (getHash)
import Routing.PushState (matches)
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Component.Utils (ChildSlot)
import Utter.Component.Wrapper as Wrapper
import Utter.Data.Route (Route(..), routeDuplex)
import Utter.Env (UserEnv)
import Routing.Duplex as RD
import Utter.Page.Home as Home

type State =
  { route :: Maybe Route
  , user :: Maybe String
  }

data Query a = Navigate Route a

data Action
  = Initialize
  | Receive { | ( user :: Maybe String | ()) }


type ChildSlots =
  ( home :: ChildSlot Unit
  )

component
  :: forall m r
   . MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => Navigate m
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
      navigate Home

    Receive { user } ->
      H.modify_ _ { user = user }
  handleQuery :: forall a. Query a -> H.HalogenM State Action ChildSlots Void m (Maybe a)
  handleQuery = case _ of
    Navigate dest a -> do
      { route, user } <- H.get
      when (route /= Just dest) do
        case (isJust user) of -- TODO: && dest `elem` [ Redirect ]
          false -> H.modify_ _ { route = Just dest }
          _ -> pure unit
      pure (Just a)


  render :: State -> H.ComponentHTML Action ChildSlots m
  render { route, user } = case route of
    Just r -> case r of
      Home ->
        HH.slot (SProxy :: _ "home") unit Home.component {} absurd
    Nothing ->
      HH.div_ [ HH.text "Page not found!" ]