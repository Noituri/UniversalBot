(ns utter.components.serverselector
  (:require
   [clojure.string :refer [blank?]]
   [re-frame.core :as rf]
   [utter.style :as style]))

(defn guild-logo [id hash] (if-not (blank? hash)
                             (str "https://cdn.discordapp.com/icons/" id "/" hash ".png")
                             "https://cdn.discordapp.com/embed/avatars/0.png"))

(defn server-entry [{:keys [id logo name selected?]}]
  [:div {:class (when selected? "selected") :on-click #(rf/dispatch [:select-guild id])}
   [style/circle-img {:src logo}]
   [:p name]])

(defn server-selector [{:keys [servers selected]}]
  [style/card
   [:h2 "Server Selector"]
   [style/horizontal-view {:color "#363178"}
    (map-indexed #(vector server-entry {:key %1 :id %1 :logo (guild-logo (%2 :id) (%2 :icon)) :name (%2 :name) :selected? (= (str %1) selected)}) servers)]])