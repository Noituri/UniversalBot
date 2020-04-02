module Utter.Api.Utils where

import Prelude

import Control.Monad.Reader (class MonadAsk, ask)
import Data.Either (Either(..))
import Data.Maybe (Maybe(..))
import Effect (Effect)
import Effect.Aff.Bus as Bus
import Effect.Aff.Class (class MonadAff, liftAff)
import Effect.Class (class MonadEffect, liftEffect)
import Effect.Ref as Ref
import Utter.Capability.Logger (class Logger, log)
import Utter.Data.User (User)
import Utter.Env (UserEnv)
import Web.HTML (window)
import Web.HTML.Window (localStorage)
import Web.Storage.Storage (setItem)
import Data.Argonaut.Core (Json)
import Data.Argonaut.Decode.Struct.Tolerant as Tolerant

writeUser :: User -> Effect Unit
writeUser { token, username } = do
    setItem "token" token =<< localStorage =<< window
    setItem "username" username =<< localStorage =<< window

decodeUser
  :: ∀ m r
   . Logger m
  => MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => m (Either String User)
  -> m (Maybe User)
decodeUser response = do
  { userEnv } <- ask
  response >>= case _ of
    Left err -> log err *> pure Nothing
    Right user -> do
      liftEffect do
        writeUser user
        Ref.write (Just user) userEnv.user
      liftAff $ Bus.write (Just user) userEnv.userBus
      pure $ Just user