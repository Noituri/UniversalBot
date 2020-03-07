(ns utter.pages.commands
  (:require
   [utter.components.container :refer [container]]
   [reagent.core :as r]
   [utter.components.optionspanel :refer [options-panel]]
   [utter.components.utterlist :refer [utter-list]]))

(def commands
  [{:kind 1 :name ".help" :description "Shows infromations about commands"}
   {:kind 1 :name ".prefix" :description "Changes prefix"}
   {:kind 2 :name ".ban" :description "Bans users from your server"}])

(def categories
  [{:name "All Commands" :icon "A"}
   {:name "Main Commands" :icon "M"}
   {:name "Moderation Commands" :icon "M"}
   {:name "Ticket Commands" :icon "T"}])

(defn commands-page []
  (let [selected-category (r/atom 0)]
    (fn []
      [container {:title "UtterBot - Commands"}
       [options-panel {:title (categories @selected-category)
                       :options (map-indexed #(hash-map :name (%2 :icon)
                                                        :selected? (= @selected-category %1))
                                             categories)}]
       [utter-list {:entries 
                    (if (= @selected-category 0)
                      commands
                      (filterv #(= (:kind %) @selected-category) commands))}]])))