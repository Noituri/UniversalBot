(ns utter.components.navbar
  (:require
   [utter.style :refer [nav-bar nav-item nav-logo nav-items-container]]
   [utter.constants :as c]
   [kee-frame.core :as k]
   [re-frame.core :as rf]))

(defn navbar []
  [nav-bar
   [nav-logo {:href (k/path-for [:home])} "UtterBot"]
   [nav-items-container
    [nav-item "Invite"]
    [nav-item {:href (k/path-for [:commands])} "Commands"]
    (if (nil? @(rf/subscribe [:user]))
      [nav-item {:href c/login-redirect} "Login"]
      [nav-item {:href (k/path-for [:panel {:id 0}])} "Web Panel"])]])