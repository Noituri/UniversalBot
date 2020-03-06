(ns utter.pages.homepage
  (:require
   [utter.style :as style]
   [re-frame.core :as rf]
   [utter.store.user :as user]
   [reagent.core :as r]))

(defn home-page []
      [:div {:class (style/content)}
       [:title "Utter - Home"]
       [:p (when-some [user @(rf/subscribe [:user])]
             (:name user))]
       [:button {:on-click #(rf/dispatch [:login {:name "Noit"}])}
        "CLICK"]
       [style/h1 "Utter - Univeral Bot" [:sup "(W.I.P)"]]])
