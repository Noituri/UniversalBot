module Utter.Env (Env, UserEnv) where

import Data.Maybe (Maybe)
import Effect.Ref (Ref)
import Effect.Aff.Bus (BusRW)
import Utter.Api.Request (BaseURL)
import Utter.Data.User (User)

type Env =
    { baseUrl :: BaseURL
    , userEnv :: UserEnv
    }

type UserEnv =
    { user :: Ref (Maybe User)
    , userBus :: BusRW (Maybe User)
    }