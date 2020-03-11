package main

import (
	"api/common"
	"context"
	"errors"
	"fmt"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/dgrijalva/jwt-go"
	"os"
)

type GuildInfoEvent struct {
	Token string `json:"token"`
}

type Guild struct {
	Id     string `json:"id"`
	Name   string `json:"name"`
	Icon   string `json:"icon"`
	Access bool   `json:"name"` // The user has permissions to modify the guild
}

type GuildInfoResponse struct {
	Guilds []Guild `json:"guilds"`
}

func handle(ctx context.Context, event GuildInfoEvent) (GuildInfoResponse, error) {
	guildsResponse := GuildInfoResponse{Guilds: nil}
	secret, ok := os.LookupEnv("jwt_secret")
	if !ok {
		return guildsResponse, errors.New("empty-secret")
	}

	token, err := jwt.ParseWithClaims(event.Token, &common.Claims{}, func(token *jwt.Token) (i interface{}, err error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("Unexpected signing method: %v", token.Header["alg"])
		}

		return []byte(secret), nil
	})
	if err != nil {
		return guildsResponse, errors.New("wrong-token")
	}

	claims, ok := token.Claims.(*common.Claims)
	if !ok || !token.Valid {
		return guildsResponse, errors.New("invalid-token")
	}

	discordGuilds, err := common.GetDiscordGuilds(claims.Token)
	if err != nil {
		return guildsResponse, err
	}

	db := common.GetConnection()
	var servers []common.Server
	db.Find(&servers)

	for _, v := range discordGuilds {
		for _, s := range servers {
			if s.Guildid == v.ID {
				guildsResponse.Guilds = append(guildsResponse.Guilds, Guild{
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
	lambda.Start(handle)
}
