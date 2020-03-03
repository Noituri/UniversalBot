(ns utter.pages.homepage
  (:require
   [utter.style :as style]
   [reagent.core :as r]
   [re-frame.core :as rf]))


(rf/reg-event-db               
 :add 
 (fn [db [_ _]]
   (update-in db [:amount] (fnil inc 0))))

(rf/reg-sub
 :amount
 (fn [db _]
   (:amount db)))

(defn home-page []
      [:div {:class (style/content)}
       [:title "Utter - Home"]
       [:button {:on-click #(rf/dispatch [:add])}
        @(rf/subscribe [:amount])]
       [style/h1 "Utter - Univeral Bot" [:sup "(W.I.P)"]]])
