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

type Guilds struct {
	Id 	   string `json:"id"`
	Name   string `json:"name"`
	Access bool   `json:"name"`  // UtterBot has access to that guild
}

type GuildInfoResponse struct {
	guilds []Guilds `json:"guilds"`
}

func handle(ctx context.Context, event GuildInfoEvent) (GuildInfoResponse, error) {
	guilds := GuildInfoResponse{guilds:nil}
	secret, ok := os.LookupEnv("jwt_secret")
	if !ok {
		return guilds, errors.New("empty-secret")
	}

	token, err := jwt.ParseWithClaims(event.Token, &common.Claims{}, func(token *jwt.Token) (i interface{}, err error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("Unexpected signing method: %v", token.Header["alg"])
		}

		return secret, nil
	})
	if err != nil {
		return guilds, errors.New("wrong-token")
	}

	claims, ok := token.Claims.(common.Claims)
	if !ok || !token.Valid {
		return guilds, errors.New("invalid-token")
	}

	fmt.Printf("%v", claims)
	return guilds, nil
}

func main() {
	lambda.Start(handle)
}

