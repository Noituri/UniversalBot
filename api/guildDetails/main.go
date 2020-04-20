package main

import (
	"api/common"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/dgrijalva/jwt-go"
	"io/ioutil"
	"log"
	"net/http"
	"os"
)

type GuildDetailsEvent struct {
	Token       string `json:"token"`
	GuildID     string `json:"guild_id"`
	ActionsFrom int    `json:"actions_from"`
}

type GuildDetailsResponse struct {
	GuildId        string          `json:"guild_id"`
	Actions        []common.Action `json:"actions"`
	Prefix         string          `json:"prefix"`
	MutedRole      string          `json:"muted_role"`
	ModLogsChannel string          `json:"mod_logs_channel"`
}

func handle(ctx context.Context, event GuildDetailsEvent) (*GuildDetailsResponse, error) {
	secret, ok := os.LookupEnv("jwt_secret")
	if !ok {
		return nil, errors.New("empty-secret")
	}

	token, err := jwt.ParseWithClaims(event.Token, &common.Claims{}, func(token *jwt.Token) (i interface{}, err error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("Unexpected signing method: %v", token.Header["alg"])
		}

		return []byte(secret), nil
	})
	if err != nil {
		return nil, errors.New("wrong-token")
	}

	claims, ok := token.Claims.(*common.Claims)
	if !ok || !token.Valid {
		return nil, errors.New("invalid-token")
	}

	discordGuilds, err := common.GetDiscordGuilds(claims.Token)
	if err != nil {
		return nil, err
	}

	db := common.GetConnection()
	var server common.Server
	db.Where("guildid LIKE ?", event.GuildID).First(&server)
	if server.Guildid == "" {
		return nil, errors.New("guild-not-found-db")
	}

	for _, v := range discordGuilds {
		if v.ID == event.GuildID {
			if v.Permissions&8 != 0 || v.Permissions&32 != 0 {
				model := GuildDetailsResponse{
					GuildId:        v.ID,
					Actions:        nil,
					Prefix:         server.Prefix,
					MutedRole:      "",
					ModLogsChannel: "",
				}

				db.Model(&server).Order("creation_date desc").Offset(event.ActionsFrom).Limit(10).Related(&model.Actions)
				var specialEntities []common.SpecialEntity
				db.Model(&server).Related(&specialEntities)
				for _, se := range specialEntities {
					switch se.EntityType {
					case 1:
						model.ModLogsChannel = se.EntityId
					case 2:
						model.MutedRole = se.EntityId
					}
				}

				return &model, nil
			} else {
				return nil, errors.New("permissions")
			}
		}
	}

	return nil, errors.New("guild-not-found")
}

func main() {
	if common.IsDebug() {
		http.HandleFunc("/guild-details", func(writer http.ResponseWriter, request *http.Request) {
			writer.Header().Set("Access-Control-Allow-Origin", "*")
			writer.Header().Set("Access-Control-Allow-Headers", "*")
			if request.Method != "POST" {
				return
			}
			println("REQUEST")

			var event GuildDetailsEvent
			body, _ := ioutil.ReadAll(request.Body)
			defer request.Body.Close()
			json.Unmarshal(body, &event)
			resp, err := handle(context.Background(), event)
			if err != nil {
				writer.WriteHeader(http.StatusInternalServerError)
				fmt.Fprintf(writer, `{"error": "%s"}`, err.Error())
			} else {
				result, _ := json.Marshal(resp)
				fmt.Fprintf(writer, string(result))
			}
		})
		log.Fatal(http.ListenAndServe(":8100", nil))
	} else {
		lambda.Start(handle)
	}
}
