module Utter.Component.Wrapper (component) where

import Prelude

import Control.Monad.Reader (class MonadAsk, asks)
import Data.Maybe (Maybe(..))
import Data.Symbol (SProxy(..))
import Effect.Aff.Class (class MonadAff)
import Effect.Ref as Ref
import Halogen (liftEffect)
import Halogen as H
import Halogen.HTML as HH
import Prim.Row as Row
import Record as Record
import Utter.Component.Utils (busEventSource)
import Utter.Env (UserEnv)
import Utter.Data.User (User)

data Action input output
  = Initialize
  | HandleUserBus (Maybe User)
  | Receive input
  | Emit output

type ChildSlots query output =
  ( inner :: H.Slot query output Unit )

_inner = SProxy :: SProxy "inner"

component
  :: forall query input output m r
   . MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => Row.Lacks "user" input
  => H.Component HH.HTML query { | (user :: Maybe User | input) } output m
  -> H.Component HH.HTML query { | input } output m
component innerComponent =
  H.mkComponent
    { initialState: Record.insert (SProxy :: _ "user") Nothing
    , render
    , eval: H.mkEval $ H.defaultEval
        { handleAction = handleAction
        , handleQuery = handleQuery
        , initialize = Just Initialize
        , receive = Just <<< Receive
        }
    }
  where
  handleAction = case _ of
    Initialize -> do
      { user, userBus } <- asks _.userEnv
      _ <- H.subscribe (HandleUserBus <$> busEventSource userBus)
      userData <- liftEffect $ Ref.read user
      H.modify_ _ { user = userData }

    HandleUserBus userData ->
      H.modify_ _ { user = userData }

    Receive input -> do
      { user } <- H.get
      H.put $ Record.insert (SProxy :: _ "user") user input

    Emit output ->
      H.raise output

  handleQuery :: forall a. query a -> H.HalogenM _ _ _ _ _ (Maybe a)
  handleQuery = H.query _inner unit

  render state = HH.slot _inner unit innerComponent state (Just <<< Emit)