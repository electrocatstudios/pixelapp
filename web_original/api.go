package main

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/dgrijalva/jwt-go"
	"github.com/gorilla/mux"
	"golang.org/x/crypto/bcrypt"
)

type LoginRequest struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}
type LoginResponse struct {
	Status  string `json:"status"`
	Message string `json:"message"`
	Token   string `json:"token"`
}

type CreateUserRequest struct {
	Username string `json:"username"`
	Email    string `json:"email"`
	Password string `json:"password"`
	Public   bool   `json:"public"`
}

type CreateUserResponse struct {
	Status  string `json:"status"`
	Message string `json:"message"`
}

func performLogin(w http.ResponseWriter, r *http.Request) {
	var input LoginRequest
	err := json.NewDecoder(r.Body).Decode(&input)
	if err != nil {
		resp := LoginResponse{Status: "fail", Message: "failed to decode body"}
		json.NewEncoder(w).Encode(&resp)
		return
	}

	conf := GetDBConfig()
	if conf.Environment == "staging" {
		if input.Email == "test" && input.Password == "test" {
			token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
				"username": "test",
				"id":       0,
				"email":    "test",
			})
			secret := conf.Secret
			tokenString, _ := token.SignedString([]byte(secret))
			var resp LoginResponse
			resp.Status = "ok"
			resp.Token = tokenString
			json.NewEncoder(w).Encode(&resp)
			return
		}
	}

	// Check database to see if we have a user
	db, err := GetDBConnection()
	if err != nil {
		fmt.Println("Failed to get valid db connection")
		resp := LoginResponse{Status: "fail", Message: "failed to get db connection"}
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var user User
	// db.Table("gsv_user").Find(&user, "email=?", input.Email)

	if result := db.Table("gsv_user").First(&user, "email=?", input.Email); result.Error != nil {
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
		"email":    user.Email,
	})

	secret := conf.Secret
	tokenString, err := token.SignedString([]byte(secret))
	if err != nil {
		fmt.Println("Failed to gen the token")
		fmt.Println(err)
		resp := LoginResponse{Status: "fail", Message: "failed to generate token"}
		json.NewEncoder(w).Encode(&resp)
		return
	}

	// fmt.Println(tokenString)

	resp := LoginResponse{Status: "ok"}
	resp.Token = tokenString
	json.NewEncoder(w).Encode(&resp)
}

func createUser(w http.ResponseWriter, r *http.Request) {
	resp := CreateUserResponse{}
	resp.Status = "ok"

	var req CreateUserRequest
	err := json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "failed to parse request body"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db, err := GetDBConnection()
	if err != nil {
		fmt.Println("Failed to get valid db connection")
		resp.Status = "fail"
		resp.Message = "Failed to get valid db connection"
		json.NewEncoder(w).Encode(&resp)
		return
	}
	var user User
	if result := db.Table("gsv_user").First(&user, "name=?", req.Username); result.Error == nil {
		fmt.Println(result.Error)

		resp.Status = "fail"
		resp.Message = "Username is already in use, please choose another"
		json.NewEncoder(w).Encode(&resp)
		return
	}
	if result := db.Table("gsv_user").First(&user, "email=?", req.Email); result.Error == nil {
		resp.Status = "fail"
		resp.Message = "Email is already in use please choose another"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	// fmt.Println(user)
	user.Email = req.Email
	user.Name = req.Username

	salt, err := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	if err != nil {
		fmt.Println(err)
		return
	}
	// fmt.Println(string(salt))
	user.Password = string(salt)

	db.Table("gsv_user").Create(&User{Name: user.Name, Email: user.Email, Password: user.Password})

	// resp.Status = "ok"
	json.NewEncoder(w).Encode(&resp)
}

// AddApiRoutes - Add in the api routes for user login etc
func AddApiRoutes(r *mux.Router) error {
	r.HandleFunc("/api/login", performLogin).Methods("POST")
	r.HandleFunc("/api/user", createUser).Methods("POST")

	return nil
}
