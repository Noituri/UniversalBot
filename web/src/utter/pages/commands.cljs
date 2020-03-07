(ns utter.pages.commands
  (:require
   [utter.components.container :refer [container]]
   [reagent.core :as r]
   [utter.components.optionspanel :refer [options-panel]]
   [utter.components.utterlist :refer [utter-list]]))

(defn commands-page []
  (let [title (r/atom "All Commands")]
    (fn []
      [container {:title "UtterBot - Commands"}
       [options-panel {:title @title
                       :options [{:name "A" :selected? true}
                                 {:name "M" :selected? false}
                                 {:name "M" :selected? false}
                                 {:name "T" :selected? false}]}]
       [utter-list {:entries [{:name ".help" :description "Shows infromations about commands"}
                              {:name ".prefix" :description "Changes prefix"}]}]])))