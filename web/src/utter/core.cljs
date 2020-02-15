(ns utter.core
  (:require
    [reagent.core :as r]
    [utter.pages.homepage :refer [home-page]]))

;; -------------------------
;; Initialize app

(defn mount-root []
  (r/render [home-page] (.getElementById js/document "app")))

(defn init! []
  (mount-root))
