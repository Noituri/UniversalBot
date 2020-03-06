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
    :background "linear-gradient(#1F1B40, #171430)"
    :background-repeat "no-repeat"
    :background-attachment "fixed"
    :color "#fff"}
   "*" {:font-family "Open Sans"}})


(defstyled container :div
  {:text-align "center"})

(defstyled nav-bar :nav
  {:display "flex"
   :margin-top "-5px"
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

