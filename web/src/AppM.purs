module Utter.AppM where

import Prelude

import Control.Comonad.Env (ask)
import Control.Monad.Reader (class MonadAsk, ReaderT, asks, runReaderT)
import Effect.Aff (Aff)
import Effect.Aff.Class (class MonadAff)
import Effect.Class (class MonadEffect)
import Effect.Console as Console
import Halogen (liftEffect)
import Routing.Duplex (print)
import Routing.Hash (setHash)
import Type.Equality (class TypeEquals, from)
import Utter.Capability.Logger (class Logger)
import Utter.Capability.Navigate (class Navigate)
import Utter.Data.Route as Route
import Utter.Env (Env)

newtype AppM a = AppM (ReaderT Env Aff a)

runAppM :: Env -> AppM ~> Aff
runAppM env (AppM a) = runReaderT a env

derive newtype instance functorAppM :: Functor AppM
derive newtype instance applyAppM :: Apply AppM
derive newtype instance applicativeAppM :: Applicative AppM
derive newtype instance bindAppM :: Bind AppM
derive newtype instance monadAppM :: Monad AppM
derive newtype instance monadEffectAppM :: MonadEffect AppM
derive newtype instance monadAffAppM :: MonadAff AppM

instance monadAskAppM :: TypeEquals e Env => MonadAsk e AppM where
  ask = AppM $ asks from

instance navigateAppM :: Navigate AppM where
  navigate = liftEffect <<< setHash <<< print Route.routeDuplex

instance loggerAppM :: Logger AppM where
  log msg = liftEffect $ Console.log msg