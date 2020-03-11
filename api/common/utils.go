package common

import (
	"encoding/json"
	"errors"
	"fmt"
	"github.com/dgrijalva/jwt-go"
	"io/ioutil"
	"net/http"
	"os"
)

const (
	DiscordEndpoint = "https://discordapp.com/api"
	DiscordGuildsEndpoint = DiscordEndpoint + "/users/@me/guilds"
	DiscordCurrentUserEndpoint = DiscordEndpoint + "/users/@me"
)

func GetDiscordGuilds(token string) ([]DiscordGuild, error) {
	client := http.Client{}
	req, _ := http.NewRequest("GET", DiscordGuildsEndpoint, nil)
	req.Header.Set("authorization", "Bearer " + token)
	res, err := client.Do(req)
	if err != nil {
		return nil, err
	}

	defer res.Body.Close()
	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		return nil, errors.New("read-body")
	}
	var discordGuilds []DiscordGuild
	if json.Unmarshal(body, &discordGuilds) != nil {
		return nil, errors.New("body-unmarshal")
	}

	return discordGuilds, nil
}

func GetDiscordCurrentUser(token string) (*DiscordUser, error) {
	client := http.Client{}
	req, _ := http.NewRequest("GET", DiscordCurrentUserEndpoint, nil)
	req.Header.Set("authorization", "Bearer " + token)
	res, err := client.Do(req)
	if err != nil {
		return nil, err
	}

	defer res.Body.Close()
	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		return nil, errors.New("read-body")
	}

	var discordUser DiscordUser
	if json.Unmarshal(body, &discordUser) != nil {
		return nil, errors.New("body-unmarshal")
	}

	return &discordUser, nil
}

func GetJWTClaims(token string) (*Claims, error) {
	secret, ok := os.LookupEnv("jwt_secret")
	if !ok {
		return nil, errors.New("empty-secret")
	}

	parsedToken, err := jwt.ParseWithClaims(token, &Claims{}, func(token *jwt.Token) (i interface{}, err error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("Unexpected signing method: %v", token.Header["alg"])
		}

		return []byte(secret), nil
	})
	if err != nil {
		return nil, errors.New("wrong-token")
	}

	claims, ok := parsedToken.Claims.(*Claims)
	if !ok || !parsedToken.Valid {
		return nil, errors.New("invalid-token")
	}

	return claims, nil
}

func IsDebug() bool {
	return os.Getenv("DEBUG") == "true"
}