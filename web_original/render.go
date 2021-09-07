package main

import (
	"errors"
	"fmt"
	"image"
	"image/color"
	"image/gif"
	"image/png"
	"os"

	"github.com/disintegration/imaging"
	"gorm.io/gorm"
)

// RenderPngImage - Renders a single frame - modifier can be both, reverse, forward - angle is in degrees
// Currently only accepts 0,90,180,270 as values
func RenderPngImage(db *gorm.DB, pixel PixelAppImageDB, frame int, modifier string, angle int, bFlip bool, colMap map[string]string) error {
	// TODO: Remove this so we can respect all values
	if angle != 0 && angle != 90 && angle != 180 && angle != 270 {
		return errors.New("Angle must be 0, 90, 180 or 270")
	}

	width := pixel.Width
	height := pixel.Height

	upLeft := image.Point{0, 0}
	lowRight := image.Point{width, height}
	if modifier == "both" {
		lowRight.Y *= 2
	}
	img := image.NewRGBA(image.Rectangle{upLeft, lowRight})

	var pixels []PixelItemDB

	tablename := GetTableName("pixel_item")
	db.Table(tablename).Where("pixel_image_id=?", pixel.ID).Find(&pixels)
	pixSize := pixel.PixelWidth

	for _, pix := range pixels {

		if pix.Frame != frame {
			continue
		}

		// alpha := uint8(pix.Alpha * 255)
		// color := color.NRGBA{uint8(pix.R), uint8(pix.G), uint8(pix.B), alpha}
		alpha := uint8(pix.Alpha * 255)
		colString := GetHexColorFromRGB(pix.R, pix.G, pix.B)
		var color color.NRGBA
		color.A = alpha
		if val, ok := colMap[colString]; ok {
			//do something here
			red, green, blue := GetColorFromHex(val)
			// color = color.NRGBA{uint8(red), uint8(green), uint8(blue), alpha}
			color.R = uint8(red)
			color.G = uint8(green)
			color.B = uint8(blue)
		} else {
			color.R = uint8(pix.R)
			color.G = uint8(pix.G)
			color.B = uint8(pix.B)
			//  = color.NRGBA{uint8(pix.R), uint8(pix.G), uint8(pix.B), alpha}
		}

		if modifier == "forward" || modifier == "both" {
			for y := 0; y < pixSize; y++ {
				for x := 0; x < pixSize; x++ {
					nxtX := (pix.X * pixSize) + x
					nxtY := (pix.Y * pixSize) + y
					img.Set(nxtX, nxtY, color)
				}
			}
		} else if modifier == "reverse" {
			for y := 0; y < pixSize; y++ {
				for x := 0; x < pixSize; x++ {
					nxtX := (width - pixSize) - (pix.X * pixSize) + x
					nxtY := (pix.Y * pixSize) + y
					img.Set(nxtX, nxtY, color)
				}
			}
		}

		if modifier == "both" {
			for y := 0; y < pixSize; y++ {
				for x := 0; x < pixSize; x++ {
					nxtX := (width - pixSize) - (pix.X * pixSize) + x
					nxtY := (pix.Y * pixSize) + y + pixel.Height
					img.Set(nxtX, nxtY, color)
				}
			}
		}

	}

	var shaders []PixelShaderLayerDB
	tablename = GetTableName("pixel_shader")
	db.Table(tablename).Where("pixel_image_id=?", pixel.ID).Find(&shaders)

	for _, shader := range shaders {
		if shader.Frame != frame {
			continue
		}

		if modifier == "forward" || modifier == "both" {
			for y := 0; y < pixSize; y++ {
				for x := 0; x < pixSize; x++ {
					rl, gl, bl, al := img.At((shader.X*pixSize)+x, (shader.Y*pixSize)+y).RGBA()
					r := uint8(rl >> 8)
					g := uint8(gl >> 8)
					b := uint8(bl >> 8)
					a := uint8(al >> 8)

					alph := uint8(shader.Alpha * 255)
					diff_a := float64(a-alph) / 255.0

					rNew := (float64(r) * diff_a) + (float64(shader.R) * (255 - diff_a))
					gNew := (float64(g) * diff_a) + (float64(shader.G) * (255 - diff_a))
					bNew := (float64(b) * diff_a) + (float64(shader.B) * (255 - diff_a))

					newColor := color.NRGBA{uint8(rNew), uint8(gNew), uint8(bNew), alph}
					if al > uint32(newColor.A) {
						newColor.A = uint8(al)
					}
					img.Set((shader.X*pixSize)+x, (shader.Y*pixSize)+y, newColor)
				}
			}
		} else if modifier == "reverse" {
			for y := 0; y < pixSize; y++ {
				for x := 0; x < pixSize; x++ {
					rl, gl, bl, al := img.At((shader.X*pixSize)+x, (shader.Y*pixSize)+y).RGBA()
					r := uint8(rl >> 8)
					g := uint8(gl >> 8)
					b := uint8(bl >> 8)
					a := uint8(al >> 8)

					alph := uint8(shader.Alpha * 255)
					diff_a := float64(a-alph) / 255.0

					rNew := (float64(r) * diff_a) + (float64(shader.R) * (255 - diff_a))
					gNew := (float64(g) * diff_a) + (float64(shader.G) * (255 - diff_a))
					bNew := (float64(b) * diff_a) + (float64(shader.B) * (255 - diff_a))

					newColor := color.NRGBA{uint8(rNew), uint8(gNew), uint8(bNew), alph}
					if al > uint32(newColor.A) {
						newColor.A = uint8(al)
					}
					
					nxtX := (width - pixSize) - (shader.X*pixSize)+x
					nxtY := (shader.Y*pixSize)+y
					
					img.Set(nxtX, nxtY, newColor)
				}
			}
		}

		if modifier == "both" {
			for y := 0; y < pixSize; y++ {
				for x := 0; x < pixSize; x++ {
					rl, gl, bl, al := img.At((shader.X*pixSize)+x, (shader.Y*pixSize)+y).RGBA()
					r := uint8(rl >> 8)
					g := uint8(gl >> 8)
					b := uint8(bl >> 8)
					a := uint8(al >> 8)

					alph := uint8(shader.Alpha * 255)
					diff_a := float64(a-alph) / 255.0

					rNew := (float64(r) * diff_a) + (float64(shader.R) * (255 - diff_a))
					gNew := (float64(g) * diff_a) + (float64(shader.G) * (255 - diff_a))
					bNew := (float64(b) * diff_a) + (float64(shader.B) * (255 - diff_a))

					newColor := color.NRGBA{uint8(rNew), uint8(gNew), uint8(bNew), alph}
					if al > uint32(newColor.A) {
						newColor.A = uint8(al)
					}

					nxtX := (width - pixSize) - (shader.X * pixSize) + x
					nxtY := (shader.Y * pixSize) + y + pixel.Height
					img.Set(nxtX, nxtY, newColor)
				}
			}
		}	
	}

	// Encode as PNG.
	fullPath := fmt.Sprintf("user/png/%s_%d.png", pixel.GUID, frame)
	// fmt.Println(fullPath)
	f, err := os.Create(fullPath)
	if err != nil {
		fmt.Println(err)
		return err

	}

	// Do the rotation
	var dstImage *image.NRGBA
	transparent := color.NRGBA{0, 0, 0, 0}
	dstImage = imaging.Rotate(img, float64(angle), transparent)
	if bFlip {
		dstImage = imaging.FlipH(dstImage)
	}

	png.Encode(f, dstImage)
	return nil
}

