(ns utter.pages.commands
  (:require
   [reagent.core :as r]
   [utter.style :as style]
   [clojure.string :refer [blank? includes?]]
   [utter.components.container :refer [container]]
   [utter.components.optionspanel :refer [options-panel]]
   [utter.components.utterlist :refer [utter-list]]))

(def commands
  [{:id 0 :kind 1 :name ".help" :description "Shows infromations about commands"}
   {:id 1 :kind 1 :name ".prefix" :description "Changes prefix"}
   {:id 2 :kind 2 :name ".ban" :description "Bans users from your server"}])

(def categories
  [{:id 0 :name "All Commands" :icon "A"}
   {:id 1 :name "Main Commands" :icon "M"}
   {:id 2 :name "Moderation Commands" :icon "M"}
   {:id 3 :name "Ticket Commands" :icon "T"}])

(defn commands-page []
  (let [selected-category (r/atom 0)
        search (r/atom "")]
    (fn []
      [container {:title "UtterBot - Commands"}
       [options-panel {:title ((categories @selected-category) :name)
                       :options (map-indexed #(hash-map
                                               :name (%2 :icon)
                                               :selected? (= @selected-category %1)
                                               :on-click (fn []
                                                           (reset! selected-category %1)
                                                           (reset! search "")))
                                             categories)}]
       [style/card
        [:h2 "Search"]
        [style/input-field {
                            :placeholder "Search"
                            :value @search
                            :on-change #(do
                                          (reset! search (-> % .-target .-value))
                                          (reset! selected-category 0))}]]
       [utter-list {:entries
                    (cond
                      (not (blank? @search)) (filterv #(includes? (% :name) @search) commands)
                      (= @selected-category 0) commands
                      :else (filterv #(= (:kind %) @selected-category) commands))}]])))