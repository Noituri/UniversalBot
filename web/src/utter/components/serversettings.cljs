(ns utter.components.serversettings
  (:require
   [kee-frame.core :as k]
   [reagent.core :as r]
   [re-frame.core :as rf]
   [reagent.cookies :as c]
   [day8.re-frame.http-fx]
   [ajax.core :as ajax]
   [utter.constants :refer [modify-guild]]
   [utter.style :as style]))

(rf/reg-event-db
 :modify-failed
 (fn [db [_ err]]
   (println err)
   (assoc db :modify-failed true)))

(rf/reg-sub
 :modify-failed?
 (fn [db _]
   (:modify-failed db)))

(k/reg-chain :save-settings
             (fn [ctx [guild-id prefix muted-role mod-logs-channel]]
               {:http-xhrio {:method          :post
                             :uri             (modify-guild)
                             :timeout         8000
                             :format          (ajax/json-request-format)
                             :body            (.stringify js/JSON (clj->js {:token ((c/get :user) :token)
                                                                            :guild_id guild-id
                                                                            :prefix prefix
                                                                            :muted_role muted-role
                                                                            :mod_logs_channel mod-logs-channel}))
                             :response-format (ajax/json-response-format {:keywords? true})
                             :on-failure      [:modify-failed]}})

             (fn [_ _]
               {}))

(defn server-settings [{:keys [guild_id prefix muted_role mod_logs_channel]}]
  (let [bot-prefix (r/atom prefix)
        muted-role (r/atom muted_role)
        mod-logs   (r/atom mod_logs_channel)]
    (fn []
      [style/card
       [:h2 "Settings"]
       [style/settings-container
        [:h4 "Prefix"]
        [style/input-field {:placeholder "Bot Prefix" :value @bot-prefix :on-change #(reset! bot-prefix (-> % .-target .-value))}]
        [:h4 "Muted Role"]
        [style/input-field {:placeholder "Muted Role ID" :value @muted-role :on-change #(reset! muted-role (-> % .-target .-value))}]
        [:h4 "Mod-logs Channel"]
        [style/input-field {:placeholder "Mod-logs Channel ID" :value @mod-logs :on-change #(reset! mod-logs (-> % .-target .-value))}]
        [style/gradient-btn {:style {:marginTop "20px"}
                             :on-click #(rf/dispatch [:save-settings guild_id @bot-prefix @muted-role @mod-logs])} "Save"]]])))