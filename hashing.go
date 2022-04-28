package main

import (
	"crypto/md5"
	"fmt"
	"io"
	"log"
	"os"
)

type Hash struct {
	value string
}

func (h *Hash) Value() string {
	return h.value
}

func (h *Hash) Equals(o *Hash) bool {
	return true
}

func Md5(file *os.File) string {
	hash := md5.New()
	_, err := io.Copy(hash, file)

	if err != nil {
		panic(err)
	}

	return fmt.Sprintf("%x", hash.Sum(nil))
}

func Test() {

	file, err := os.Open("data/mascot.jpg")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	hash := Md5(file)
	fmt.Println(hash)

	fileCopy, err := os.Create("data/mascot_2.jpg")
	if err != nil {
		log.Fatal(err)
	}
	defer fileCopy.Close()

	b, err := io.Copy(fileCopy, file)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%d bytes written.\n", b)

	fmt.Println(Md5(fileCopy))
}
