package main

import (
	"api/common"
	"context"
	"errors"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/dgrijalva/jwt-go"
	"golang.org/x/oauth2"
	"os"
	"time"
)

type LoginEvent struct {
	Code string `json:"code"`
}

type LoginResponse struct {
	Username string `json:"username"`
	Avatar   string `json:"avatar"`
	Token    string `json:"token"`
}

func handle(ctx context.Context, event LoginEvent) (*LoginResponse, error) {
	secret, ok := os.LookupEnv("jwt_secret")
	if !ok {
		return nil, errors.New("empty-secret")
	}

	redirectUrl, ok := os.LookupEnv("oauth_redirect")
	if !ok {
		return nil, errors.New("empty-oauth-redirect")
	}

	clientID, ok := os.LookupEnv("oauth_client_id")
	if !ok {
		return nil, errors.New("empty-oauth-client-id")
	}

	clientSecret, ok := os.LookupEnv("oauth_client_secret")
	if !ok {
		return nil, errors.New("empty-oauth-client-secret")
	}

	oauth := &oauth2.Config{
		RedirectURL:  redirectUrl,
		ClientID:     clientID,
		ClientSecret: clientSecret,
		Scopes:       []string{"email", "identify", "guilds"},
		Endpoint: oauth2.Endpoint{
			AuthURL:   "https://discordapp.com/api/oauth2/authorize",
			TokenURL:  "https://discordapp.com/api/oauth2/token",
			AuthStyle: 0,
		},
	}

	token, err := oauth.Exchange(ctx, event.Code)
	if err != nil {
		return nil, errors.New(err.Error())
	}

	jwtToken := jwt.NewWithClaims(jwt.SigningMethodHS256, common.Claims{
		Token:        token.AccessToken,
		RefreshToken: token.RefreshToken,
		StandardClaims: jwt.StandardClaims{
			ExpiresAt: time.Now().UTC().Add(730 * time.Hour).UnixNano(),
		},
	})

	finalToken, err := jwtToken.SignedString([]byte(secret))
	if err != nil {
		return nil, err
	}
	user, err := common.GetDiscordCurrentUser(token.AccessToken)
	if err != nil {
		return nil, err
	}

	return &LoginResponse{
		Username: user.Username,
		Avatar:   user.Avatar,
		Token:    finalToken,
	}, nil
}

func main() {
	lambda.Start(handle)
}
