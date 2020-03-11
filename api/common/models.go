package common

import "github.com/lib/pq"

type Server struct {
	Id             int
	Guildid        string   `gorm:"type:varchar"`
	Prefix         string   `gorm:"varchar;default:'.'"`
	Enabledmodules pq.StringArray `gorm:"type:text;default:'{}'"`
}
