(ns utter.components.actionslist
  (:require
   [reagent.core :as r]
   [utter.style :as style]))

(defn list-entry [{:keys [kind description]}]
  (let [show-details? (r/atom false)]
    (fn []
      [style/list-entry {:on-click #(swap! show-details? not)}
       [:div.list-text
        [:h3 kind]
        [:h5 description]]
       (when @show-details?
         [style/list-entry {:bg-color :dark :style {:marginTop "15px"}}
          [:div.list-text
           [:h5 "Creation Date: 00:00 00-00-000"]]])])))

(defn actions-list []
  [style/card
   [:h2 "Actions"]
   [:div
    [list-entry {:kind "Ban" :description "User test has been banned by admin!"}]
    [list-entry {:kind "Ban" :description "User test has been banned by admin!"}]
    [list-entry {:kind "Ban" :description "User test has been banned by admin!"}]
    [list-entry {:kind "Ban" :description "User test has been banned by admin!"}]]])