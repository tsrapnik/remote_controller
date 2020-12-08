module Main exposing (..)

import Browser
import Html exposing (Html, button, div, li, text)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = \_ -> Sub.none
        }


type Msg
    = ShutDown



{- model -}


type alias Model =
    {}


init : () -> ( Model, Cmd Msg )
init () =
    ( {}, Cmd.none )



{- view -}


view : Model -> Html Msg
view model =
    li []
        [ button [ onClick ShutDown ] [ text "shut down" ]
        ]



{- update -}


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ShutDown ->
            ( model
            , Cmd.none
            )
