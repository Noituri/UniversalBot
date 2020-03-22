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
import Utter.Component.Wrapper as Wrapper
import Utter.Env (UserEnv)
import Web.Event.Event (preventDefault)
import Web.UIEvent.MouseEvent (MouseEvent, toEvent)

data Action = Initialize

component
  :: forall q o m r
   . MonadAff m
  => MonadAsk { userEnv :: UserEnv | r } m
  => Navigate m
  => H.Component HH.HTML q {} o m
component = Wrapper.component $ H.mkComponent
  { initialState: \x -> {}
  , render
  , eval: H.mkEval $ H.defaultEval { handleAction = handleAction }
  }
  where
    handleAction :: forall slots. Action -> H.HalogenM {} Action slots o m Unit
    handleAction = case _ of
      Initialize -> do
        H.modify_ \_ -> {}

render :: forall slots m. {} -> H.ComponentHTML Action slots m
render state =
  HH.div_
    [ HH.h1_
      [ HH.text "Welcome" ]
    ]
