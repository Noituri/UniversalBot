module Utter.Component.Navbar (component) where

import Prelude

import Data.Maybe (Maybe, isJust, isNothing)
import Halogen.HTML as HH
import Halogen.HTML.Properties as HP
import Utter.Component.Utils (cssClass, getLink, whenElem)
import Utter.Data.Route (Route(..))
import Utter.Data.User (User)

component :: forall i p. Maybe User -> HH.HTML i p
component user =
  HH.nav_
    [ logo "UtterBot"
    , HH.div [ cssClass "nav-items-container" ]
      [ item "Invite" $ getLink Home
      , item "Commands" $ getLink $ Commands 0
      , whenElem (isJust user) \_ ->
          item "Panel" $ getLink Panel
      , whenElem (isNothing user) \_ ->
          item "Login" "https://discordapp.com/api/oauth2/authorize?client_id=535183435539677195&redirect_uri=http%3A%2F%2Flocalhost%3A8081%2F%23%2Fredirect%2F&response_type=code&scope=identify%20email%20guilds"
      ]
    ]
    where
      logo :: String -> HH.HTML i p
      logo name = HH.a [ cssClass "nav-item nav-logo", HP.href $ getLink Home ]
                    [ HH.text name ]
      item :: String -> String -> HH.HTML i p
      item name path = HH.a [ cssClass "nav-item", HP.href path ]
                        [ HH.text name ]