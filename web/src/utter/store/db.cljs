(ns utter.store.db
  (:require
   [cljs.spec.alpha :as s]
   [re-frame.core :as rf]))

(s/def ::username string?)
(s/def ::avatar string?)
(s/def ::token string?)
(s/def ::user (s/or 
               :signed-in (s/keys :req-un [::username ::avatar ::token])
               :signed-out nil?))

(s/def ::db-spec (s/keys :req-un [::user]))

(defn check-spec [spec db]
  (when-not (s/valid? spec db)
   (throw (ex-info
           (str "spec check failed: " (s/explain-str spec db)) {}))))

(def check-spec-interceptor (rf/after (partial check-spec ::db-spec)))

(def initial-state
  {:user nil
   :guilds nil})

(defn init [] 
  (rf/dispatch-sync [:initialize]))