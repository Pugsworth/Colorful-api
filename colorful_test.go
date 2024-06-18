package colorful

import "testing"

func BenchGenerateImage(b *testing.B) {
    for i := 0; i < b.N; i++ {
        generateImage()
    }
}

