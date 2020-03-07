(ns utter.components.serverselector
  (:require
   [utter.style :as style]))

(def tmp-logo "https://cdn.discordapp.com/embed/avatars/0.png")

(defn server-entry [{:keys [logo name selected?]}]
  [:div {:class (when selected? "selected")}
   [style/circle-img {:src logo}]
   [:p name]])

(defn server-selector []
  [style/card
   [:h2 "Server Selector"]
   [style/horizontal-view {:color "#363178"}
    [server-entry {:logo tmp-logo :name "Test Server" :selected? true}]
    [server-entry {:logo tmp-logo :name "Test Server2"}]]])