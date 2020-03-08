(ns utter.components.utterlist
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

(defn utter-list [{:keys [title entries]}]
  [style/card
   (when (some? title) [:h2 title])
   [:div
    (->
     (map-indexed #(vector list-entry {:key (%2 :id)
                                       :kind (%2 :name)
                                       :description (%2 :description)}) entries)
     (doall))]])