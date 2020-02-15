(ns utter.pages.homepage
  (:require
    [utter.style :as style]
    [reagent.core :as r]))

(defn home-page []
      [:div {:class (style/content)}
        [:title "Utter - Home"] 
        [style/h1 "Utter - Univeral Bot" [:sup "(W.I.P)"]]])
