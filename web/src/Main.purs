module Utter.Main where

import Prelude

import Data.Maybe (Maybe(..))
import Effect (Effect)
import Effect.Aff (launchAff_)
import Effect.Aff.Bus as Bus
import Effect.Ref as Ref
import Halogen as H
import Halogen.Aff as HA
import Halogen.VDom.Driver (runUI)
import Routing.Duplex (parse)
import Routing.Hash (matchesWith)
import Utter.Api.Request (BaseURL(..))
import Utter.AppM (runAppM)
import Utter.Component.Router as Router
import Utter.Data.Route (routeDuplex)
import Utter.Env (Env, UserEnv)
  
main :: Effect Unit
main = HA.runHalogenAff do
  body <- HA.awaitBody
  user <- H.liftEffect $ Ref.new Nothing -- $ Just { username: "Test", token: "TestToken" }
  userBus <- H.liftEffect Bus.make

  let
    environ :: Env
    environ = { baseUrl, userEnv }
      where
        baseUrl = BaseURL "http://localhost:8080"
        userEnv :: UserEnv
        userEnv = { user, userBus }
 
  hIO <- runUI (H.hoist (runAppM environ) Router.component) {} body

  void $ H.liftEffect $ matchesWith (parse routeDuplex) \old new ->
    when (old /= Just new) do
      launchAff_ $ hIO.query $ H.tell $ Router.Navigate new

  pure unit
