package common

import "github.com/dgrijalva/jwt-go"

type Claims struct {
	Token        string `json:"token"`
	RefreshToken string `json:"refreshToken"`
	jwt.StandardClaims
}
