module Main exposing (..)

import Browser
import Html exposing (Html, button, div, li, text)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)
import Http


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = \_ -> Sub.none
        }


type Msg
    = PostCommand RemoteCommand
    | CommandPosted (Result Http.Error ())


type RemoteCommand
    = ShutDown
    | Brightness100
    | Brightness50
    | Brightness0
    | ShutdownMonitor



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
        [ button [ onClick (PostCommand ShutDown) ] [ text "shut down" ]
        , button [ onClick (PostCommand Brightness100) ] [ text "brightness 100" ]
        , button [ onClick (PostCommand Brightness50) ] [ text "brightness 50" ]
        , button [ onClick (PostCommand Brightness0) ] [ text "brightness 0" ]
        , button [ onClick (PostCommand ShutdownMonitor) ] [ text "shutdown monitor" ]
        ]



{- update -}


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        PostCommand command ->
            ( model
            , postCommand command
            )

        CommandPosted result ->
            ( model
            , Cmd.none
            )



{- http -}


postCommand : RemoteCommand -> Cmd Msg
postCommand remoteCommand =
    Http.post
        { url = "http://192.168.1.2/"
        , body = Http.stringBody "text/plain" (remoteCommandToString remoteCommand)
        , expect = Http.expectWhatever CommandPosted
        }



{- conversions -}


remoteCommandToString : RemoteCommand -> String
remoteCommandToString remoteCommand =
    case remoteCommand of
        ShutDown ->
            "shutdown"
        Brightness100 ->
            "brightness_100"
        Brightness50 ->
            "brightness_50"
        Brightness0 ->
            "brightness_0"
        ShutdownMonitor ->
            "shutdown_monitor"
