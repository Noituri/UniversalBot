package common

import (
	"errors"
	"fmt"
	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/postgres"
	"log"
	"os"
)

var db *gorm.DB

func init() {
	_ = GetConnection()
}

func GetConnection() *gorm.DB {
	if db != nil {
		return db
	}

	con, err := connection()
	if err != nil {
		log.Fatalf("Could not connect to db. Error: %s", err)
	}

	db = con
	return db
}

func connection() (*gorm.DB, error) {
	dbHost, ok := os.LookupEnv("db_host")
	if !ok {
		return nil, errors.New("empty-db-host")
	}

	dbPort, ok := os.LookupEnv("db_port")
	if !ok {
		return nil, errors.New("empty-db-port")
	}

	dbUser, ok := os.LookupEnv("db_user")
	if !ok {
		return nil, errors.New("empty-db-user")
	}

	dbName, ok := os.LookupEnv("db_name")
	if !ok {
		return nil, errors.New("empty-db-name")
	}

	dbPass, ok := os.LookupEnv("db_pass")
	if !ok {
		return nil, errors.New("empty-db-password")
	}

	dbString := fmt.Sprintf("host=%s port=%s user=%s dbname=%s password=%s",
		dbHost, dbPort, dbUser, dbName, dbPass)

	db, err := gorm.Open("postgres", dbString)
	if err != nil {
		return nil, errors.New("db-open")
	}

	return db, nil
}
