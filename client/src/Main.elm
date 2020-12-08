import Browser
import Html exposing (Html, button, div, input, text, time)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)

main : Program Encode.Value Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = \_ -> Sub.none
        }


type Msg
    = EmptyMessage



{- model -}


type alias Model =
    {
    }

init : ( Model, Cmd Msg )
init 
    ({}, Msg.none)


{- view -}


view : Model -> Html Msg
view model =
    li []
        [
            [ button [ onClick Save ] [ text "shut down" ]
            ]
        ]


{- update -}


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        EmptyMessage ->
            (model
            , msg.none
            )


