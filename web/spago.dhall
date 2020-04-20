{-
Welcome to a Spago project!
You can edit this file as you like.
-}
{ name = "Utter"
, dependencies =
  [ "aff-bus"
  , "affjax"
  , "argonaut"
  , "console"
  , "effect"
  , "foreign"
  , "generics-rep"
  , "halogen"
  , "integers"
  , "psci-support"
  , "record"
  , "routing"
  , "routing-duplex"
  , "tolerant-argonaut"
  , "web-events"
  ]
, packages = ./packages.dhall
, sources = [ "src/**/*.purs", "test/**/*.purs" ]
}
