package common

import "github.com/dgrijalva/jwt-go"

type Claims struct {
	Token        string `json:"token"`
	RefreshToken string `json:"refreshToken"`
	jwt.StandardClaims
}

type DiscordGuild struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Icon        string `json:"icon"`
	Owner       bool   `json:"owner"`
	Permissions int    `json:"permissions"`
}
