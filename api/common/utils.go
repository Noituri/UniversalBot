package common

import (
	"encoding/json"
	"errors"
	"io/ioutil"
	"net/http"
)

const DiscordGuildsEndpoint = "https://discordapp.com/api/users/@me/guilds"

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
