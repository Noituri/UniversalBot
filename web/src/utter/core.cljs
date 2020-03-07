(ns utter.core
  (:require
   [utter.routing :refer [router]]
   [reagent.core :as r]))

;; -------------------------
;; Initialize app

(defn mount-root []
  (r/render router
            (.getElementById js/document "app")))

(defn init! []
  (mount-root))
