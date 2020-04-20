module Utter.Page.NotFound (component) where

import Prelude

import Control.Monad.Reader (class MonadAsk)
import Data.Maybe (Maybe(..), isJust, isNothing)
import Data.Monoid (guard)
import Data.Symbol (SProxy(..))
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Capability.Logger (class Logger, log)
import Utter.Component.Container as Container
import Utter.Component.FeatureCard as FeatureCard
import Utter.Component.Utils (cssClass)
import Utter.Component.Wrapper as Wrapper
import Utter.Data.Route (Route(..))
import Utter.Data.User (User)
import Utter.Env (UserEnv)

type State = Maybe User

data Action
  = Receive { user :: Maybe User }
  | GoBack

component
  :: forall q o m r
   . MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => Navigate m
  => Logger m
  => H.Component HH.HTML q {} o m
component = Wrapper.component $ H.mkComponent
  { initialState
  , render
  , eval: H.mkEval $ H.defaultEval { handleAction = handleAction
                                   , receive = Just <<< Receive
                                   }
  }
  where
    initialState { user } = user
    handleAction :: forall slots. Action -> H.HalogenM State Action slots o m Unit
    handleAction = case _ of
      Receive { user } -> do
        H.modify_ \_ -> user
      GoBack -> do
        navigate Home

render :: forall slots m. State -> H.ComponentHTML Action slots m
render state =
  Container.component state "Not Found" $
    [ HH.div_
      [ HH.h1 [ cssClass "top-margin" ]
          [ HH.text "Not sure what you were looking for. But it's not here :/" ]    
      , HH.p [ cssClass "gradient-btn", HE.onClick \_ -> Just GoBack ]
          [ HH.text "Take me back!" ]
      ]
    ]
