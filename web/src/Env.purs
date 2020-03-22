module Utter.Env (Env, UserEnv) where

import Data.Maybe (Maybe)
import Effect.Ref (Ref)
import Effect.Aff.Bus (BusRW)
import Utter.Api.Request (BaseURL)

type Env =
    { baseUrl :: BaseURL
    , userEnv :: UserEnv
    }

type UserEnv =
    { user :: Ref (Maybe String) -- String just for now
    , userBus :: BusRW (Maybe String) -- ^^^^^^^^^
    }