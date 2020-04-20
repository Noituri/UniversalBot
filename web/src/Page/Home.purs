module Utter.Page.Home (component) where

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
import Utter.Capability.Navigate (class Navigate)
import Utter.Component.Container as Container
import Utter.Component.FeatureCard as FeatureCard
import Utter.Component.Utils (cssClass)
import Utter.Component.Wrapper as Wrapper
import Utter.Data.User (User)
import Utter.Env (UserEnv)

type State = Maybe User

data Action = Receive { user :: Maybe User }

component
  :: forall q o m r
   . MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => Navigate m
  => H.Component HH.HTML q {} o m
component = Wrapper.component $ H.mkComponent
  { initialState
  , render
  , eval: H.mkEval $ H.defaultEval
      { handleAction = handleAction
      , receive = Just <<< Receive
      }
  }
  where
    initialState { user } = user
    handleAction :: forall slots. Action -> H.HalogenM State Action slots o m Unit
    handleAction = case _ of
      Receive { user } -> do
        H.modify_ \_ -> user

render :: forall slots m. State -> H.ComponentHTML Action slots m
render state =
  Container.component state "Home" $
    [ HH.div_
        [ HH.h1 [ cssClass "heading" ]
          [ HH.text "Utter" ]
        , HH.h1_
          [ HH.text "The Universal Bot" ]
        , HH.p [ cssClass "gradient-btn" ]
          [ HH.text "Try it!" ]
        ]
    , HH.div [ cssClass "top-margin" ]
        [ HH.h2_
            [ HH.text "Features" ]
        , HH.div [ cssClass "features" ]
            [ FeatureCard.component
                "Powerful"
                "Need moderation, utilities or tickets? We've got you covered! UtterBot offers many commands categorised into modules."
            , FeatureCard.component
                "Configurable"
                "Don't need some commands? Disable them!\nNeed only ticket commands? Just enable ticket module!"
            , FeatureCard.component
                "Web Panel"
                "Configure UtterBot from your web browser. Check the moderation logs!"
            , FeatureCard.component
                "Open Source"
                "Want to check the code out? Or contribute to the project? Everything is open-source. Feel free to dive in to the project!"
            ]
        ]
    ]
