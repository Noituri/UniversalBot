(ns utter.components.actionslist
  (:require
   [utter.style :as style]))

(defn list-entry [{:keys [kind description]}]
  [style/list-entry
   [:div.list-text
    [:h3 kind]
    [:h5 description]]])

(defn actions-list []
  [style/card
   [:h2 "Actions"]
   [:div
    [list-entry {:kind "Ban" :description "User test has been banned by admin!"}]
    [list-entry {:kind "Ban" :description "User test has been banned by admin!"}]
    [list-entry {:kind "Ban" :description "User test has been banned by admin!"}]
    [list-entry {:kind "Ban" :description "User test has been banned by admin!"}]]])