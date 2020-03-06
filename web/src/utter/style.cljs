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
   :body {:margin 0}})


(defstyled container :div
  {:display "flex"
   :text-align "center"
   :width "100%"
   :flex-wrap "wrap"
   :justify-content "center"})

(defstyled heading :h1
  {:font-size "72px"
   :font-weight "bold"})

(defstyled nav-bar :nav
  {:display "flex"
   :margin "-5px 10px 0 10px"
   :justify-content "space-between"
   :align-items "center"})

(defstyled nav-logo :p
  {:cursor "pointer"
   :font-weight "bold"
   :text-decoration "none"
   :&:hover {:color "#a59bff"}})

(defstyled nav-items-container :div
  {:display "flex"})

(defstyled nav-item :p
  {:cursor "pointer"
   :text-decoration "none"
   :padding "5px"
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

