(ns utter.pages.panel
  (:require
   [kee-frame.core :as k]
   [re-frame.core :as rf]
   [reagent.cookies :as c]
   [day8.re-frame.http-fx]
   [ajax.core :as ajax]
   [utter.constants :refer [get-guilds]]
   [utter.components.container :refer [container]]
   [utter.components.serverselector :refer [server-selector]]
   [utter.components.optionspanel :refer [options-panel]]
   [utter.components.utterlist :refer [utter-list]]
   [utter.components.serversettings :refer [server-settings]]
   [reagent.core :as r]
   [utter.style :as style]))

;; TODO: add loading indicator
(k/reg-controller :panel
                  {:params (fn [route-data]
                             (when (-> route-data :data :name (= :panel))
                               (-> route-data
                                   :path-params
                                   :id)))
                   :start  (fn [ctx _] [:panel/load])})

(rf/reg-event-db
 :guild-retrieve-failed
 (fn [db [_ err]]
   (println err)
   (assoc db :guilds-retrieve-failed true)))

(rf/reg-sub
 :guilds-retrieve-failed?
 (fn [db _]
   (:guilds-retrieve-failed db)))

(rf/reg-sub :selected-guild
         (fn []
           (rf/subscribe [:kee-frame/route]))
         (fn [route _]
           (-> route :path-params :id)))

(rf/reg-event-fx
 :select-guild
 (fn [_ [_ server]]
   (println server)
   {:navigate-to [:panel {:id server}]}))

(k/reg-chain :panel/load
             (fn [ctx [_]]
               {:http-xhrio {:method          :post
                             :uri             (get-guilds)
                             :timeout         8000
                             :format          (ajax/json-request-format)
                             :body            (.stringify js/JSON (clj->js {:token ((c/get :user) :token)}))
                             :response-format (ajax/json-response-format {:keywords? true})
                             :on-failure      [:guild-retrieve-failed]}})

             (fn [{:keys [db]} [result _]]
               {:db (assoc db :guilds result)}))

(rf/reg-sub
 :guilds
 (fn [db _]
   (:guilds db)))

(defn actions-list []
   [utter-list {:name "Actions"
                :entries [{:id 2
                           :name "Ban"
                           :description "User XXX has beeen banned by YYY"}
                          {:id 1
                           :name "Ban"
                           :description "User XXX has beeen banned by YYY"}
                          {:id 0
                           :name "Ban"
                           :description "User XXX has beeen banned by YYY"}]}])

(defn panel-page []
  (let [selected-option (r/atom 0)
        guilds (rf/subscribe [:guilds])
        selected-guild (rf/subscribe [:selected-guild])]
    (fn []
       (if (-> @guilds count (> 0))
         [container {:title "UtterBot - Panel"}
          [server-selector {:servers @guilds :selected @selected-guild}]
          [options-panel {:options
                          [{:icon "fa-newspaper"
                            :selected? (= @selected-option 0)
                            :on-click #(reset! selected-option 0)}
                           {:icon "fa-wrench"
                            :selected? (= @selected-option 1)
                            :on-click #(reset! selected-option 1)}]}]
          (case @selected-option
            0 [actions-list]
            1 [server-settings])
          [:div
           [style/gradient-btn {:bg :red :on-click #(rf/dispatch [:logout])} "Logout"]]]
         [container {:title "UtterBot - Panel"}
          [style/card
           [:h2 "Invite UtterBot first!"]
           [style/gradient-btn "Invite"]]
          [:div
           [style/gradient-btn {:bg :red :on-click #(rf/dispatch [:logout])} "Logout"]]]))))