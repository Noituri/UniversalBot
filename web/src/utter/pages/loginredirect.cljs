(ns utter.pages.loginredirect
  (:require
   [kee-frame.core :as k]
   [utter.style :as style]
   [utter.components.container :refer [container]]
   [day8.re-frame.http-fx]
   [ajax.core :as ajax]
   [utter.constants :refer [code-exchange]]
   [re-frame.core :as rf]))

(k/reg-controller :redirect
                  {:params (fn [route-data]
                             (when (-> route-data :data :name (= :redirect))
                               (-> route-data
                                   :path-params
                                   :code)))
                   :start  (fn [ctx code] [:redirect/load code])})

(rf/reg-event-db
 :login-request-failed
 (fn [db [_ err]]
   (println err)
   (assoc db :login-failed true)))

(rf/reg-sub
 :login-failed?
 (fn [db _]
   (:login-failed db)))

(k/reg-chain :redirect/load
             (fn [ctx [code]]
               {:http-xhrio {:method          :post
                             :uri             (code-exchange)
                             :timeout         8000
                             :format          (ajax/json-request-format)
                             :body            (str "{\"code\":\"" code "\"}")
                             :response-format (ajax/json-response-format {:keywords? true})
                             :on-failure      [:login-request-failed]}})

             (fn [{:keys [db]} [_ result]]
               {:db (assoc db :user result)}))

(rf/reg-event-fx :go-home
              (fn [_ _]
                {:navigate-to [:home]}))

(defn login-redirect []
  (let [user (rf/subscribe [:user])]
    (fn []
      [container {:title "UtterBot - Signing In"}
       [:div
        [style/heading "UtterBot"]
        [:h1 (cond
               (some? @user) (str "Welcome " (@user :username) "!")
               @(rf/subscribe [:login-failed?]) "Error occurred while signing in!"
               :else "Signing in...")]
        (when (some? @user) [style/gradient-btn {:on-click #(rf/dispatch [:go-home])} "Home"])]])))
