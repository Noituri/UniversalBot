(ns utter.store.user
  (:require
   [re-frame.core :as rf]))

(rf/reg-event-db
 :logout
 (fn [db [_ r]]
   (assoc db :user nil)))

(rf/reg-sub
 :user
 (fn [db _]
   (:user db)))