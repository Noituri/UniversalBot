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
	"net/http"
	"os"
)

type GuildDetailEvent struct {
	Token   string `json:"token"`
	GuildID string `json:"guild_id"`
}

func handle(ctx context.Context, event GuildDetailEvent) (string, error) {
	secret, ok := os.LookupEnv("jwt_secret")
	if !ok {
		return "", errors.New("empty-secret")
	}

	token, err := jwt.ParseWithClaims(event.Token, &common.Claims{}, func(token *jwt.Token) (i interface{}, err error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("Unexpected signing method: %v", token.Header["alg"])
		}

		return []byte(secret), nil
	})
	if err != nil {
		return "", errors.New("wrong-token")
	}

	claims, ok := token.Claims.(*common.Claims)
	if !ok || !token.Valid {
		return "", errors.New("invalid-token")
	}

	client := http.Client{}
	req, _ := http.NewRequest("GET", "https://discordapp.com/api/users/@me/guilds", nil)
	req.Header.Set("authorization", "Bearer "+claims.Token)
	res, err := client.Do(req)
	if err != nil {
		return "", err
	}

	defer res.Body.Close()
	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		return "", errors.New("read-body")
	}
	var discordGuilds []common.DiscordGuild
	if json.Unmarshal(body, &discordGuilds) != nil {
		return "", errors.New("body-unmarshal")
	}

	db := common.GetConnection()
	var servers []common.Server
	db.Find(&servers) // TODO: "WHERE guildid = '<event.guildid>'

	return "OK", nil
}

func main() {
	lambda.Start(handle)
}
