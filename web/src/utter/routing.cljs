(ns utter.routing
  (:require
   [kee-frame.core :as k]
   [re-frame.core :as rf]
   [reagent.cookies :as c]
   [utter.store.db :as db]
   [utter.constants :refer [debug?]]
   [utter.pages.homepage :refer [home-page]]
   [utter.pages.loginredirect :refer [login-redirect]]
   [utter.pages.panel :refer [panel-page]]
   [utter.pages.commands :refer [commands-page]]))


(def routes
  [["/"               :home]
   ["/redirect/:code" :redirect]
   ["/panel"          :panel]
   ["/commands"       :commands]])

(defn router []
  (rf/dispatch [:load-user (c/get :user)])
  [k/switch-route (comp :name :data)
   :home     [home-page]
   :redirect [login-redirect]
   :panel    [panel-page]
   :commands [commands-page]
   nil [:div "Loading..."]])

(k/start! {:debug?         debug?
           :routes         routes
           :hash-routing?  debug?
           :initial-db     db/initial-state
           :app-db-spec    ::db/db-spec})