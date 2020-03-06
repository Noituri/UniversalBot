(ns utter.core
  (:require
   [reagent.core :as r]
   [utter.store.db :as db]
   [utter.pages.homepage :refer [home-page]]))

;; -------------------------
;; Initialize app


(defn mount-root []
  (db/init)
  (r/render [home-page] (.getElementById js/document "app")))

(defn init! []
  (mount-root))
