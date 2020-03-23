module Utter.Component.Utils where

import Prelude

import Control.Monad.Rec.Class (forever)
import Effect.Aff (forkAff, killFiber, error)
import Effect.Aff.Bus as Bus
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML (ClassName(..), IProp)
import Halogen.HTML.Properties (class_)
import Halogen.Query.EventSource as ES

type ChildSlot a = forall q. H.Slot q Void a

busEventSource :: forall m r act. MonadAff m => Bus.BusR' r act -> ES.EventSource m act
busEventSource bus =
  ES.affEventSource \emitter -> do
    fiber <- forkAff $ forever $ ES.emit emitter =<< Bus.read bus
    pure (ES.Finalizer (killFiber (error "Event source closed") fiber))

cssClass :: forall r i. String -> IProp (class :: String | r) i
cssClass name = class_ $ ClassName name