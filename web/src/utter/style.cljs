(ns utter.style
  (:require-macros
    [cljss.core])
  (:require
    [cljss.core :as css :refer [inject-global] :refer-macros [defstyles]]
    [cljss.reagent :refer-macros [defstyled]])
  )

;; -------------------------
;; Global styles

(inject-global
  {:html {
    :margin 0
    :background-color "#1f1c3b"}
   "*" {:font-family "Open Sans"}})

(defstyles content []
  {:text-align "center"})

(defstyled h1 :h1
  {:color "#fff"
   :font-weight "bold"})

