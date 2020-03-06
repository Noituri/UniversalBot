(ns utter.components.featurecard
  (:require
   [utter.style :as style]
   [reagent.core :as r]))

(defn featurecard [{:keys [title description]}]
  [style/card
   [:h3 title]
   [:h5 description]])