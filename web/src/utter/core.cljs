(ns utter.core
  (:require
   [reagent.core :as r]
   [re-frame.core :as rf]
   [utter.pages.homepage :refer [home-page]]))

;; -------------------------
;; Initialize app

(rf/reg-event-db
 :initialize
 (fn [_ _]
   {:amount 0}))

(defn mount-root []
  (rf/dispatch-sync [:initialize])
  (r/render [home-page] (.getElementById js/document "app")))

(defn init! []
  (mount-root))
