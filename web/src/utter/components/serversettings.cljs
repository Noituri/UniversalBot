(ns utter.components.serversettings
  (:require
   [utter.style :as style]))

(defn server-settings []
  [style/card
   [:h2 "Settings"]
   [style/settings-container
    [:h4 "Prefix"]
    [style/input-field {:placeholder "Bot Prefix" :defaultValue "."}]
    [:h4 "Muted Role"]
    [style/input-field {:placeholder "Muted Role ID"}]
    [:h4 "Mod-logs Channel"]
    [style/input-field {:placeholder "Mod-logs Channel ID"}]
    [style/gradient-btn {:style {:marginTop "20px"}} "Save"]]])