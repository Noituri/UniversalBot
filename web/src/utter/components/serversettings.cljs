(ns utter.components.serversettings
  (:require
   [reagent.core :as r]
   [utter.style :as style]))

(defn server-settings [{:keys [prefix muted_role mod_logs_channel]}]
  [style/card
   [:h2 "Settings"]
   [style/settings-container
    [:h4 "Prefix"]
    [style/input-field {:placeholder "Bot Prefix" :value prefix :on-change #(swap! prefix (-> % .-target .-value))}]
    [:h4 "Muted Role"]
    [style/input-field {:placeholder "Muted Role ID" :value muted_role :on-change #(swap! muted_role (-> % .-target .-value))}]
    [:h4 "Mod-logs Channel"]
    [style/input-field {:placeholder "Mod-logs Channel ID" :value mod_logs_channel :on-change #(swap! mod_logs_channel (-> % .-target .-value))}]
    [style/gradient-btn {:style {:marginTop "20px"}} "Save"]]])