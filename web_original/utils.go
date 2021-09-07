package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"math/rand"
	"strconv"
	"strings"
	"time"

	"github.com/dgrijalva/jwt-go"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// GenerateRandomString - Generate random string of chars
func GenerateRandomString(length int) string {
	rand.Seed(time.Now().UnixNano())
	chars := []rune("abcdefghijklmnopqrstuvwxyz")

	var b strings.Builder
	for i := 0; i < length; i++ {
		b.WriteRune(chars[rand.Intn(len(chars))])
	}
	return b.String()
}

var connPool *gorm.DB

// GetDBConnection - get connection to the database
func GetDBConnection() (*gorm.DB, error) {

	if connPool != nil {
		return connPool, nil
	}

	conf := GetDBConfig()
	sslMode := "require"

	if conf.Environment == "staging" {
		// Don't use ssl on staging
		sslMode = "disable"
	}
	connStr := fmt.Sprintf("host=%s port=%s user=%s dbname=%s password=%s sslmode=%s", conf.Hostname, conf.Port, conf.Username, conf.Database, conf.Password, sslMode)

	db, err := gorm.Open(postgres.Open(connStr), &gorm.Config{})
	// Set up the db features
	sqlDB, _ := db.DB()
	// SetMaxIdleConns sets the maximum number of connections in the idle connection pool.
	sqlDB.SetMaxIdleConns(10)

	// SetMaxOpenConns sets the maximum number of open connections to the database.
	sqlDB.SetMaxOpenConns(100)

	// SetConnMaxLifetime sets the maximum amount of time a connection may be reused.
	sqlDB.SetConnMaxLifetime(15 * time.Minute)

	if err != nil {
		fmt.Println("Failed to connect to db")
		fmt.Println(err)
		return nil, err
	}
	return db, nil
}

// GetUserFromToken - Pass in token from request - get User back
func GetUserFromToken(reqToken string) (User, error) {
	var user User

	splitToken := strings.Split(reqToken, "Bearer ")
	reqToken = splitToken[1]

	config := GetDBConfig()
	secret := config.JWTSecret

	claims := jwt.MapClaims{}

	jwt.ParseWithClaims(reqToken, claims, func(token *jwt.Token) (interface{}, error) {
		return []byte(secret), nil
	})

	db, err := GetDBConnection()
	if err != nil {
		return user, err
	}

	email := claims["email"]
	if email == nil || email == "" {
		return user, errors.New("payload incorrectly formatted - logout and login again")
	}

	playerID := claims["id"]

	if result := db.Table("gsv_user").First(&user, "ID=?", playerID); result.Error != nil {
		fmt.Println(result.Error)
		return user, errors.New("User does not exist")
	}

	if user.MaxNumberPixels < 1 {
		user.MaxNumberPixels = config.DefaultNumberPixels
		db.Table("gsv_user").Save(&user)
	}

	return user, nil
}

func GetMenuBar() string {
	content, err := ioutil.ReadFile("html/snippets/menu_bar.html")
	if err != nil {
		fmt.Println("Failed to read in menu_bar.html")
		return ""
	}

	return string(content)
}

func GetPixelMenuBar() string {
	content, err := ioutil.ReadFile("html/snippets/pixel_menu_bar.html")
	if err != nil {
		fmt.Println("Failed to read in pixel_menu_bar.html")
		return ""
	}

	return string(content)
}

func GetPixelToolbar() string {
	content, err := ioutil.ReadFile("html/snippets/pixel_tool_bar.html")
	if err != nil {
		fmt.Println("Failed to read in pixel_tool_bar.html")
		return ""
	}
	return string(content)
}

func GetPixelRenderMenu() string {
	content, err := ioutil.ReadFile("html/snippets/render_menu_bar.html")
	if err != nil {
		fmt.Println("Failed to read in menu_bar.html")
		return ""
	}

	return string(content)
}

func GetTableName(tablename string) string {
	conf := GetDBConfig()
	prepend := conf.TablePrepend
	return fmt.Sprintf("%s%s", prepend, tablename)
}

// GetHexColorFromRGB - return a hex string from the red, green and blue
func GetHexColorFromRGB(r, g, b int) string {
	red := fmt.Sprintf("%x", r)
	if r < 16 {
		red = fmt.Sprintf("0%x", r)
	}
	green := fmt.Sprintf("%x", g)
	if g < 16 {
		green = fmt.Sprintf("0%x", g)
	}
	blue := fmt.Sprintf("%x", b)
	if b < 16 {
		blue = fmt.Sprintf("0%x", b)
	}
	return fmt.Sprintf("%s%s%s", red, green, blue)
}

// GetColorFromHex - return the int vals of red, green and blue from hex string
func GetColorFromHex(hex string) (int, int, int) {
	redstr := hex[0:2]
	greenstr := hex[2:4]
	bluestr := hex[4:6]
	// TODO: Error checks on conversions
	r, _ := strconv.ParseInt(redstr, 16, 32)
	g, _ := strconv.ParseInt(greenstr, 16, 32)
	b, _ := strconv.ParseInt(bluestr, 16, 32)
	return int(r), int(g), int(b)
}
