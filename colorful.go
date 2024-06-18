package colorful

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"math/rand"
	"os"
)

// "image/color"

func generateImage() image.Image {
    width := 2048
    height := 2048
    rect := image.Rectangle{image.Point{0, 0}, image.Point{width, height}}
    img := image.NewRGBA(rect)

    for x := 0; x < width; x++ {
        for y := 0; y < height; y++ {
            col := color.RGBA{uint8(rand.Intn(0xFF)), uint8(rand.Intn(0xFF)), uint8(rand.Intn(0xFF)), 0xFF}
            img.Set(x, y, col)
        }
    }

    return img
}



func main() {
    fmt.Println("Hello, World!")

    img := generateImage()
    f, _ := os.Create("image.png")
    png.Encode(f, img)
}
