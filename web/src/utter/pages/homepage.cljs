(ns utter.pages.homepage
  (:require
   [utter.style :as style]
   [utter.components.container :refer [container]]
   [utter.components.featurecard :refer [featurecard]]
   [re-frame.core :as rf]
   [utter.store.user :as user]
   [reagent.core :as r]))

;; TODO: add technology section
(defn home-page []
  [container {:title "UtterBot - Home"}
   [:div
    [style/heading "Utter"]
    [:h1 "The Univeral Bot"]
    [style/gradient-btn "Try it!"]]
   [:div {:style {:margin-top "30px"}}
    [:h2 "Features"]
    [style/features
     [featurecard 
      {:title "Powerful"
       :description "Need moderation, utilities or tickets? We've got you covered! UtterBot offers many commands categorised into modules."}]
     [featurecard
      {:title "Configurable"
       :description "Don't need some commands? Disable them!\nNeed only ticket commands? Just enable ticket module!"}]
     [featurecard
      {:title "Web Panel"
       :description "Configure UtterBot from your web browser. Check the moderation logs!"}]
     [featurecard
      {:title "Open Source"
       :description "Want to check the code out? Or contribute to the project? Everything is open-source. Feel free to dive in to the project!"}]]]])
