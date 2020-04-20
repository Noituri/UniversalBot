module Utter.Api.Utils where

import Prelude

import Control.Apply (lift2)
import Control.Monad.Reader (class MonadAsk, ask, lift)
import Data.Argonaut.Core (Json)
import Data.Argonaut.Decode.Struct.Tolerant as Tolerant
import Data.Either (Either(..))
import Data.Maybe (Maybe(..))
import Effect (Effect)
import Effect.Aff.Bus as Bus
import Effect.Aff.Class (class MonadAff, liftAff)
import Effect.Class (class MonadEffect, liftEffect)
import Effect.Ref as Ref
import Utter.Capability.Logger (class Logger, log)
import Utter.Data.Guild (Guild)
import Utter.Data.User (User)
import Utter.Env (UserEnv)
import Web.HTML (window)
import Web.HTML.Window (localStorage)
import Web.Storage.Storage (getItem, setItem, removeItem)

writeUser :: User -> Effect Unit
writeUser { token, username } = do
    setItem "token" token =<< localStorage =<< window
    setItem "username" username =<< localStorage =<< window

readUser :: Effect (Maybe User)
readUser = do
  token <- getItem "token" =<< localStorage =<< window
  username <- getItem "username" =<< localStorage =<< window
  pure $ lift2 (\u t -> { username: u, token: t }) username token

logoutUser :: Effect Unit
logoutUser = do
  removeItem "token" =<< localStorage =<< window
  removeItem "username" =<< localStorage =<< window

validateUser
  :: ∀ m r
   . Logger m
  => MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => m (Either String User)
  -> m (Maybe User)
validateUser response = do
  { userEnv } <- ask
  response >>= case _ of
    Left err -> log err *> pure Nothing
    Right user -> do
      liftEffect do
        writeUser user
        Ref.write (Just user) userEnv.user
      liftAff $ Bus.write (Just user) userEnv.userBus
      pure $ Just user

validateRequest
  :: ∀ m r a
   . Logger m
  => MonadAff m
  => m (Either String a)
  -> m (Maybe a)
validateRequest response = do
  response >>= case _ of
    Left err -> log err *> pure Nothing
    Right guilds -> do
      pure $ Just guilds