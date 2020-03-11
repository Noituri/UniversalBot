package common

import (
	"github.com/lib/pq"
	"time"
)

type Server struct {
	Id             int
	Guildid        string         `gorm:"type:varchar"`
	Prefix         string         `gorm:"varchar;default:'.'"`
	Enabledmodules pq.StringArray `gorm:"type:text;default:'{}'"`
}

type Action struct {
	Id           int       `json:"-"`
	Server       Server    `gorm:"foreignkey:ServerId" json:"-"`
	ServerId     uint      `json:"-"`
	ActionType   int       `json:"action_type"`
	Issuer       string    `gorm:"type:varchar" json:"issuer"`
	Target       string    `gorm:"type:varchar" json:"target"`
	Message      string    `gorm:"type:varchar" json:"message"`
	CreationDate time.Time `gorm:"type:timestamp" json:"creation_date"`
}

type SpecialEntity struct {
	Id         int
	Server     Server `gorm:"foreignkey:ServerId" json:"-"`
	ServerId   uint
	EntityType int
	EntityId   string `gorm:"type:varchar"`
}
