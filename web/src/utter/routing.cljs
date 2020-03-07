(ns utter.routing
  (:require
   [kee-frame.core :as k]
   [utter.store.db :as db]
   [utter.pages.homepage :refer [home-page]]
   [utter.pages.panel :refer [panel-page]]
   [utter.pages.commands :refer [commands-page]]))

(def routes
  [["/"         :home]
   ["/panel"    :panel]
   ["/commands" :commands]])

(def debug? false)

(def router
  [k/switch-route (comp :name :data)
   :panel    [panel-page]
   :home     [home-page]
   :commands [commands-page]
   nil [:div "Loading..."]])

(k/start! {:debug?         debug?
           :routes         routes
           :hash-routing?  debug?
           :initial-db     db/initial-state
           :app-db-spec    ::db-spec})