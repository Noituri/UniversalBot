(ns utter.pages.panel
  (:require
   [utter.components.container :refer [container]]
   [utter.components.serverselector :refer [server-selector]]
   [utter.components.optionspanel :refer [options-panel]]
   [utter.components.utterlist :refer [utter-list]]
   [utter.components.serversettings :refer [server-settings]]
   [utter.style :as style]))

(defn panel-page []
  [container {:title "UtterBot - Panel"}
   [server-selector]
   [options-panel {:options
                   [{:name "1" :selected? true}
                    {:name "2" :selected? false}]}]
   [utter-list {:name "Actions"
                :entries [{:name "Ban"
                           :description "User XXX has beeen banned by YYY"}
                          {:name "Ban"
                           :description "User XXX has beeen banned by YYY"}
                          {:name "Ban"
                           :description "User XXX has beeen banned by YYY"}]}]])