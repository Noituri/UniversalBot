(ns utter.pages.panel
  (:require
   [utter.components.container :refer [container]]
   [utter.components.serverselector :refer [server-selector]]
   [utter.components.optionspanel :refer [options-panel]]
   [utter.components.actionslist :refer [actions-list]]
   [utter.components.serversettings :refer [server-settings]]
   [utter.style :as style]))

(defn panel []
  [container {:title "UtterBot - Panel"}
   [server-selector]
   [options-panel]
   [actions-list]])