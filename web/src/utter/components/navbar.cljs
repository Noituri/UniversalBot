(ns utter.components.navbar
  (:require
   [utter.style :refer [nav-bar nav-item nav-logo nav-items-container]]
   [kee-frame.core :as k]
   [reagent.core :as r]))

(defn navbar []
  [nav-bar
   [nav-logo {:href (k/path-for [:home])} "UtterBot"]
   [nav-items-container
    [nav-item "Invite"]
    [nav-item {:href (k/path-for [:commands])} "Commands"]
    [nav-item {:href (k/path-for [:panel])} "Web Panel"]]])