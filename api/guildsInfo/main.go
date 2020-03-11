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

type GuildInfoEvent struct {
	Token string `json:"token"`
}

type Guild struct {
	Id 	   string `json:"id"`
	Name   string `json:"name"`
	Icon   string `json:"icon"`
	Access bool   `json:"name"`  // The user has permissions to modify the guild
}

type GuildInfoResponse struct {
	Guilds []Guild `json:"guilds"`
}

type DiscordGuild struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Icon        string `json:"icon"`
	Owner       bool   `json:"owner"`
	Permissions int    `json:"permissions"`
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

	fmt.Printf("%v", claims)

	client := http.Client{}
	req, _ := http.NewRequest("GET", "https://discordapp.com/api/users/@me/guilds", nil)
	req.Header.Set("authorization", "Bearer " + claims.Token)
	res, err := client.Do(req)
	if err != nil {
		return guildsResponse, err
	}

	defer res.Body.Close()
	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		return guildsResponse, errors.New("read-body")
	}

	var discordGuilds []DiscordGuild
	if json.Unmarshal(body, &discordGuilds) != nil {
		return guildsResponse, errors.New("body-unmarshal")
	}

	for _, v := range discordGuilds {
		// Filter guilds where utterbot is not in
		guildsResponse.Guilds = append(guildsResponse.Guilds, Guild{
			Id:     v.ID,
			Name:   v.Name,
			Icon:   v.Icon,
			Access: v.Permissions&8 != 0 || v.Permissions&32 != 0,
		})
	}
	fmt.Printf("%v", discordGuilds)
	return guildsResponse, nil
}

func main() {
	lambda.Start(handle)
}

