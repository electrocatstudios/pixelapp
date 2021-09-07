package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
)

// DBConfig - format of data to be read from file
type DBConfig struct {
	Port                string `json:"port"`
	Hostname            string `json:"hostname"`
	Database            string `json:"database"`
	Username            string `json:"username"`
	Password            string `json:"password"`
	Secret              string `json:"secret"`
	Environment         string `json:"environment"`
	WebsockDelay        int    `json:"websockdelay"`
	JWTSecret           string `json:"jwtsecret"`
	TablePrepend        string `json:"tableprepend"`
	DefaultNumberPixels int    `json:"defaultnumberpixels"`
}

// GetDBConfig - get the config from the config file
func GetDBConfig() DBConfig {
	ret := DBConfig{}

	// Read in from config file
	file, err := ioutil.ReadFile("config.json")
	if err != nil {
		fmt.Println("Failed to read in the config file")
		return ret
	}
	err = json.Unmarshal([]byte(file), &ret)
	if err != nil {
		fmt.Println("Failed to decode the json config")
		return ret
	}

	if ret.DefaultNumberPixels == 0 {
		ret.DefaultNumberPixels = 5
	}
	return ret
}
