(ns utter.store.db
  (:require
   [cljs.spec.alpha :as s]
   [re-frame.core :as rf]))

(s/def ::name string?)
(s/def ::user (s/or 
               :signed-in (s/keys :req-un [::name])
               :signed-out nil?))

(s/def ::db-spec (s/keys :req-un [::user]))

(defn check-spec [spec db]
  (when-not (s/valid? spec db)
   (throw (ex-info
           (str "spec check failed: " (s/explain-str spec db)) {}))))

(def check-spec-interceptor (rf/after (partial check-spec ::db-spec)))

(def initial-state
  {:user nil})
;; (rf/reg-event-db
;;  :initialize
;;  [check-spec-interceptor]
;;  (fn [_ _]
;;    {:user nil}))

(defn init [] 
  (rf/dispatch-sync [:initialize]))