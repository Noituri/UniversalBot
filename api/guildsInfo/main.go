package main

import (
	"api/common"
	"context"
	"encoding/json"
	"fmt"
	"github.com/aws/aws-lambda-go/lambda"
	"io/ioutil"
	"log"
	"net/http"
)

type GuildInfoEvent struct {
	Token string `json:"token"`
}

type Guild struct {
	Id     string `json:"id"`
	Name   string `json:"name"`
	Icon   string `json:"icon"`
	Access bool   `json:"access"` // The user has permissions to modify the guild
}

type GuildInfoResponse struct {
	Guilds []Guild `json:"guilds"`
}

func handle(ctx context.Context, event GuildInfoEvent) ([]Guild, error) {
	claims, err := common.GetJWTClaims(event.Token)
	if err != nil {
		return nil, err
	}

	discordGuilds, err := common.GetDiscordGuilds(claims.Token)
	if err != nil {
		return nil, err
	}

	db := common.GetConnection()
	var servers []common.Server
	db.Find(&servers)

	var guildsResponse []Guild
	for _, v := range discordGuilds {
		for _, s := range servers {
			if s.Guildid == v.ID {
				guildsResponse = append(guildsResponse, Guild{
					Id:     v.ID,
					Name:   v.Name,
					Icon:   v.Icon,
					Access: v.Permissions&8 != 0 || v.Permissions&32 != 0,
				})
			}
		}
	}
	return guildsResponse, nil
}
func main() {
	if common.IsDebug() {
		http.HandleFunc("/guilds", func(writer http.ResponseWriter, request *http.Request) {
			writer.Header().Set("Access-Control-Allow-Origin", "*")
			var event GuildInfoEvent
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
		log.Fatal(http.ListenAndServe(":8090", nil))
	} else {
		lambda.Start(handle)
	}
}
