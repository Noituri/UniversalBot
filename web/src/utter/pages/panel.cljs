(ns utter.pages.panel
  (:require
   [kee-frame.core :as k]
   [re-frame.core :as rf]
   [utter.components.container :refer [container]]
   [utter.components.serverselector :refer [server-selector]]
   [utter.components.optionspanel :refer [options-panel]]
   [utter.components.utterlist :refer [utter-list]]
   [utter.components.serversettings :refer [server-settings]]
   [reagent.core :as r]
   [utter.style :as style]))

(defn actions-list []
   [utter-list {:name "Actions"
                :entries [{:id 2
                           :name "Ban"
                           :description "User XXX has beeen banned by YYY"}
                          {:id 1
                           :name "Ban"
                           :description "User XXX has beeen banned by YYY"}
                          {:id 0
                           :name "Ban"
                           :description "User XXX has beeen banned by YYY"}]}])

(defn panel-page []
  (let [selected-option (r/atom 0)]
    (fn []
      [container {:title "UtterBot - Panel"}
       [server-selector]
       [options-panel {:options
                       [{:icon "fa-newspaper"
                         :selected? (= @selected-option 0)
                         :on-click #(reset! selected-option 0)}
                        {:icon "fa-wrench"
                         :selected? (= @selected-option 1)
                         :on-click #(reset! selected-option 1)}]}]
       (case @selected-option
         0 [actions-list]
         1 [server-settings])
       [style/gradient-btn "Logout"]])))