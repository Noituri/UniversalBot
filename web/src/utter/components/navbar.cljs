(ns utter.components.navbar
  (:require
   [utter.style :refer [nav-bar nav-item nav-logo nav-items-container]]
   [reagent.core :as r]))

(defn navbar []
  [nav-bar
   [nav-logo "UtterBot"]
   [nav-items-container
    [nav-item "Invite"]
    [nav-item "Commands"]
    [nav-item "Web Panel"]]])