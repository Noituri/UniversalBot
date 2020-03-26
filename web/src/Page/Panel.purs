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
import Utter.Capability.Logger (class Logger)
import Utter.Capability.Navigate (class Navigate, navigate)
import Utter.Component.Container as Container
import Utter.Component.FeatureCard as FeatureCard
import Utter.Component.OptionsPanel as OptionsPanel
import Utter.Component.ServerSelector as ServerSelector
import Utter.Component.Utils (ChildSlot, cssClass)
import Utter.Component.Wrapper as Wrapper
import Utter.Data.Route (Route)
import Utter.Data.Server (Server)
import Utter.Data.User (User)
import Utter.Env (UserEnv)

type State = Maybe User

data Action = Receive { | ( user :: Maybe User | ()) }

type ChildSlots =
  ( serverSelector :: ChildSlot Unit
  )

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
    handleAction :: Action -> H.HalogenM State Action ChildSlots o m Unit
    handleAction = case _ of
      Receive { user } ->
        H.modify_ \_ -> user
-- HH.slot (SProxy :: _ "home") unit Home.component {} absurd
    render :: State -> H.ComponentHTML Action ChildSlots m
    render state =
      Container.component state "Panel" $
        [ HH.slot (SProxy :: _ "serverSelector") unit ServerSelector.component
            { servers: [ { id: "1", icon: "", name: "Test1" }
                      , { id: "2", icon: "", name: "Test2" }
                      ]
            , selected: 0
            } absurd
          -- ServerSelector.component 
            -- [ { id: "1", icon: "", name: "Test1" }
            -- , { id: "2", icon: "", name: "Test2" }
            -- ] 0 TODO: use HH.slots because it's a component with state now
        --  OptionsPanel.component Nothing [ "fa-newspaper", "fa-wrench" ] 0
        ]
