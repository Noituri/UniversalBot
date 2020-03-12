(ns utter.store.user
  (:require
   [reagent.cookies :as c]
   [re-frame.core :as rf]))

(rf/reg-event-db
 :logout
 (fn [db [_ r]]
   (c/remove! :user)
   (rf/dispatch [:go-home])
   (assoc db :user nil)))

(rf/reg-event-db
 :load-user
 (fn [db [_ user]]
   (assoc db :user user)))

(rf/reg-sub
 :user
 (fn [db _]
   (:user db)))