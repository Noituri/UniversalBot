(ns utter.components.container
  (:require
   [utter.style :as style]
   [utter.components.navbar :refer [navbar]]
   [reagent.core :as r]))

(defn container [{:keys [title]} & children]
  [:div
   [navbar]
   [style/container
    [:title title]
    children]])