module Main exposing (..)

import Browser
import Html exposing (Html, button, div, h1, input, li, text)
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
    = Shutdown
    | ShutdownMonitor
    | Brightness String
    | Volume String
    | Netflix
    | VrtNuTvGuide
    | VrtNuLive
    | Spotify



{- model -}


type alias Model =
    { server_ip : String, brightness : Int, volume : Int }


init : String -> ( Model, Cmd Msg )
init server_ip =
    ( { server_ip = server_ip, brightness = 0, volume = 0 }, Cmd.none )



{- view -}


view : Model -> Html Msg
view model =
    li []
        [ button [ onClick (PostCommand Shutdown) ] [ text "shut down" ]
        , button [ onClick (PostCommand ShutdownMonitor) ] [ text "shutdown monitor" ]
        , div []
            [ h1 [] [ text "monitor brightness" ]
            , input
                [ type_ "range"
                , Html.Attributes.min "0"
                , Html.Attributes.max "100"
                , value <| String.fromInt model.brightness
                , onInput (\newValue -> PostCommand (Brightness newValue))
                ]
                []
            ]
        , div []
            [ h1 [] [ text "volume" ]
            , input
                [ type_ "range"
                , Html.Attributes.min "0"
                , Html.Attributes.max "100"
                , value <| String.fromInt model.volume
                , onInput (\newValue -> PostCommand (Volume newValue))
                ]
                []
            ]
        , button [ onClick (PostCommand Netflix) ] [ text "netflix" ]
        , button [ onClick (PostCommand VrtNuTvGuide) ] [ text "vrt nu tv guide" ]
        , button [ onClick (PostCommand VrtNuLive) ] [ text "vrt nu live" ]
        , button [ onClick (PostCommand Spotify) ] [ text "spotify" ]
        ]



{- update -}


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        PostCommand command ->
            case command of
                Brightness value ->
                    let
                        brightness =
                            Maybe.withDefault 0 (String.toInt value)
                    in
                    ( { model | brightness = brightness }
                    , postCommand command model.server_ip
                    )

                Volume value ->
                    let
                        volume =
                            Maybe.withDefault 0 (String.toInt value)
                    in
                    ( { model | volume = volume }
                    , postCommand command model.server_ip
                    )

                _ ->
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
        Shutdown ->
            Encode.object [ ( "Shutdown", Encode.null ) ]

        ShutdownMonitor ->
            Encode.object [ ( "ShutdownMonitor", Encode.null ) ]

        Brightness value ->
            let
                brightness =
                    Maybe.withDefault 0 (String.toInt value)
            in
            Encode.object [ ( "Brightness", Encode.object [ ( "value", Encode.int brightness ) ] ) ]

        Volume value ->
            let
                volume =
                    Maybe.withDefault 0 (String.toInt value)
            in
            Encode.object [ ( "Volume", Encode.object [ ( "value", Encode.int volume ) ] ) ]

        Netflix ->
            Encode.object [ ( "Netflix", Encode.null ) ]

        VrtNuTvGuide ->
            Encode.object [ ( "VrtNuTvGuide", Encode.null ) ]

        VrtNuLive ->
            Encode.object [ ( "VrtNuLive", Encode.null ) ]

        Spotify ->
            Encode.object [ ( "Spotify", Encode.null ) ]
