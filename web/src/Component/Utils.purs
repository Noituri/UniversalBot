module Utter.Component.Utils where

import Prelude

import Control.Monad.Rec.Class (forever)
import Data.Maybe (Maybe(..))
import Effect.Aff (forkAff, killFiber, error)
import Effect.Aff.Bus as Bus
import Effect.Aff.Class (class MonadAff)
import Halogen as H
import Halogen.HTML (ClassName(..), IProp, HTML, text)
import Halogen.HTML.Properties (class_)
import Halogen.Query.EventSource as ES
import Routing.Duplex (print)
import Utter.Data.Route (Route, routeDuplex)

type ChildSlot a = forall q. H.Slot q Void a

busEventSource :: forall m r act. MonadAff m => Bus.BusR' r act -> ES.EventSource m act
busEventSource bus =
  ES.affEventSource \emitter -> do
    fiber <- forkAff $ forever $ ES.emit emitter =<< Bus.read bus
    pure (ES.Finalizer (killFiber (error "Event source closed") fiber))

cssClass :: forall r i. String -> IProp (class :: String | r) i
cssClass name = class_ $ ClassName name

getLink :: Route -> String
getLink = append "#" <<< print routeDuplex

whenElem :: forall p i. Boolean -> (Unit -> HTML p i) -> HTML p i
whenElem cond f = if cond then f unit else text ""

maybeElem :: forall p i a. Maybe a -> (a -> HTML p i) -> HTML p i
maybeElem (Just x) f = f x
maybeElem _ _ = text ""

isMaybeTrue :: Maybe Boolean -> Boolean
isMaybeTrue Nothing = false
isMaybeTrue (Just b) = b