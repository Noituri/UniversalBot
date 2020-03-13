package common

import (
	"encoding/json"
	"errors"
	"fmt"
	"github.com/bwmarrin/discordgo"
	"github.com/dgrijalva/jwt-go"
	"io/ioutil"
	"net/http"
	"os"
	"time"
)

const (
	DiscordEndpoint = "https://discordapp.com/api"
	DiscordGuildsEndpoint = DiscordEndpoint + "/users/@me/guilds"
	DiscordCurrentUserEndpoint = DiscordEndpoint + "/users/@me"
)

var rl *discordgo.RateLimiter

func init() {
	_ = getRateLimiter()
}

func getRateLimiter() *discordgo.RateLimiter {
	if rl != nil {
		return rl
	}

	rl = discordgo.NewRatelimiter()
	return rl
}

func sendRequest(method, endpoint, token string, out interface{}) error {
	limiter := getRateLimiter()
	bucket := limiter.LockBucket(endpoint)
	client := http.Client{}
	req, _ := http.NewRequest(method, endpoint, nil)
	req.Header.Set("authorization", "Bearer " + token)
	res, err := client.Do(req)
	if err != nil {
		return err
	}

	err = bucket.Release(res.Header)
	if err != nil {
		return err
	}

	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		return errors.New("read-body")
	}

	if res.StatusCode == 429 {
		ratelimit := discordgo.TooManyRequests{}
		err = json.Unmarshal(body, &ratelimit)
		if err != nil {
			return errors.New("rate-limit")
		}
		time.Sleep(ratelimit.RetryAfter * time.Millisecond)
		return sendRequest(method, endpoint, token, out)
	}

	defer func() {
		err := res.Body.Close()
		if err != nil {
			return
		}
	}()

	if json.Unmarshal(body, out) != nil {
		println(string(body))
		return errors.New("body-unmarshal")
	}


	return nil
}

func GetDiscordGuilds(token string) ([]DiscordGuild, error) {
	var discordGuilds []DiscordGuild
	if err := sendRequest("GET", DiscordGuildsEndpoint, token, &discordGuilds); err != nil {
		return nil, err
	}

	return discordGuilds, nil
}

func GetDiscordCurrentUser(token string) (*DiscordUser, error) {
	var discordUser DiscordUser
	if err := sendRequest("GET", DiscordCurrentUserEndpoint, token, &discordUser); err != nil {
		return nil, err
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