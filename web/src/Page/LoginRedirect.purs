module Utter.Page.LoginRedirect where

import Prelude

import Control.Monad.Reader (class MonadAsk)
import Data.Maybe (Maybe(..), isJust, isNothing)
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Events as HE
import Utter.Capability.Api (class Api, signin)
import Utter.Capability.Logger (class Logger, log)
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Component.Container as Container
import Utter.Component.Utils (cssClass, maybeElem, whenElem)
import Utter.Component.Wrapper as Wrapper
import Utter.Data.Route (Route(..))
import Utter.Data.User (User)
import Utter.Env (UserEnv)

type Input = { code :: String }

type State =
  { user :: Maybe User
  , code :: String
  }

data Action
  = Initialize
  | Receive { user :: Maybe User, code :: String }
  | GoHome

component
  :: ∀ q o m r
   . MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => Navigate m
  => Api m
  => Logger m
  => H.Component HH.HTML q Input o m
component = Wrapper.component $ H.mkComponent
  { initialState
  , render
  , eval: H.mkEval $ H.defaultEval
      { handleAction = handleAction
      , initialize = Just Initialize
      , receive = Just <<< Receive
      }
  }
  where
    initialState { user, code } =
      { user
      , code
      }
    handleAction :: ∀ slots. Action -> H.HalogenM State Action slots o m Unit
    handleAction = case _ of
      Initialize -> do
        { code } <- H.get
        signin code >>= case _ of
          Nothing -> log "Could not signin!"
          Just user -> log "Signed in!"
      Receive { user, code } -> do
        H.modify_ \st -> st { user = user, code = code }
      GoHome -> do
        navigate Home
    render :: ∀ slots. State -> H.ComponentHTML Action slots m
    render { user, code } =
      Container.component user "Signing In" $
        [ HH.div_
            [ HH.h1 [ cssClass "heading" ]
                [ HH.text "Utter" ]
            , maybeElem user \{ username } -> HH.h1_ [ HH.text $ "Welcome " <> username <> "!" ]
            , whenElem (isNothing user) \_ -> HH.h1_ [ HH.text "Signing in..." ]
            , whenElem (isJust user) \_ -> 
                HH.p [ cssClass "gradient-btn", HE.onClick \_ -> Just GoHome  ]
                  [ HH.text "Home" ]
            ]
        ]