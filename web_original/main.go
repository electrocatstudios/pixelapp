package main

import (
	"fmt"
	"html/template"
	"log"
	"net/http"
	"os"
	"strings"

	"github.com/gorilla/mux"
)

func indexPage(w http.ResponseWriter, r *http.Request) {
	http.ServeFile(w, r, "/index.html")
}

func getJS(w http.ResponseWriter, r *http.Request) {
	filename := strings.Split(r.URL.Path, "/")[2]
	http.ServeFile(w, r, "js/"+filename)
}

func getImg(w http.ResponseWriter, r *http.Request) {
	filename := strings.Split(r.URL.Path, "/")[2]
	http.ServeFile(w, r, "img/"+filename)
}

func getCSS(w http.ResponseWriter, r *http.Request) {
	filename := strings.Split(r.URL.Path, "/")[2]
	http.ServeFile(w, r, "css/"+filename)
}

func getLoginPage(w http.ResponseWriter, r *http.Request) {
	menu_tmpl := template.Must(template.ParseFiles("html/login.html"))
	data := map[string]interface{}{
		"MenuBar": template.HTML(GetMenuBar()),
	}
	menu_tmpl.Execute(w, data)
}

func getRegisterPage(w http.ResponseWriter, r *http.Request) {
	http.ServeFile(w, r, "html/register.html")
}

func fileExists(filename string) bool {
	info, err := os.Stat(filename)
	if os.IsNotExist(err) {
		return false
	}
	return !info.IsDir()
}

func getLetsEncrypt(w http.ResponseWriter, r *http.Request) {
	// Get the let's encrypt challenge file
	fp := "le" + r.URL.Path

	if !fileExists(fp) {
		http.ServeFile(w, r, "html/404.html")
	} else {
		http.ServeFile(w, r, fp)
	}
}

func main() {
	fmt.Println("Starting server")

	CheckAdminTables()
	// CheckDbTables()
	// CheckSnowgameTables()
	// LoadSavedData()

	r := mux.NewRouter()
	r.HandleFunc("/", indexPage)
	r.HandleFunc("/login", getLoginPage)
	r.HandleFunc("/register", getRegisterPage)

	r.HandleFunc("/js/{filename}", getJS)
	r.HandleFunc("/img/{filename}", getImg)

	r.HandleFunc("/css/{filename}", getCSS)

	// Let's Encrypt route - for certs
	r.HandleFunc("/.well-known/acme-challenge/{filepath}", getLetsEncrypt)

	AddAdminRoutes(r)
	AddApiRoutes(r)
	AddPixelAppRoutes(r)

	http.Handle("/", r)

	log.Fatal(http.ListenAndServe(":8081", nil))
}
