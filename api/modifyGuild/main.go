package main

import (
	"api/common"
	"context"
	"errors"
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
	lambda.Start(handle)
}
