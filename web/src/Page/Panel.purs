module Utter.Page.Panel (component) where

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
import Utter.Component.OptionsPanel as OptionsPanel
import Utter.Component.ServerSelector as ServerSelector
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

render :: forall slots m. State -> H.ComponentHTML Action slots m
render state =
  Container.component state "Panel" $
    [ ServerSelector.component 
        [ { id: "1", icon: "", name: "Test1" }
        , { id: "2", icon: "", name: "Test2" }
        ] 0
    , OptionsPanel.component Nothing [ "fa-newspaper", "fa-wrench" ] 0
    ]
