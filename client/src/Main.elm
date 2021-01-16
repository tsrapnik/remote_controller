module Main exposing (..)

import Browser
import Html exposing (Html, button, div, li, text)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)
import Http
import Json.Encode as Encode


main : Program String Model Msg
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
    | Netflix
    | VrtNuTvGuide
    | VrtNuLive



{- model -}


type alias Model =
    { server_ip : String }


init : String -> ( Model, Cmd Msg )
init server_ip =
    ( { server_ip = server_ip }, Cmd.none )



{- view -}


view : Model -> Html Msg
view model =
    li []
        [ button [ onClick (PostCommand ShutDown) ] [ text "shut down" ]
        , button [ onClick (PostCommand Brightness100) ] [ text "brightness 100" ]
        , button [ onClick (PostCommand Brightness50) ] [ text "brightness 50" ]
        , button [ onClick (PostCommand Brightness0) ] [ text "brightness 0" ]
        , button [ onClick (PostCommand ShutdownMonitor) ] [ text "shutdown monitor" ]
        , button [ onClick (PostCommand Netflix) ] [ text "netflix" ]
        , button [ onClick (PostCommand VrtNuTvGuide) ] [ text "vrt nu tv guide" ]
        , button [ onClick (PostCommand VrtNuLive) ] [ text "vrt nu live" ]
        ]



{- update -}


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        PostCommand command ->
            ( model
            , postCommand command model.server_ip
            )

        CommandPosted result ->
            ( model
            , Cmd.none
            )



{- http -}


postCommand : RemoteCommand -> String -> Cmd Msg
postCommand remoteCommand server_ip =
    Http.post
        { url = "http://" ++ server_ip
        , body = Http.jsonBody (remoteCommandToJson remoteCommand)
        , expect = Http.expectWhatever CommandPosted
        }



{- conversions -}


remoteCommandToJson : RemoteCommand -> Encode.Value
remoteCommandToJson remoteCommand =
    case remoteCommand of
        ShutDown ->
            Encode.object[("command", Encode.string "shutdown")]

        Brightness100 ->
            Encode.object[("command", Encode.string "brightness_100")]

        Brightness50 ->
            Encode.object[("command", Encode.string "brightness_50")]

        Brightness0 ->
            Encode.object[("command", Encode.string "brightness_0")]

        ShutdownMonitor ->
            Encode.object[("command", Encode.string "shutdown_monitor")]

        Netflix ->
            Encode.object[("command", Encode.string "netflix")]

        VrtNuTvGuide ->
            Encode.object[("command", Encode.string "vrt_nu_tv_guide")]

        VrtNuLive ->
            Encode.object[("command", Encode.string "vrt_nu_live")]
