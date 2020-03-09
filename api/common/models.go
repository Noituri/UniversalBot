package common

type Server struct {
	Id             int
	Guildid        string   `gorm:"type:varchar"`
	Prefix         string   `gorm:"varchar;default:'.'"`
	Enabledmodules []string `gorm:"type:text[];default:'{}'"`
}
