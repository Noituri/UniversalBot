package main

import (
	"api/common"
	"context"
	"errors"
	"fmt"
	"github.com/aws/aws-lambda-go/lambda"
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
	resp, err := handle(context.Background(), ModifyGuildEvent{
		Token:       "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ0b2tlbiI6Ikc3Z25YUlYyQ2pndE9IUVV1RnN0NDFvVHJzbkwybiIsInJlZnJlc2hUb2tlbiI6Ikk5MzRNSlgxZGRiRVZwbHp3WlhGT1h3RlhEek5sVCIsImV4cCI6MTU4NjU3NzMwOTAzMzg0MjgzMH0.Tsl5uh3e8lfpKjxocejopBBa0fEWb5RdnFOCgDT7PJ8",
		GuildID:     "567768634735198209",
		Prefix:         "!",
		MutedRole:      "1212123123123",
		ModLogsChannel: "1232133355555",
	})
	if err != nil {
		println(err.Error())
		return
	}

	fmt.Printf("%v", resp)
	return
	lambda.Start(handle)
}