// RenderSpriteSheetImage - draw all available frames for image into one long sprite sheet
func RenderSpriteSheetImage(db *gorm.DB, pixel PixelAppImageDB, colMap map[string]string) error {

	var frameCount int
	tablename := GetTableName("pixel_item")
	row := db.Table(tablename).Where("pixel_image_id=?", pixel.ID).Select("max(frame)").Row()
	row.Scan(&frameCount)

	// fmt.Printf("%d\n", frameCount)
	width := pixel.Width
	height := pixel.Height * 2

	upLeft := image.Point{0, 0}
	img_width := width * (frameCount + 1)
	lowRight := image.Point{img_width, height}

	img := image.NewRGBA(image.Rectangle{upLeft, lowRight})
	var pixels []PixelItemDB

	// tablename := GetTableName("pixel_item")
	db.Table(tablename).Where("pixel_image_id=?", pixel.ID).Find(&pixels)
	pixSize := pixel.PixelWidth

	for _, pix := range pixels {

		alpha := uint8(pix.Alpha * 255)
		colString := GetHexColorFromRGB(pix.R, pix.G, pix.B)
		var color color.NRGBA
		color.A = alpha
		if val, ok := colMap[colString]; ok {
			//do something here
			red, green, blue := GetColorFromHex(val)
			// color = color.NRGBA{uint8(red), uint8(green), uint8(blue), alpha}
			color.R = uint8(red)
			color.G = uint8(green)
			color.B = uint8(blue)
		} else {
			color.R = uint8(pix.R)
			color.G = uint8(pix.G)
			color.B = uint8(pix.B)
			//  = color.NRGBA{uint8(pix.R), uint8(pix.G), uint8(pix.B), alpha}
		}

		offsetX := pix.Frame * pixel.Width

		for y := 0; y < pixSize; y++ {
			for x := 0; x < pixSize; x++ {
				nxtX := (pix.X * pixSize) + x
				nxtY := (pix.Y * pixSize) + y

				img.Set(nxtX+offsetX, nxtY, color)
			}
		}

		// Reverse image on second row
		for y := 0; y < pixSize; y++ {
			for x := 0; x < pixSize; x++ {
				nxtX := (width - pixSize) - (pix.X * pixSize) + x
				nxtY := (pix.Y * pixSize) + y + pixel.Height
				// fmt.Printf("%d, %d, %d \n", nxtX, nxtY, offsetX)
				img.Set(nxtX+offsetX, nxtY, color)
			}
		}
	}

	var shaders []PixelShaderLayerDB
	tablename = GetTableName("pixel_shader")
	db.Table(tablename).Where("pixel_image_id=?", pixel.ID).Find(&shaders)

	for _, shader := range shaders {
		offsetX := shader.Frame * pixel.Width
		for y := 0; y < pixSize; y++ {
			for x := 0; x < pixSize; x++ {
				rl, gl, bl, al := img.At((shader.X*pixSize)+x, (shader.Y*pixSize)+y).RGBA()
				r := uint8(rl >> 8)
				g := uint8(gl >> 8)
				b := uint8(bl >> 8)
				a := uint8(al >> 8)

				alph := uint8(shader.Alpha * 255)
				diff_a := float64(a-alph) / 255.0

				rNew := (float64(r) * diff_a) + (float64(shader.R) * (255 - diff_a))
				gNew := (float64(g) * diff_a) + (float64(shader.G) * (255 - diff_a))
				bNew := (float64(b) * diff_a) + (float64(shader.B) * (255 - diff_a))

				newColor := color.NRGBA{uint8(rNew), uint8(gNew), uint8(bNew), alph}
				if al > uint32(newColor.A) {
					newColor.A = uint8(al)
				}
				img.Set((shader.X*pixSize)+x+offsetX, (shader.Y*pixSize)+y, newColor)

				// Do the reverse frame beneath
				offsetX := shader.Frame * pixel.Width
				nxtX := (width - pixSize) - (shader.X * pixSize) + x
				nxtY := (shader.Y * pixSize) + y + pixel.Height
				img.Set(nxtX+offsetX, nxtY, newColor)
				
			}
		}
	}

	// Encode as PNG.
	fullPath := fmt.Sprintf("user/png/%s_spritesheet.png", pixel.GUID)
	f, err := os.Create(fullPath)
	if err != nil {
		fmt.Println(err)
		return err

	}
	png.Encode(f, img)
	return nil
}

