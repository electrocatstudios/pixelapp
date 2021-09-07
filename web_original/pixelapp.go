package main

import (
	"encoding/json"
	"fmt"
	"html/template"
	"image"
	"io"
	"math/rand"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	guuid "github.com/google/uuid"
	"github.com/gorilla/mux"
	"gorm.io/gorm"
)

// PixelAppImageDB - table for holding image information
type PixelAppImageDB struct {
	gorm.Model
	OwnerUserID uint      `gorm:"column:owner_user_id"`
	TimeCreated time.Time `gorm:"column:time_created"`
	LastUpdated time.Time `gorm:"column:last_update"`
	Name        string    `gorm:"column:name"`
	Description string    `gorm:"column:description"`
	Width       int       `gorm:"column:width"`
	Height      int       `gorm:"column:height"`
	PixelWidth  int       `gorm:"column:pixel_width"`
	GUID        string    `gorm:"column:guid"`
}

// PixelItemDB - an individual pixel for the image
type PixelItemDB struct {
	gorm.Model
	PixelImageID uint    `gorm:"column:pixel_image_id"`
	X            int     `gorm:"column:x"`
	Y            int     `gorm:"column:y"`
	R            int     `gorm:"column:r"`
	G            int     `gorm:"column:g"`
	B            int     `gorm:"column:b"`
	Alpha        float64 `gorm:"column:alpha"`
	Layer        int     `gorm:"column:layer"`
	Frame        int     `gorm:"frame"`
}

// PixelShaderLayerDB - the shader layer pixels for the image
type PixelShaderLayerDB struct {
	gorm.Model
	PixelImageID uint    `gorm:"column:pixel_image_id"`
	X            int     `gorm:"column:x"`
	Y            int     `gorm:"column:y"`
	R            int     `gorm:"column:r"`
	G            int     `gorm:"column:g"`
	B            int     `gorm:"column:b"`
	Alpha        float64 `gorm:"column:alpha"`
	Frame        int     `gorm:"frame"`
}

func getPixelApp(w http.ResponseWriter, r *http.Request) {
	menu_tmpl := template.Must(template.ParseFiles("html/pixelapp/index.html"))
	data := map[string]interface{}{
		"MenuBar": template.HTML(GetMenuBar()),
	}
	menu_tmpl.Execute(w, data)
}

func getNewPixelScreen(w http.ResponseWriter, r *http.Request) {
	menu_tmpl := template.Must(template.ParseFiles("html/pixelapp/setuppixel.html"))
	data := map[string]interface{}{
		"MenuBar": template.HTML(GetMenuBar()),
	}
	menu_tmpl.Execute(w, data)
}

func getNewPixelFromImageScreen(w http.ResponseWriter, r *http.Request) {
	menu_tmpl := template.Must(template.ParseFiles("html/pixelapp/importpixel.html"))
	data := map[string]interface{}{
		"MenuBar": template.HTML(GetMenuBar()),
	}
	menu_tmpl.Execute(w, data)
}

func getPixelAppScreen(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	pixelid := vars["pixelid"]

	db, err := GetDBConnection()
	if err != nil {
		fmt.Println("Failed to get db connection while getting pixel app")
		return
	}
	tableName := GetTableName("pixel")

	var pixel PixelAppImageDB
	if result := db.Table(tableName).First(&pixel, "guid=?", pixelid); result.Error != nil {
		fmt.Println("Failed to find image in db")
		fmt.Println(result.Error)
		return
	}

	menu_tmpl := template.Must(template.ParseFiles("html/pixelapp/pixel.html"))
	width := pixel.Width
	height := pixel.Height
	pw := pixel.PixelWidth
	if width == 0 {
		width = 200
	}
	if height == 0 {
		height = 500
	}
	if pw == 0 {
		pw = 20
	}

	// If we are in debug then setup a string
	randNum := rand.Intn(1000)
	outstr := fmt.Sprintf("?t=%d", randNum)
	//

	data := map[string]interface{}{
		"MenuBar":      template.HTML(GetPixelMenuBar()),
		"ToolBar":      template.HTML(GetPixelToolbar()),
		"RandomNumber": outstr,
		"PixelID":      pixelid,
		"Width":        width,
		"Height":       height,
		"PixelWidth":   pw,
		"PixelName":    pixel.Name,
	}
	menu_tmpl.Execute(w, data)
}

func getPixelRenderPage(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	pixelid := vars["pixelid"]

	db, err := GetDBConnection()
	if err != nil {
		fmt.Println("Failed to get db connection while getting pixel app")
		return
	}

	tableName := GetTableName("pixel")

	var pixel PixelAppImageDB
	if result := db.Table(tableName).First(&pixel, "guid=?", pixelid); result.Error != nil {
		fmt.Println("Failed to find image in db")
		fmt.Println(result.Error)
		return
	}

	menu_tmpl := template.Must(template.ParseFiles("html/pixelapp/render.html"))
	width := pixel.Width
	height := pixel.Height
	pw := pixel.PixelWidth
	if width == 0 {
		width = 200
	}
	if height == 0 {
		height = 500
	}
	if pw == 0 {
		pw = 20
	}
	data := map[string]interface{}{
		"MenuBar":    template.HTML(GetPixelRenderMenu()),
		"PixelID":    pixelid,
		"Width":      width,
		"Height":     height,
		"PixelWidth": pw,
		"PixelName":  pixel.Name,
	}
	menu_tmpl.Execute(w, data)
}

func getLoadPixelScreen(w http.ResponseWriter, r *http.Request) {
	menu_tmpl := template.Must(template.ParseFiles("html/pixelapp/saved.html"))
	data := map[string]interface{}{
		"MenuBar": template.HTML(GetMenuBar()),
	}
	menu_tmpl.Execute(w, data)
}

func getPixelappJS(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	filename := vars["filename"]
	http.ServeFile(w, r, "js/pixel/"+filename)
}

// NewPixelRequest - Request a new picture be created
type NewPixelRequest struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	Width       int    `json:"width"`
	Height      int    `json:"height"`
	PixelWidth  int    `json:"pixelwidth"`
}

// NewPixelResponse - Respond to new picture creation
type NewPixelResponse struct {
	Status  string `json:"status"`
	Message string `json:"message"`
	PixelID string `json:"pixelid"`
}

