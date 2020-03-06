(ns utter.pages.homepage
  (:require
   [utter.style :as style]
   [utter.components.container :refer [container]]
   [re-frame.core :as rf]
   [utter.store.user :as user]
   [reagent.core :as r]))

(defn home-page []
      [container {:title "UtterBot - Home"}
       [:h1 "Utter - Univeral Bot" [:sup "(W.I.P)"]]])
