(ns utter.store.user
  (:require
   [utter.store.db :refer [check-spec-interceptor]]
   [re-frame.core :as rf]))

(rf/reg-event-db
 :login
 check-spec-interceptor
 (fn [db [_ new-user]]
   (assoc db :user new-user)))

(rf/reg-event-db
 :logout
 (fn [db [_ _]]
   (assoc db :user nil)))

(rf/reg-sub
 :user
 (fn [db _]
   (:user db)))