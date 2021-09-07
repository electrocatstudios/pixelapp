package main

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/dgrijalva/jwt-go"
	"github.com/gorilla/mux"
	"gorm.io/gorm"

	"golang.org/x/crypto/bcrypt"
)

// AdminUser - Allow user to log in and view stuff
type AdminUser struct {
	gorm.Model
	Name     string
	Email    string
	Salt     string
	Password string
}

// CheckAdminTables - Create tables for admin users if necessary
func CheckAdminTables() {
	db, err := GetDBConnection()

	if err != nil {
		fmt.Println("Failed to connect to db to check tables")
		return
	}

	config := GetDBConfig()
	prepend := config.TablePrepend

	tableName := fmt.Sprintf("%s_admin", prepend)

	db.Table(tableName).AutoMigrate(&AdminUser{})
}

func performAdminLogin(w http.ResponseWriter, r *http.Request) {
	var input LoginRequest
	err := json.NewDecoder(r.Body).Decode(&input)
	if err != nil {
		resp := LoginResponse{Status: "fail", Message: "failed to decode body"}
		json.NewEncoder(w).Encode(&resp)
		return
	}
	// Check database to see if we have a user
	db, err := GetDBConnection()
	if err != nil {
		fmt.Println("Failed to get valid db connection")
		resp := LoginResponse{Status: "fail", Message: "failed to get db connection"}
		json.NewEncoder(w).Encode(&resp)
	}

	var user AdminUser
	config := GetDBConfig()
	prepend := config.TablePrepend
	tableName := fmt.Sprintf("%s_admin", prepend)

	if result := db.Table(tableName).First(&user, "email=?", input.Email); result.Error != nil {
		resp := LoginResponse{Status: "fail", Message: "Email does not match any account"}
		json.NewEncoder(w).Encode(&resp)
		return
	}

	err = bcrypt.CompareHashAndPassword([]byte(user.Password), []byte(input.Password))
	if err != nil {
		resp := LoginResponse{Status: "fail", Message: "Password does not match"}
		json.NewEncoder(w).Encode(&resp)
		return
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"username": user.Name,
		"id":       user.ID,
		"admin":    true,
	})

	secret := GetDBConfig().Secret
	tokenString, err := token.SignedString([]byte(secret))
	if err != nil {
		fmt.Println("Failed to gen the token")
		fmt.Println(err)
		resp := LoginResponse{Status: "fail", Message: "failed to generate token"}
		json.NewEncoder(w).Encode(&resp)
		return
	}

	resp := LoginResponse{Status: "ok"}
	resp.Token = tokenString
	json.NewEncoder(w).Encode(&resp)
}

// AddAdminRoutes - Add in the api routes for admin login etc
func AddAdminRoutes(r *mux.Router) error {
	r.HandleFunc("/api/admin/login", performAdminLogin).Methods("POST")
	// r.HandleFunc("/api/admin/user", createUser).Methods("POST")

	return nil
}
