package main

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"log"
	"math"
	"net/http"
	"strings"

	"golang.org/x/image/font"
	"golang.org/x/image/font/basicfont"
	"golang.org/x/image/math/fixed"

	"encoding/hex"
)

func imageFromColors(colors []color.RGBA) image.Image {
    blockSize := 64

    wBlocks := len(colors)
    hBlocks := 1

    // TODO: Figure out best image resolution from number of colors (> 2:1 aspect wraps??)
    if wBlocks > 6 {
        wBlocks = 6
        hBlocks = int(math.Ceil(float64(len(colors)) / 6.0))
    }

    width := wBlocks * blockSize
    height := hBlocks * blockSize

    rect := image.Rectangle{image.Point{0, 0}, image.Point{width, height}}
    img := image.NewRGBA(rect)

    // Fill the blocks with the given colors
    i := 0

    for y := 0; y < height; y += blockSize {
        for x := 0; x < width; x += blockSize {
            col := color.RGBA{240, 240, 240, 0xFF}

            if (i < len(colors)) {
                col = colors[i]
            }

            for hb := 0; hb < blockSize; hb++ {
                for wb := 0; wb < blockSize; wb++ {
                    px := x + wb
                    py := y + hb

                    if px < width && py < height {
                        img.Set(px, py, col)
                    }
                }
            }

            i += 1
        }
    }


    return img
}


func generateImage(colors []color.RGBA) image.Image {
    width := 128 // Total image width
    height := 128 // Total image height

    blockSize := 16 // length of side for each block
    // wBlocks := width/blockSize // Number of blocks along width
    // hBlocks := height/blockSize // Number of blocks along height
    wBlocks := len(colors)
    hBlocks := 1

    rect := image.Rectangle{image.Point{0, 0}, image.Point{width, height}}
    img := image.NewRGBA(rect)

    i := 0

    for wb := 0; wb < wBlocks; wb++ {
        for hb := 0; hb < hBlocks; hb++ {
            blockRect := image.Rectangle{image.Point{wb*blockSize, hb*blockSize}, image.Point{(wb+1)*blockSize, (hb+1)*blockSize}}
            // col := color.RGBA{uint8(rand.Intn(0xFF)), uint8(rand.Intn(0xFF)), uint8(rand.Intn(0xFF)), 0xFF}
            col := colors[i]

            // Stop on the last color for now...
            if i < len(colors)-1 {
                i += 1
            }

            for x := blockRect.Min.X; x < blockRect.Max.X; x++ {
                for y := blockRect.Min.Y; y < blockRect.Max.Y; y++ {
                    img.Set(x, y, col)
                }
            }
        }
    }

    /*
    for x := 0; x < width; x++ {
        for y := 0; y < height; y++ {
            col := color.RGBA{uint8(rand.Intn(0xFF)), uint8(rand.Intn(0xFF)), uint8(rand.Intn(0xFF)), 0xFF}
            img.Set(x, y, col)
        }
    }
    */

    return img
}


func addLabel(img *image.RGBA, x, y int, label string) {
    col := color.RGBA{20, 30, 20, 255}

    face := basicfont.Face7x13

    textWidth := font.MeasureString(face, label).Ceil()
    textHeight := face.Metrics().Ascent.Ceil() + face.Metrics().Descent.Ceil()

    point := fixed.Point26_6{fixed.I(x - textWidth / 2), fixed.I(y - textHeight / 2)}

    d := &font.Drawer{
        Dst: img,
        Src: image.NewUniform(col),
        Face: basicfont.Face7x13,
        Dot: point,
    }

    d.DrawString(label)
}


func generateErrorImage(err string) image.Image {
    width := 200
    height := 120

    rect := image.Rectangle{image.Point{0, 0}, image.Point{width, height}}
    img := image.NewRGBA(rect)

    x := width/2
    y := height/2

    addLabel(img, x, y, err)

    return img
}


func errorHandler(w http.ResponseWriter, r *http.Request, code int) {
    img := generateErrorImage(fmt.Sprint(code))

    w.Header().Add("Content-Type", "image/png")
    err := png.Encode(w, img)
    if (err != nil) {
        fmt.Fprintf(w, "Encountered an error while encoding PNG; '%s'", err)
    }
}


func handler(w http.ResponseWriter, r *http.Request) {
    if (r.URL.Path != "/") {
        errorHandler(w, r, http.StatusNotFound)
        return
    }

    fmt.Fprintf(w, "Hi there, I love %s!\n", r.URL.Path[1:])
    fmt.Fprintf(w, "Query: '%s'", r.URL.Query().Encode())
}


func getColors(r *http.Request) []color.RGBA {
    var colors = make([]color.RGBA, 0)

    var qColors = r.URL.Query().Get("colors")
    qColors = strings.ReplaceAll(qColors, "#", "")

    for _, colStr := range strings.Split(qColors, ",") {
        parts, err := hex.DecodeString(colStr)
        if err != nil {
            log.Fatal(err)
        }

        col := color.RGBA{parts[0], parts[1], parts[2], 0xFF}

        colors = append(colors, col)
    }

    return colors
}


func genPaletteHandler(w http.ResponseWriter, r *http.Request) {
    w.Header().Add("Content-Type", "image/png")

    var colors = getColors(r)
    fmt.Printf("There are %i colors\n", len(colors))
    for _, col := range colors {
        cc := make([]byte, 3)
        cc[0] = col.R
        cc[1] = col.G
        cc[2] = col.B
        fmt.Printf("  %s\n", hex.EncodeToString(cc))
    }

    // img := generateImage(colors)
    img := imageFromColors(colors)
    err := png.Encode(w, img)
    if err != nil {
        fmt.Fprintf(w, "Encountered an error while encoding PNG; '%s'", err)
    }
}


func main() {
    fmt.Println("Hello, World!")

    // img := generateImage()
    // f, _ := os.Create("image.png")
    // png.Encode(f, img)

    http.HandleFunc("/", handler)
    http.HandleFunc("/palette", genPaletteHandler)
    log.Fatal(http.ListenAndServe("localhost:8080", nil))
}
