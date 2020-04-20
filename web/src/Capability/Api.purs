module Utter.Capability.Api where

import Prelude

import Data.Maybe (Maybe)
import Halogen (lift)
import Halogen as H
import Utter.Data.Guild (Guild)
import Utter.Data.GuildDetails (GuildDetails)
import Utter.Data.Requests (ReqGuildDetails)
import Utter.Data.User (User)

class Monad m <= Api m where
  signin :: String -> m (Maybe User)
  getGuilds :: String -> m (Maybe (Array Guild))
  getGuildDetails :: ReqGuildDetails -> m (Maybe GuildDetails)
  modifyGuild :: String -> GuildDetails -> m (Maybe {})

instance loggerHalogenM :: Api m => Api (H.HalogenM st act slots msg m) where
  signin = lift <<< signin
  getGuilds = lift <<< getGuilds
  getGuildDetails = lift <<< getGuildDetails
  modifyGuild token details = lift $ modifyGuild token details