// CreateBasicGif creates a GIF image with the given width and height.
// It uses white background and a black pixel in the middle of the image.
func CreateGif(db *gorm.DB, pixel PixelAppImageDB) error {
	width := pixel.Width
	height := pixel.Height

	pixSize := pixel.PixelWidth

	// palette := []color.Color{color.White, color.Black}
	rect := image.Rect(0, 0, width, height)
	// img := image.NewPaletted(rect, palette)

	// img.SetColorIndex(width/2, height/2, 1)
	var imgs []*image.Paletted
	var frames []int
	tablename := GetTableName("pixel_item")
	db.Distinct("frame").Table(tablename).Where("pixel_image_id=?", pixel.ID).Find(&frames)
	var delays []int
	for _, frame := range frames {
		var pixels []PixelItemDB
		tablename := GetTableName("pixel_item")
		db.Table(tablename).Where("pixel_image_id=? and frame=?", pixel.ID, frame).Find(&pixels)
		imgs = append(imgs, RenderFrame(rect, pixels, pixSize))
		// TODO: Look up delays
		delays = append(delays, 0)
	}

	anim := gif.GIF{Delay: delays, Image: imgs}
	fullPath := fmt.Sprintf("user/gif/%s.gif", pixel.GUID)
	outputFile, err := os.OpenFile(fullPath, os.O_WRONLY|os.O_CREATE, 0777)
	defer outputFile.Close()
	if err != nil {
		fmt.Println(err)
		// panic(err)
		return err
	}

	err = gif.EncodeAll(outputFile, &anim)
	if err != nil {

		fmt.Println(err)
		// panic(err)
		return err
	}

	return nil
}

func RenderFrame(rect image.Rectangle, pixels []PixelItemDB, pixSize int) *image.Paletted {
	// var col color.Color
	col := color.RGBA{0, 0, 0, 0}
	palette := []color.Color{col}
	frame := image.NewPaletted(rect, palette)

	for _, pix := range pixels {

		color := color.RGBA{uint8(pix.R), uint8(pix.G), uint8(pix.B), uint8(pix.Alpha)}
		for y := 0; y < pixSize; y++ {
			for x := 0; x < pixSize; x++ {
				frame.Set((pix.X*pixSize)+x, (pix.Y*pixSize)+y, color)
			}
		}
	}
	return frame
}
