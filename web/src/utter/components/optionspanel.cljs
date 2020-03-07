(ns utter.components.optionspanel
  (:require
   [reagent.core :as r]
   [utter.style :as style]))

(defn option [{:keys [selected?]}]
  [style/panel-option {:class (when selected? "selected")}
   (into [:<>] (r/children (r/current-component)))])

(defn options-panel []
  [style/card
   [style/horizontal-view {:style {:padding 0} :color "#4c45a0"}
    [option {:selected? true} [:h2 "1"]]
    [option {:selected? false} [:h2 "2"]]]]) ; TODO: instead of number use icons