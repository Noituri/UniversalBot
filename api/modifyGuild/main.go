package main

import (
	"api/common"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"github.com/aws/aws-lambda-go/lambda"
	"io/ioutil"
	"log"
	"net/http"
)

type ModifyGuildEvent struct {
	Token          string `json:"token"`
	GuildID        string `json:"guild_id"`
	Prefix         string `json:"prefix"`
	MutedRole      string `json:"muted_role"`
	ModLogsChannel string `json:"mod_logs_channel"`
}

func handle(ctx context.Context, event ModifyGuildEvent) (string, error) {
	claims, err := common.GetJWTClaims(event.Token)
	if err != nil {
		return "", err
	}

	discordGuilds, err := common.GetDiscordGuilds(claims.Token)
	if err != nil {
		return "", err
	}

	db := common.GetConnection()
	var server common.Server
	db.Where("guildid LIKE ?", event.GuildID).First(&server)
	if server.Guildid == "" {
		return "", errors.New("guild-not-found-db")
	}

	for _, v := range discordGuilds {
		if v.ID == event.GuildID {
			if v.Permissions&8 != 0 || v.Permissions&32 != 0 {
				server.Prefix = event.Prefix
				db.Save(&server)

				var specialEntities []common.SpecialEntity
				db.Model(&server).Related(&specialEntities)
				modLogsChanged := false
				mutedRoleChanged := false
				for i, se := range specialEntities {
					switch se.EntityType {
					case 1:
						specialEntities[i].EntityId = event.ModLogsChannel
						modLogsChanged = true
					case 2:
						specialEntities[i].EntityId = event.MutedRole
						mutedRoleChanged = true
					}

					db.Save(&specialEntities[i])
				}

				if !modLogsChanged {
					entity := common.SpecialEntity{
						Server:     server,
						ServerId:   uint(server.Id),
						EntityType: 1,
						EntityId:   event.ModLogsChannel,
					}
					db.Create(&entity)
				}

				if !mutedRoleChanged {
					entity := common.SpecialEntity{
						Server:     server,
						ServerId:   uint(server.Id),
						EntityType: 2,
						EntityId:   event.MutedRole,
					}
					db.Create(&entity)
				}

				return `{"status": "ok"}`, nil
			} else {
				return "", errors.New("permissions")
			}
		}
	}

	return "", errors.New("guild-not-found")
}

func main() {
	if common.IsDebug() {
		http.HandleFunc("/modify-guild", func(writer http.ResponseWriter, request *http.Request) {
			writer.Header().Set("Access-Control-Allow-Origin", "*")
			var event ModifyGuildEvent
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
		log.Fatal(http.ListenAndServe(":8110", nil))
	} else {
		lambda.Start(handle)
	}
}