func postNewPixelApp(w http.ResponseWriter, r *http.Request) {
	var resp NewPixelResponse
	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var req NewPixelRequest
	err = json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		resp.Status = "fail"
		resp.Message = fmt.Sprintf("Failed with error: %s", err)
		json.NewEncoder(w).Encode(&resp)
		return
	}

	config := GetDBConfig()
	tableName := fmt.Sprintf("%spixel", config.TablePrepend)

	db, err := GetDBConnection()
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to connect to db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var count int64
	db.Table(tableName).Where("owner_user_id = ?", user.ID).Count(&count)

	if count >= int64(user.MaxNumberPixels-1) {
		resp.Status = "fail"
		resp.Message = "Exceeded number of Pixels, please upgrade or remove them"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	id := guuid.New().String()
	var newpixel PixelAppImageDB
	newpixel.OwnerUserID = user.ID
	newpixel.Name = req.Name
	newpixel.Description = req.Description
	newpixel.Width = req.Width
	newpixel.Height = req.Height
	newpixel.PixelWidth = req.PixelWidth

	newpixel.GUID = id
	if res := db.Table(tableName).Create(&newpixel); res.Error != nil {
		fmt.Println(res.Error)
		var resp NewPixelResponse
		resp.Status = "fail"
		resp.Message = "Failed to create new game"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	// Create picture details in db
	resp.Status = "ok"
	resp.PixelID = id
	json.NewEncoder(w).Encode(&resp)
}

func postNewPixelFromFile(w http.ResponseWriter, r *http.Request) {
	var resp NewPixelResponse
	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var req PixelFileDescriptor
	err = json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		resp.Status = "fail"
		resp.Message = fmt.Sprintf("Failed with error: %s", err)
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db, err := GetDBConnection()
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to connect to db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	config := GetDBConfig()
	id := guuid.New().String()

	var newpixel PixelAppImageDB
	newpixel.OwnerUserID = user.ID
	newpixel.Name = req.Name
	newpixel.Description = req.Description
	newpixel.Width = req.Width
	newpixel.Height = req.Height
	newpixel.PixelWidth = req.PixelWidth

	tableName := fmt.Sprintf("%spixel", config.TablePrepend)

	newpixel.GUID = id
	if res := db.Table(tableName).Create(&newpixel); res.Error != nil {
		fmt.Println(res.Error)
		resp.Status = "fail"
		resp.Message = "Failed to create new game"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	tableName = GetTableName("pixel_item")
	for _, p := range req.Pixels {
		var pix PixelItemDB
		pix.X = p.X
		pix.Y = p.Y
		pix.R = p.R
		pix.G = p.G
		pix.B = p.B
		pix.Alpha = p.Alpha
		pix.Layer = p.Layer
		pix.Frame = p.Frame
		pix.PixelImageID = newpixel.ID
		db.Table(tableName).Create(&pix)
	}

	tableName = GetTableName("pixel_shader")
	for _, p := range req.Shaders {
		var pix PixelShaderLayerDB
		pix.X = p.X
		pix.Y = p.Y
		pix.R = p.R
		pix.G = p.G
		pix.B = p.B
		pix.Alpha = p.Alpha
		pix.Frame = p.Frame
		pix.PixelImageID = newpixel.ID
		db.Table(tableName).Create(&pix)
	}
	db.Commit()

	resp.Status = "ok"
	resp.PixelID = newpixel.GUID
	json.NewEncoder(w).Encode(&resp)
}

// CheckPixelAppTables - create or update tables needed for pixel app
func CheckPixelAppTables() {
	db, err := GetDBConnection()

	if err != nil {
		fmt.Println("Failed to connect to db to check tables")
		return
	}

	config := GetDBConfig()
	prepend := config.TablePrepend

	tableName := fmt.Sprintf("%spixel", prepend)
	db.Table(tableName).AutoMigrate(&PixelAppImageDB{})

	tableName = fmt.Sprintf("%spixel_item", prepend)
	db.Table(tableName).AutoMigrate(&PixelItemDB{})
	db.Table(tableName).Migrator().AlterColumn(&PixelItemDB{}, "alpha")

	tableName = fmt.Sprintf("%spixel_shader", prepend)
	db.Table(tableName).AutoMigrate(&PixelShaderLayerDB{})
	db.Table(tableName).Migrator().AlterColumn(&PixelShaderLayerDB{}, "alpha")
}

// PixelDescriptor - the details of the pixel
type PixelDescriptor struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
	Width       int    `json:"width"`
	Height      int    `json:"height"`
	PixelWidth  int    `json:"pixelwidth"`
}

// SavedPixelResponse - response to get saved pixel list
type SavedPixelResponse struct {
	Status  string            `json:"status"`
	Message string            `json:"message"`
	Pixels  []PixelDescriptor `json:"pixels"`
}

func getSavedPixels(w http.ResponseWriter, r *http.Request) {
	var resp SavedPixelResponse
	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db, err := GetDBConnection()
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to connect to db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixels []PixelAppImageDB
	tableName := GetTableName("pixel")
	if res := db.Table(tableName).Find(&pixels, "owner_user_id=? AND deleted_at is NULL", user.ID); res.Error != nil {

		fmt.Println(res.Error)
		var resp NewPixelResponse
		resp.Status = "fail"
		resp.Message = "Failed to find any pixels"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	for _, p := range pixels {
		var nxt PixelDescriptor
		nxt.ID = p.GUID
		nxt.Name = p.Name
		nxt.Description = p.Description
		nxt.Width = p.Width
		nxt.Height = p.Height
		nxt.PixelWidth = p.PixelWidth
		resp.Pixels = append(resp.Pixels, nxt)
	}
	resp.Status = "ok"
	json.NewEncoder(w).Encode(&resp)
}

// PixelDataIndividual - an individual pixel
type PixelDataIndividual struct {
	X     int     `json:"x"`
	Y     int     `json:"y"`
	R     int     `json:"r"`
	G     int     `json:"g"`
	B     int     `json:"b"`
	Alpha float64 `json:"alpha"`
	Frame int     `json:"frame"`
	Layer int     `json:"layer"`
}

// PixelShaderIndividual - an individual pixel on the shader layer
type PixelShaderIndividual struct {
	X     int     `json:"x"`
	Y     int     `json:"y"`
	R     int     `json:"r"`
	G     int     `json:"g"`
	B     int     `json:"b"`
	Alpha float64 `json:"alpha"`
	Frame int     `json:"frame"`
}

// PixelDataRequest - post new set of pixel data
type PixelDataRequest struct {
	Guid    string                  `json:"guid"`
	Pixels  []PixelDataIndividual   `json:"pixels"`
	Shaders []PixelShaderIndividual `json:"shaders"`
}

type PixelCopyRequest struct {
	Guid string `json:"fromguid"`
	// TODO: Other bits required - like new name, new size etc
}

// PixelDataResponse - respond with status for updated pixel data
type PixelDataResponse struct {
	Status  string `json:"status"`
	Message string `json:"message"`
}

func postNewPixels(w http.ResponseWriter, r *http.Request) {
	var resp SavedPixelResponse
	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var req PixelDataRequest
	err = json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		resp.Status = "fail"
		resp.Message = fmt.Sprintf("Failed with error: %s", err)
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db, err := GetDBConnection()
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to connect to db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixel PixelAppImageDB
	tableName := GetTableName("pixel")
	if res := db.Table(tableName).First(&pixel, "guid=?", req.Guid); res.Error != nil {
		fmt.Println(res.Error)
		resp.Status = "fail"
		resp.Message = "Failed to find pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}
	if pixel.OwnerUserID != user.ID {
		resp.Status = "fail"
		resp.Message = "Unauthorized"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	tableName = GetTableName("pixel_item")
	// Remove all pixels before starting to save
	db.Table(tableName).Where("pixel_image_id=?", pixel.ID).Delete(&PixelItemDB{})
	for _, p := range req.Pixels {
		var pix PixelItemDB
		pix.X = p.X
		pix.Y = p.Y
		pix.R = p.R
		pix.G = p.G
		pix.B = p.B
		pix.Alpha = p.Alpha
		pix.Layer = p.Layer
		pix.Frame = p.Frame
		pix.PixelImageID = pixel.ID
		db.Table(tableName).Create(&pix)
	}

	tableName = GetTableName("pixel_shader")
	db.Table(tableName).Where("pixel_image_id=?", pixel.ID).Delete(&PixelShaderLayerDB{})
	for _, p := range req.Shaders {
		var pix PixelShaderLayerDB
		pix.X = p.X
		pix.Y = p.Y
		pix.R = p.R
		pix.G = p.G
		pix.B = p.B
		pix.Alpha = p.Alpha
		pix.Frame = p.Frame
		pix.PixelImageID = pixel.ID
		db.Table(tableName).Create(&pix)
	}
	db.Commit()
	resp.Status = "ok"
	json.NewEncoder(w).Encode(&resp)
}

func copyToNewPixels(w http.ResponseWriter, r *http.Request) {
	var resp SavedPixelResponse
	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var req PixelDataRequest
	err = json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		resp.Status = "fail"
		resp.Message = fmt.Sprintf("Failed with error: %s", err)
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db, err := GetDBConnection()
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to connect to db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixel PixelAppImageDB
	tableName := GetTableName("pixel")
	if res := db.Table(tableName).First(&pixel, "guid=?", req.Guid); res.Error != nil {
		fmt.Println(res.Error)
		resp.Status = "fail"
		resp.Message = "Failed to find pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}
	if pixel.OwnerUserID != user.ID {
		resp.Status = "fail"
		resp.Message = "Unauthorized"
		json.NewEncoder(w).Encode(&resp)
		return
	}
	resp.Status = "ok"
	json.NewEncoder(w).Encode(&resp)
}

// GetPixelResponse - get data on the current pixel data
type GetPixelResponse struct {
	Status  string                `json:"status"`
	Message string                `json:"message"`
	Pixels  []PixelDataIndividual `json:"pixels"`
}

func getPixels(w http.ResponseWriter, r *http.Request) {
	var resp GetPixelResponse
	vars := mux.Vars(r)
	pixelid := vars["pixelid"]
	if pixelid == "" {
		resp.Status = "fail"
		resp.Message = "No id given"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db, err := GetDBConnection()
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to connect to db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixel PixelAppImageDB
	tableName := GetTableName("pixel")
	if res := db.Table(tableName).First(&pixel, "guid=?", pixelid); res.Error != nil {
		// fmt.Println(res.Error)
		resp.Status = "fail"
		resp.Message = "Failed to find pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	if pixel.OwnerUserID != user.ID {
		resp.Status = "fail"
		resp.Message = "Failed to find user pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixelList []PixelItemDB
	tableName = GetTableName("pixel_item")
	if res := db.Table(tableName).Find(&pixelList, "pixel_image_id=?", pixel.ID); res.Error != nil {
		resp.Status = "ok"
		// No image data to load
		json.NewEncoder(w).Encode(&resp)
		return
	}

	for _, p := range pixelList {
		var nxt PixelDataIndividual
		nxt.X = p.X
		nxt.Y = p.Y
		nxt.R = p.R
		nxt.G = p.G
		nxt.B = p.B
		nxt.Alpha = p.Alpha
		nxt.Layer = p.Layer
		nxt.Frame = p.Frame
		// DEBUG
		// fmt.Printf("%d: %d,%d  (%d, %d, %d)\n", p.ID, p.X, p.Y, p.R, p.G, p.B)
		resp.Pixels = append(resp.Pixels, nxt)
	}

	resp.Status = "ok"
	json.NewEncoder(w).Encode(&resp)
}

type PixelFileDescriptor struct {
	Name        string                `json:"name"`
	Description string                `json:"description"`
	Width       int                   `json:"width"`
	Height      int                   `json:"height"`
	PixelWidth  int                   `json:"pixelwidth"`
	Pixels      []PixelDataIndividual `json:"pixels"`
	Shaders     []PixelDataIndividual `json:"shaders"`
}

func getPixelsAsFile(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	pixelid := vars["pixelid"]
	if pixelid == "" {
		var resp GetPixelResponse
		resp.Status = "fail"
		resp.Message = "No id given"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		var resp GetPixelResponse
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		var resp GetPixelResponse
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db, err := GetDBConnection()
	if err != nil {
		var resp GetPixelResponse
		resp.Status = "fail"
		resp.Message = "Failed to connect to db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixel PixelAppImageDB
	tableName := GetTableName("pixel")
	if res := db.Table(tableName).First(&pixel, "guid=?", pixelid); res.Error != nil {
		var resp GetPixelResponse
		resp.Status = "fail"
		resp.Message = "Failed to find pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	if pixel.OwnerUserID != user.ID {
		var resp GetPixelResponse
		resp.Status = "fail"
		resp.Message = "Failed to find user pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixelList []PixelItemDB
	tableName = GetTableName("pixel_item")
	if res := db.Table(tableName).Find(&pixelList, "pixel_image_id=?", pixel.ID); res.Error != nil {
		var resp GetPixelResponse
		resp.Status = "ok"
		// No image data to load
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var ret PixelFileDescriptor
	ret.Name = pixel.Name
	ret.Description = pixel.Description
	ret.Width = pixel.Width
	ret.Height = pixel.Height
	ret.PixelWidth = pixel.PixelWidth

	for _, p := range pixelList {
		var nxt PixelDataIndividual
		nxt.X = p.X
		nxt.Y = p.Y
		nxt.R = p.R
		nxt.G = p.G
		nxt.B = p.B
		nxt.Alpha = p.Alpha
		nxt.Layer = p.Layer
		nxt.Frame = p.Frame
		// DEBUG
		// fmt.Printf("%d: %d,%d  (%d, %d, %d)\n", p.ID, p.X, p.Y, p.R, p.G, p.B)
		ret.Pixels = append(ret.Pixels, nxt)
	}

	var shaders []PixelShaderLayerDB
	tablename := GetTableName("pixel_shader")
	db.Table(tablename).Where("pixel_image_id=?", pixel.ID).Find(&shaders)

	for _, shader := range shaders {
		var nxt PixelDataIndividual
		nxt.X = shader.X
		nxt.Y = shader.Y
		nxt.R = shader.R
		nxt.G = shader.G
		nxt.B = shader.B
		nxt.Alpha = shader.Alpha
		nxt.Frame = shader.Frame
		// DEBUG
		// fmt.Printf("%d: %d,%d  (%d, %d, %d)\n", p.ID, p.X, p.Y, p.R, p.G, p.B)
		ret.Shaders = append(ret.Shaders, nxt)
	}

	json.NewEncoder(w).Encode(&ret)
}

// DeletePixelResponse - for responding that pixel has been deleted
type DeletePixelResponse struct {
	Status  string `json:"status"`
	Message string `json:"message"`
}

func deletePixel(w http.ResponseWriter, r *http.Request) {
	var resp GetPixelResponse
	vars := mux.Vars(r)
	pixelid := vars["pixelid"]
	if pixelid == "" {
		resp.Status = "fail"
		resp.Message = "No id given"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db, err := GetDBConnection()
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to connect to db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixel PixelAppImageDB
	tableName := GetTableName("pixel")
	if res := db.Table(tableName).First(&pixel, "guid=?", pixelid); res.Error != nil {
		// fmt.Println(res.Error)
		resp.Status = "fail"
		resp.Message = "Failed to find pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	if pixel.OwnerUserID != user.ID {
		resp.Status = "fail"
		resp.Message = "Failed to find user pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db.Table(tableName).Delete(&pixel)
	resp.Status = "ok"
	json.NewEncoder(w).Encode(&resp)
}

// GetShaderResponse - get the details of the shader layer
type GetShaderResponse struct {
	Status  string                  `json:"status"`
	Message string                  `json:"message"`
	Shaders []PixelShaderIndividual `json:"shaders"`
}

func getPixelShader(w http.ResponseWriter, r *http.Request) {
	// PixelShaderIndividual
	var resp GetShaderResponse
	vars := mux.Vars(r)
	pixelid := vars["pixelid"]
	if pixelid == "" {
		resp.Status = "fail"
		resp.Message = "No id given"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db, err := GetDBConnection()
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to connect to db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixel PixelAppImageDB
	tableName := GetTableName("pixel")
	if res := db.Table(tableName).First(&pixel, "guid=?", pixelid); res.Error != nil {
		// fmt.Println(res.Error)
		resp.Status = "fail"
		resp.Message = "Failed to find pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	if pixel.OwnerUserID != user.ID {
		resp.Status = "fail"
		resp.Message = "Failed to find user pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixelList []PixelItemDB
	tableName = GetTableName("pixel_shader")
	if res := db.Table(tableName).Find(&pixelList, "pixel_image_id=?", pixel.ID); res.Error != nil {
		resp.Status = "ok"
		// No image data to load
		json.NewEncoder(w).Encode(&resp)
		return
	}

	for _, p := range pixelList {
		var nxt PixelShaderIndividual
		nxt.X = p.X
		nxt.Y = p.Y
		nxt.R = p.R
		nxt.G = p.G
		nxt.B = p.B
		nxt.Alpha = p.Alpha
		nxt.Frame = p.Frame
		// DEBUG
		// fmt.Printf("%d: %d,%d  (%d, %d, %d): %f\n", p.ID, p.X, p.Y, p.R, p.G, p.B, p.Alpha)
		resp.Shaders = append(resp.Shaders, nxt)
	}

	resp.Status = "ok"
	json.NewEncoder(w).Encode(&resp)
}

// UpdatePixelRequest - request change of size
type UpdatePixelRequest struct {
	Width  int `json:"width"`
	Height int `json:"height"`
}

// UpdatePixelResponse - to respond to an update pixel request
type UpdatePixelResponse struct {
	Status  string `json:"status"`
	Message string `json:"message"`
}

func updatePixelSize(w http.ResponseWriter, r *http.Request) {
	var resp UpdatePixelResponse
	vars := mux.Vars(r)
	pixelid := vars["pixelid"]
	if pixelid == "" {
		resp.Status = "fail"
		resp.Message = "No id given"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var req UpdatePixelRequest
	err := json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		resp.Status = "fail"
		resp.Message = fmt.Sprintf("Failed with error: %s", err)
		json.NewEncoder(w).Encode(&resp)
		return
	}

	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db, err := GetDBConnection()
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to connect to db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixel PixelAppImageDB
	tableName := GetTableName("pixel")
	if res := db.Table(tableName).First(&pixel, "guid=?", pixelid); res.Error != nil {
		// fmt.Println(res.Error)
		resp.Status = "fail"
		resp.Message = "Failed to find pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	if pixel.OwnerUserID != user.ID {
		resp.Status = "fail"
		resp.Message = "Pixel not owned by user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	if req.Width == 0 && req.Height == 0 {
		resp.Status = "fail"
		resp.Message = "No new size information"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	if req.Width != 0 {
		db.Table(tableName).Model(&pixel).Update("width", req.Width)
	}
	if req.Height != 0 {
		db.Table(tableName).Model(&pixel).Update("height", req.Height)
	}
	resp.Status = "ok"
	json.NewEncoder(w).Encode(&resp)
}

// DoublePixelDensityRequest - a post request to double pixel density
type DoublePixelDensityRequest struct {
	NewImageName   string `json:"newimagename"`
	MultiplyFactor int    `json:"multiplyingfactor"`
}

// DoublePixelDensityResponse - response with status and new guid of created image
type DoublePixelDensityResponse struct {
	Status  string `json:"status"`
	Message string `json:"message"`
	GUID    string `json:"guid"`
}

func doublePixelDensity(w http.ResponseWriter, r *http.Request) {
	var resp DoublePixelDensityResponse
	vars := mux.Vars(r)
	pixelid := vars["pixelid"]
	if pixelid == "" {
		resp.Status = "fail"
		resp.Message = "No id given"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var req DoublePixelDensityRequest
	err := json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		resp.Status = "fail"
		resp.Message = fmt.Sprintf("Failed with error: %s", err)
		json.NewEncoder(w).Encode(&resp)
		return
	}

	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	db, err := GetDBConnection()
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to connect to db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixel PixelAppImageDB
	tableName := GetTableName("pixel")
	if res := db.Table(tableName).First(&pixel, "guid=?", pixelid); res.Error != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find pixel"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	if pixel.OwnerUserID != user.ID {
		resp.Status = "fail"
		resp.Message = "Pixel not owned by user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	// Check max number not exceeded
	var count int64
	db.Table(tableName).Where("owner_user_id = ? and deleted_at is not null", user.ID).Count(&count)

	if count >= int64(user.MaxNumberPixels-1) {
		resp.Status = "fail"
		resp.Message = "Exceeded number of Pixels, please upgrade or remove them"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	// Create new image
	id := guuid.New().String()
	var newpixel PixelAppImageDB
	newpixel.OwnerUserID = user.ID
	var newName string
	if strings.TrimSpace(req.NewImageName) != "" {
		newName = strings.TrimSpace(req.NewImageName)
	} else {
		newName = fmt.Sprintf("%s x2", pixel.Name)
	}
	newpixel.Name = newName
	newpixel.Description = pixel.Description
	newpixel.Width = pixel.Width
	newpixel.Height = pixel.Height
	// Check doubling doesn't break the width/height requirements

	if pixel.Width%2 != 0 {
		resp.Status = "fail"
		resp.Message = "Cannot make pixel width any smaller - already at min"
		json.NewEncoder(w).Encode(&resp)
		return
	}
	newpixel.PixelWidth = pixel.PixelWidth / 2
	newpixel.GUID = id

	if res := db.Table(tableName).Create(&newpixel); res.Error != nil {
		fmt.Println(res.Error)
		var resp NewPixelResponse
		resp.Status = "fail"
		resp.Message = "Failed to create new game"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	// Read in old image pixels
	var pixelList []PixelItemDB
	tableName = GetTableName("pixel_item")
	if res := db.Table(tableName).Find(&pixelList, "pixel_image_id=?", pixel.ID); res.Error != nil {
		resp.Status = "ok"
		// No image data to load
		json.NewEncoder(w).Encode(&resp)
		return
	}

	// Create new ones against the new image
	for _, p := range pixelList {
		// Top Left
		var nxt PixelItemDB
		nxt.PixelImageID = newpixel.ID
		nxt.X = (p.X * 2)
		nxt.Y = (p.Y * 2)
		nxt.R = p.R
		nxt.G = p.G
		nxt.B = p.B
		nxt.Alpha = p.Alpha
		nxt.Layer = p.Layer
		nxt.Frame = p.Frame
		db.Table(tableName).Create(&nxt)

		// Top Right
		var nxt2 PixelItemDB
		nxt2.PixelImageID = newpixel.ID
		nxt2.X = (p.X * 2) + 1
		nxt2.Y = p.Y * 2
		nxt2.R = p.R
		nxt2.G = p.G
		nxt2.B = p.B
		nxt2.Alpha = p.Alpha
		nxt2.Layer = p.Layer
		nxt2.Frame = p.Frame
		db.Table(tableName).Create(&nxt2)

		// Bottom left
		var nxt3 PixelItemDB
		nxt3.PixelImageID = newpixel.ID
		nxt3.X = p.X * 2
		nxt3.Y = (p.Y * 2) + 1
		nxt3.R = p.R
		nxt3.G = p.G
		nxt3.B = p.B
		nxt3.Alpha = p.Alpha
		nxt3.Layer = p.Layer
		nxt3.Frame = p.Frame
		db.Table(tableName).Create(&nxt3)

		// Bottom Right
		var nxt4 PixelItemDB
		nxt4.PixelImageID = newpixel.ID
		nxt4.X = (p.X * 2) + 1
		nxt4.Y = (p.Y * 2) + 1
		nxt4.R = p.R
		nxt4.G = p.G
		nxt4.B = p.B
		nxt4.Alpha = p.Alpha
		nxt4.Layer = p.Layer
		nxt4.Frame = p.Frame
		db.Table(tableName).Create(&nxt4)
	}

	resp.Status = "ok"
	resp.GUID = newpixel.GUID
	json.NewEncoder(w).Encode(&resp)
}

// RenderImageResponse - Responsd to request to render image
type RenderImageResponse struct {
	Status  string `json:"status"`
	Message string `json:"message"`
}

// Return rendered image - or 404 if not exists
func getRenderedImage(w http.ResponseWriter, r *http.Request) {
	tokenCookie, err := r.Cookie("ecs_gsv_token")
	if err != nil {
		w.Write([]byte("Failed to get token"))
		return
	}
	token := strings.Replace(tokenCookie.Value, "ecs_gsv_token=", "", 1)
	// fmt.Println(token)
	user, err := GetUserFromTokenString(token)
	if err != nil {
		fmt.Println(err)
		// TODO: return 404 page
		return
	}

	vars := mux.Vars(r)
	pixelid := vars["pixelid"]
	filetype := vars["filetype"]

	db, err := GetDBConnection()
	if err != nil {
		fmt.Println("Failed to get db connection while getting pixel app")
		return
	}
	tableName := GetTableName("pixel")

	// Sort out any color substitutions from query param
	var colMap map[string]string
	colMap = map[string]string{}
	values := r.URL.Query()
	for k, v := range values {
		colMap[k] = v[0]
	}

	var pixel PixelAppImageDB
	if result := db.Table(tableName).First(&pixel, "guid=?", pixelid); result.Error != nil {
		fmt.Println("Failed to find image in db")
		fmt.Println(result.Error)
		return
	}

	if pixel.OwnerUserID != user.ID {
		fmt.Printf("User not authorised %d\n", user.ID)
		return
	}
	tablename := GetTableName("pixel_item")

	var frames []int

	db.Distinct("frame").Table(tablename).Where("pixel_image_id=?", pixel.ID).Find(&frames)
	if len(frames) == 0 {
		fmt.Println("No frames exist")
		return
	}

	// DEBUG
	// var pixels []PixelItemDB
	// tablename = GetTableName("pixel_item")
	// db.Table(tablename).Where("pixel_image_id=?", pixel.ID).Find(&pixels)
	// for _, p := range pixels {
	// 	fmt.Println(p.Frame)
	// }
	// END DEBUG

	// TOOD: Use chose frame from db
	fullPath := fmt.Sprintf("user/%s/%s_0.%s", filetype, pixelid, filetype)

	if os.Stat(fullPath); !os.IsNotExist(err) {
		// Render image
		for _, frame := range frames {
			if filetype == "png" {
				if RenderPngImage(db, pixel, frame, "forward", 0, false, colMap) != nil {
					fmt.Println("ERROR rendering image")
					return
				}
			} else if filetype == "spritesheet" {
				if RenderSpriteSheetImage(db, pixel, colMap) != nil {
					fmt.Println("ERROR rendering image")
					return
				}
				spriteSheetPath := fmt.Sprintf("user/png/%s_spritesheet.png", pixel.GUID)
				http.ServeFile(w, r, spriteSheetPath)
				return
			} else if filetype == "gif" {
				if CreateGif(db, pixel) != nil {
					fmt.Println("ERROR Rendering gif")
					return
				} else {
					fullPath = fmt.Sprintf("user/%s/%s.%s", filetype, pixelid, filetype)
				}
			}
		}
	}

	http.ServeFile(w, r, fullPath)
}

func getRenderedFrame(w http.ResponseWriter, r *http.Request) {
	tokenCookie, err := r.Cookie("ecs_gsv_token")
	if err != nil {
		w.Write([]byte("Failed to get token"))
		return
	}
	token := strings.Replace(tokenCookie.Value, "ecs_gsv_token=", "", 1)

	user, err := GetUserFromTokenString(token)
	if err != nil {
		fmt.Println(err)
		// TODO: return 404 page
		return
	}

	vars := mux.Vars(r)
	pixelid := vars["pixelid"]
	frame, _ := strconv.Atoi(vars["frame"])
	modifier := vars["modifier"]
	angle, _ := strconv.Atoi(vars["angle"])
	flip := vars["flip"]

	db, err := GetDBConnection()
	if err != nil {
		fmt.Println("Failed to get db connection while getting pixel app")
		return
	}
	tableName := GetTableName("pixel")

	var pixel PixelAppImageDB
	if result := db.Table(tableName).First(&pixel, "guid=?", pixelid); result.Error != nil {
		fmt.Println("Failed to find image in db")
		fmt.Println(result.Error)
		return
	}

	if pixel.OwnerUserID != user.ID {
		fmt.Printf("User not authorised %d\n", user.ID)
		return
	}
	tablename := GetTableName("pixel_item")

	var frames []int

	db.Distinct("frame").Table(tablename).Where("pixel_image_id=?", pixel.ID).Find(&frames)
	if len(frames) == 0 {
		fmt.Println("No frames exist")
		return
	}
	bFlip := (flip == "true")

	// Sort out any color substitutions from query param
	var colMap map[string]string
	colMap = map[string]string{}
	values := r.URL.Query()
	for k, v := range values {
		colMap[k] = v[0]
	}

	if RenderPngImage(db, pixel, frame, modifier, angle, bFlip, colMap) != nil {
		fmt.Println("ERROR rendering image")
		return
	}

	fullPath := fmt.Sprintf("user/png/%s_%d.png", pixelid, frame)
	http.ServeFile(w, r, fullPath)
}

//CheckPixelFolders - create if not exist the folders for output
func CheckPixelFolders() {
	path := "user"
	if _, err := os.Stat(path); os.IsNotExist(err) {
		fmt.Printf("Making folder %s\n", path)
		os.Mkdir(path, 0755)
	}
	path = "user/png"
	if _, err := os.Stat(path); os.IsNotExist(err) {
		fmt.Printf("Making folder %s\n", path)
		os.Mkdir(path, 0755)
	}
	path = "user/jpg"
	if _, err := os.Stat(path); os.IsNotExist(err) {
		fmt.Printf("Making folder %s\n", path)
		os.Mkdir(path, 0755)
	}
	path = "user/gif"
	if _, err := os.Stat(path); os.IsNotExist(err) {
		fmt.Printf("Making folder %s\n", path)
		os.Mkdir(path, 0755)
	}
	path = "user/import"
	if _, err := os.Stat(path); os.IsNotExist(err) {
		fmt.Printf("Making folder %s\n", path)
		os.Mkdir(path, 0755)
	}
}

// PixelInformationResponse - A list of the information about a pixel
type PixelInformationResponse struct {
	Status      string   `json:"status"`
	Message     string   `json:"message"`
	FrameCount  uint     `json:"framecount"`
	Description string   `json:"description"`
	Colors      []string `json:"colors"`
}

func getPixelInformation(w http.ResponseWriter, r *http.Request) {
	var resp PixelInformationResponse
	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	vars := mux.Vars(r)
	pixelid := vars["pixelid"]

	db, err := GetDBConnection()
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to connect to database"
		json.NewEncoder(w).Encode(&resp)
		return
	}
	tableName := GetTableName("pixel")

	var pixel PixelAppImageDB
	if result := db.Table(tableName).First(&pixel, "guid=?", pixelid); result.Error != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find image in db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	if pixel.OwnerUserID != user.ID {
		resp.Status = "fail"
		resp.Message = "Failed to find image in db"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var topframe PixelItemDB
	tableName = GetTableName("pixel_item")
	if result := db.Table(tableName).Where("pixel_image_id = ?", pixel.ID).Order("frame desc").First(&topframe); result.Error != nil {
		// No result - no frames, which is ok
		resp.Status = "ok"
		resp.Description = pixel.Description
		resp.FrameCount = 0
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var pixels []PixelItemDB
	if result := db.Table(tableName).Where("pixel_image_id=?", pixel.ID).Find(&pixels); result.Error != nil {
		resp.Status = "fail"
		resp.Message = "Failed to get hold of pixels"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	for _, p := range pixels {
		// Make a hex color from rgb values (decimal)
		col := GetHexColorFromRGB(p.R, p.G, p.B)

		// Don't include transparent colours
		if p.Alpha > 0 {
			bFoundCol := false
			for _, t := range resp.Colors {
				if t == col {
					bFoundCol = true
				}
			}
			if !bFoundCol {
				resp.Colors = append(resp.Colors, col)
			}
		}
	}

	resp.Status = "ok"
	resp.Description = pixel.Description
	resp.FrameCount = uint(topframe.Frame + 1)
	json.NewEncoder(w).Encode(&resp)
}

// Respond with the uploaded image id
type ImportImage struct {
	Status  string `json:"status"`
	Message string `json:"message"`
	ImageId string `json:"imageid"`
}

const MAX_UPLOAD_SIZE = 2 * 1024 * 1024 // 2MB

// Allow uploading of imported image
func postNewImportImage(w http.ResponseWriter, r *http.Request) {
	var resp ImportImage
	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	path := fmt.Sprintf("user/import/%d", user.ID)
	if _, err := os.Stat(path); os.IsNotExist(err) {
		fmt.Printf("Making folder for user %s\n", path)
		os.Mkdir(path, 0755)
	}

	r.Body = http.MaxBytesReader(w, r.Body, MAX_UPLOAD_SIZE)
	if err := r.ParseMultipartForm(MAX_UPLOAD_SIZE); err != nil {
		resp.Status = "fail"
		resp.Message = "The uploaded file is too big. Please choose an file that's less than 2MB in size"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	file, fileHeader, err := r.FormFile("pixel_image")
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Error decoding file"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	defer file.Close()
	ext := filepath.Ext(fileHeader.Filename)
	if ext != ".png" {
		resp.Status = "fail"
		resp.Message = "File must be of type PNG"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	filename := GenerateRandomString(12)
	fullpath := fmt.Sprintf("%s/%s.png", path, filename)

	dst, err := os.Create(fullpath)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to create file on server"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	defer dst.Close()

	// Copy the uploaded file to the filesystem
	// at the specified destination
	_, err = io.Copy(dst, file)
	if err != nil {
		// http.Error(w, err.Error(), http.StatusInternalServerError)
		resp.Status = "fail"
		resp.Message = "Failed to encode file on server"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	resp.Status = "ok"
	resp.ImageId = filename
	json.NewEncoder(w).Encode(&resp)
}

// Retrieve the previously stored
func getImportedImage(w http.ResponseWriter, r *http.Request) {
	tokenCookie, err := r.Cookie("ecs_gsv_token")
	if err != nil {
		w.Write([]byte("Failed to get token"))
		return
	}
	token := strings.Replace(tokenCookie.Value, "ecs_gsv_token=", "", 1)

	user, err := GetUserFromTokenString(token)
	if err != nil {
		w.WriteHeader(404)
		fmt.Fprint(w, "Image not found #err 21953")
		return
	}

	vars := mux.Vars(r)
	imageId := vars["image_id"]

	// Get the image associated with the call
	// But check it belongs to the user first
	path := fmt.Sprintf("user/import/%d", user.ID)
	if _, err := os.Stat(path); os.IsNotExist(err) {
		w.WriteHeader(404)
		fmt.Fprint(w, "Image not found")
		return
	}

	path = fmt.Sprintf("user/import/%d/%s.png", user.ID, imageId)
	http.ServeFile(w, r, path)
}

// ImportedImageDetails - details about the image sent
type ImportedImageDetails struct {
	Status  string `json:"status"`
	Message string `json:"message"`
	Width   int    `json:"width"`
	Height  int    `json:"height"`
}

func getImportedImageDetails(w http.ResponseWriter, r *http.Request) {
	var resp ImportedImageDetails
	reqToken := r.Header.Get("Authorization")
	if reqToken == "" {
		resp.Status = "fail"
		resp.Message = "missing bearer token"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	user, err := GetUserFromToken(reqToken)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "Failed to find user"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	path := fmt.Sprintf("user/import/%d", user.ID)
	if _, err := os.Stat(path); os.IsNotExist(err) {
		resp.Status = "fail"
		resp.Message = "Folder doesn't exist"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	vars := mux.Vars(r)
	imageId := vars["image_id"]

	filepath := fmt.Sprintf("%s/%s.png", path, imageId)

	f, err := os.Open(filepath)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "File doesn't exist"
		json.NewEncoder(w).Encode(&resp)
		return
	}
	defer f.Close()

	image, _, err := image.Decode(f)
	resp.Status = "ok"
	resp.Width = image.Bounds().Max.X
	resp.Height = image.Bounds().Max.Y
	json.NewEncoder(w).Encode(&resp)
}

// RenderImportImageRequest - The request with information about import image
type RenderImportImageRequest struct {
	Name         string `json:"name"`
	Description  string `json:"description"`
	TargetWidth  int    `json:"targetwidth"`
	TargetHeight int    `json:"targetheight"`
	PictureID    string `json:"picid"`
	PixelWidth   int    `json:"pixelwidth"`
	StartX       int    `json:"startx"`
	StartY       int    `json:"starty"`
	EndX         int    `json:"endx"`
	EndY         int    `json:"endy"`
}

// RenderImportImageResponse - Response to the import render
type RenderImportImageResponse struct {
	Status  string `json:"status"`
	Message string `json:"message"`
	PixelID string `json:"pixelid"`
}

func renderNewImportImage(w http.ResponseWriter, r *http.Request) {
	var resp RenderImportImageResponse
	tokenCookie, err := r.Cookie("ecs_gsv_token")
	if err != nil {
		resp.Status = "fail"
		resp.Message = "token missing"
		json.NewEncoder(w).Encode(&resp)
		return
	}
	token := strings.Replace(tokenCookie.Value, "ecs_gsv_token=", "", 1)

	user, err := GetUserFromTokenString(token)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "user missing"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	path := fmt.Sprintf("user/import/%d", user.ID)
	if _, err := os.Stat(path); os.IsNotExist(err) {
		resp.Status = "fail"
		resp.Message = "Folder doesn't exist"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	var req RenderImportImageRequest
	err = json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		resp.Status = "fail"
		resp.Message = fmt.Sprintf("Failed with error: %s", err)
		json.NewEncoder(w).Encode(&resp)
		return
	}

	path = fmt.Sprintf("user/import/%d/%s.png", user.ID, req.PictureID)
	if _, err := os.Stat(path); os.IsNotExist(err) {
		resp.Status = "fail"
		resp.Message = "File doesn't exist"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	if req.PixelWidth < 1 {
		resp.Status = "fail"
		resp.Message = "Pixel width is too small"
		json.NewEncoder(w).Encode(&resp)
		return
	}

	f, err := os.Open(path)
	if err != nil {
		resp.Status = "fail"
		resp.Message = "File doesn't exist"
		json.NewEncoder(w).Encode(&resp)
		return
	}
	defer f.Close()

	var newPixelImageDB PixelAppImageDB
	newPixelImageDB.Name = req.Name
	newPixelImageDB.Description = req.Description

	numPixX := int((req.EndX - req.StartX) / req.PixelWidth)
	numPixY := int((req.EndY - req.StartY) / req.PixelWidth)
	newPixelImageDB.Height = numPixY * req.PixelWidth
	newPixelImageDB.Width = numPixX * req.PixelWidth

	newPixelImageDB.PixelWidth = req.PixelWidth
	newPixelImageDB.OwnerUserID = user.ID
	id := guuid.New().String()
	newPixelImageDB.GUID = id

	db, err := GetDBConnection()
	if err != nil {
		fmt.Println("Failed to get db connection while getting pixel app")
		return
	}

	tableName := GetTableName("pixel")
	db.Table(tableName).Create(&newPixelImageDB)

	image, _, err := image.Decode(f)

	county := 0
	tableName = GetTableName("pixel_item")
	for y := req.StartY; y < req.TargetHeight; y += req.PixelWidth {
		countx := 0
		for x := req.StartX; x < req.TargetHeight; x += req.PixelWidth {
			var nxt PixelItemDB
			nxt.PixelImageID = newPixelImageDB.ID
			nxt.X = countx
			nxt.Y = county
			nxt.Alpha = 1

			var redTotal int
			var greenTotal int
			var blueTotal int
			var count int

			for dy := y; dy < y+req.PixelWidth; dy++ {
				for dx := x; dx < x+req.PixelWidth; dx++ {
					pixel := image.At(dx, dy)

					ar, ag, ab, aa := pixel.RGBA()

					if aa > 0 {
						redTotal += (int(ar) >> 8)
						greenTotal += (int(ag) >> 8)
						blueTotal += (int(ab) >> 8)
						count += 1
					}
				}
			}

			if count > 0 {
				nxt.R = int(redTotal / count)
				nxt.G = int(greenTotal / count)
				nxt.B = int(blueTotal / count)
				nxt.Layer = 0
				nxt.Frame = 0
				db.Table(tableName).Create(&nxt)
			}

			countx += 1
		}
		county += 1
	}

	resp.PixelID = newPixelImageDB.GUID
	resp.Status = "ok"
	json.NewEncoder(w).Encode(&resp)
}

// AddPixelAppRoutes - add in the routes for the pixel app
func AddPixelAppRoutes(r *mux.Router) error {
	CheckPixelAppTables()
	CheckPixelFolders()
	// Pages
	r.HandleFunc("/pixelapp", getPixelApp)
	r.HandleFunc("/pixelapp/new", getNewPixelScreen)
	r.HandleFunc("/pixelapp/newfromimage", getNewPixelFromImageScreen)
	r.HandleFunc("/pixelapp/load", getLoadPixelScreen)
	r.HandleFunc("/pixelapp/{pixelid}", getPixelAppScreen)
	r.HandleFunc("/pixelapp/render/{pixelid}", getPixelRenderPage)

	r.HandleFunc("/js/pixelapp/{filename}", getPixelappJS)
	r.HandleFunc("/img/pixelapp/{pixelid}/{filetype}", getRenderedImage).Methods("GET")
	r.HandleFunc("/img/pixelapp/render/{pixelid}/{frame}/{modifier}/{angle}/{flip}", getRenderedFrame).Methods("GET")
	r.HandleFunc("/pixelapp/importimage/{image_id}", getImportedImage).Methods("GET")

	// API
	r.HandleFunc("/api/pixelapp/saved", getSavedPixels).Methods("GET")
	r.HandleFunc("/api/pixelapp/new", postNewPixelApp).Methods("POST")
	r.HandleFunc("/api/pixelapp/newfromfile", postNewPixelFromFile).Methods("POST")

	r.HandleFunc("/api/pixelapp/save", postNewPixels).Methods("POST")
	r.HandleFunc("/api/pixelapp/copy", copyToNewPixels).Methods("POST")
	r.HandleFunc("/api/pixelapp/{pixelid}", getPixels).Methods("GET")
	r.HandleFunc("/api/pixelapp/download/{pixelid}", getPixelsAsFile).Methods("GET")
	r.HandleFunc("/api/pixelapp/{pixelid}", deletePixel).Methods("DELETE")
	r.HandleFunc("/api/pixelapp/shader/{pixelid}", getPixelShader).Methods("GET")
	r.HandleFunc("/api/pixelapp/size/{pixelid}", updatePixelSize).Methods("POST")
	r.HandleFunc("/api/pixelapp/double/{pixelid}", doublePixelDensity).Methods("POST")
	r.HandleFunc("/api/pixelapp/info/{pixelid}", getPixelInformation).Methods("GET")

	r.HandleFunc("/api/pixelapp/importimage", postNewImportImage).Methods("POST")
	r.HandleFunc("/api/pixelapp/importimage/{image_id}", getImportedImageDetails).Methods("GET")
	r.HandleFunc("/api/pixelapp/import/finalize", renderNewImportImage).Methods("POST")

	return nil
}
