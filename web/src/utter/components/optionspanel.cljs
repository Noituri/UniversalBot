(ns utter.components.optionspanel
  (:require
   [reagent.core :as r]
   [utter.style :as style]))

(defn option [{:keys [selected? on-click]}]
  [style/panel-option {:class (when selected? "selected") :on-click on-click}
   (into [:<>] (r/children (r/current-component)))])

(defn options-panel [{:keys [title options]}]
  [style/card
   (when (some? title) [:h2 title])
   [style/horizontal-view {:style {:padding 0} :color "#4c45a0"}
    (->
     (map-indexed #(vector option {:key %1 
                                   :selected? (%2 :selected?)
                                   :on-click (when-some [f (%2 :on-click)] f)}
                           [:h2 (%2 :name)]) options)
     (doall))]]) ; TODO: instead of number use icons
