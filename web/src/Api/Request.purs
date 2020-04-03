module Utter.Api.Request (BaseURL(..), exchangeCodeReq, getGuildsReq, getGuildDetailsReq) where

import Prelude

import Affjax (Request, printError, request)
import Affjax.RequestBody as RB
import Affjax.RequestHeader (RequestHeader(..))
import Affjax.ResponseFormat as RF
import Data.Argonaut.Core (Json)
import Data.Argonaut.Decode.Struct.Tolerant as Tolerant
import Data.Argonaut.Decode.Struct.Tolerant ((.::))
import Data.Argonaut.Encode (encodeJson)
import Data.Bifunctor (bimap)
import Data.Either (Either(..))
import Data.HTTP.Method (Method(..))
import Data.Maybe (Maybe(..))
import Data.Tuple (Tuple(..))
import Effect.Aff.Class (class MonadAff, liftAff)
import Routing.Duplex (print)
import Utter.Api.Endpoint (Endpoint(..), endpointUrl)
import Utter.Capability.Logger (class Logger, log)
import Utter.Data.User (User)
import Utter.Data.Guild (Guild)
import Utter.Data.GuildDetails (GuildDetails)
import Utter.Data.Requests (ReqGuildDetails)

newtype BaseURL = BaseURL String

data RequestMethod
  = Get
  | Post (Maybe Json)

type RequestOptions =
  { endpoint :: Endpoint
  , method :: RequestMethod
  }

defaultRequest :: RequestOptions -> Request Json
defaultRequest { endpoint, method } =
  { method: Left method
  , url: endpointUrl endpoint
  , headers: []
  , content: RB.json <$> body
  , username: Nothing
  , password: Nothing
  , withCredentials: false
  , responseFormat: RF.json
  }
  where
  Tuple method body = case method of
    Get -> Tuple GET Nothing
    Post b -> Tuple POST b

exchangeCodeReq :: ∀ m. Logger m => MonadAff m => String -> m (Either String User)
exchangeCodeReq code = do
  res <- liftAff $ request $ defaultRequest { endpoint: ExchangeCode, method: Post $ Just $ encodeJson { code } }
  pure $ Tolerant.decodeJson =<< bimap printError _.body res

getGuildsReq :: ∀ m. Logger m => MonadAff m => String -> m (Either String (Array Guild))
getGuildsReq token = do
  res <- liftAff $ request $ defaultRequest { endpoint: Guilds, method: Post $ Just $ encodeJson { token } }
  pure $ Tolerant.decodeJson =<< decodeAt "guilds" =<< bimap printError _.body res

getGuildDetailsReq :: ∀ m. Logger m => MonadAff m => ReqGuildDetails -> m (Either String GuildDetails)
getGuildDetailsReq { token, guild_id, actions_from } = do
  res <- liftAff $ request $ defaultRequest
    { endpoint: Guilds
    , method: Post $ Just $ encodeJson { token, guild_id, actions_from }
    }
  pure $ Tolerant.decodeJson =<< bimap printError _.body res

decodeAt :: ∀ a. Tolerant.DecodeJson a => String -> Json -> Either String a
decodeAt key = (_ .:: key) <=< Tolerant.decodeJson