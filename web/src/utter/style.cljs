(ns utter.style
  (:require-macros
    [cljss.core])
  (:require
    [cljss.core :as css :refer [inject-global]]
    [cljss.reagent :refer-macros [defstyled]])
  )

;; -------------------------
;; Global styles

(inject-global
  {:html {
          :margin 0
          :background "linear-gradient(#2c275a, #0f0c2c)"
          :background-repeat "no-repeat"
          :background-attachment "fixed"
          :color "#fff"}
   "*" {:font-family "Open Sans"}
   :body {:margin 0}
   "::placeholder" {:text-align "center"
                    :color "#bfbfc0"}})


(defstyled container :div
  {:display "flex"
   :text-align "center"
   :flex-direction "column"
   :width "100%"
   :flex-wrap "wrap"
   :align-content "center"
   :justify-content "center"})

(defstyled heading :h1
  {:font-size "72px"
   :margin "40px 0 0 0"
   :font-weight "bold"})

(defstyled nav-bar :nav
  {:display "flex"
   :margin "5px 10px 0 10px"
   :justify-content "space-between"
   :align-items "center"})

(defstyled nav-logo :a
  {:cursor "pointer"
   :font-weight "bold"
   :text-decoration "none"
   :&:link {:color "#fff"}
   :&:visited {:color "#fff"}
   :&:hover {:color "#a59bff"}})

(defstyled nav-items-container :div
  {:display "flex"})

(defstyled nav-item :a
  {:cursor "pointer"
   :text-decoration "none"
   :padding "5px"
   :&:link {:color "#fff"}
   :&:visited {:color "#fff"}
   :&:hover {:color "#a59bff"}})

(defstyled gradient-btn :p
  {:display "inline-flex"
   :cursor "pointer"
   :background "linear-gradient(45deg, #2d266b, #a59bff 80%)"
   :border-radius "15px"
   :font-size "24px"
   :padding "5px"
   :justify-content "center"
   :width "100px"})

(defstyled features :div
  {::css/media 
   {[:only :screen :and [:min-width "750px"]]
    {:display "flex"
     :flex-direction "row"
     :flex-wrap "wrap"
     :max-width "700px"
     :justify-content "space-around"}}})

(defstyled card :div
  {:border-radius "10px"
   :background-color "#242151"
   :padding "6px"
   :flex "1 0 40%"
   :margin "10px 5px 10px 5px"
   :width "100%"
   :max-width "300px"
   :height "130px"})

(defstyled circle-img :img
  {:border-radius "100px"
   :width "80px"
   :height "80px"})

(defstyled panel-option :div
  {:background-color "#2e2a66"
   :border-radius "10px"
   :width "30px"
   :height "30px"
   :padding "10px !important"
   ">*" {:margin 0}})

(defstyled horizontal-view :div
  {:overflow "auto"
   :white-space "nowrap"
   :padding "0px 20px 20px 20px"
   ">*" {:display "inline-block"
         :margin "0 10px"
         :cursor "pointer"}
   ">.selected" {:background-color (with-meta #(str %) :color)
                 :padding "10px 10px 0 10px"
                 :border-radius "10px"}})

(defstyled list-entry :div
  {:background-color "#2e2a66"
   :border-radius "10px"
   :padding "10px"
   :margin-bottom "10px"
   :width "inherit"
   :&:hover {:background-color "#2a265b"}
   ".list-text>*" {:text-align "left"
                   :margin 0}})

(defstyled input-field :input
  {:border-radius "10px"
   :outline "none"
   :border "none"
   :background-color "#2e2a66"
   :width "90%"
   :text-align "center"
   :font-size "16px"
   :padding "10px 5px"
   :color "#fff"})

(defstyled settings-container :div
  {">input" {:margin-top "10px"}
   ">h4" {:margin "10px 0 0 0"}})
