package main

import "C"
import (
	"net/url"

	"github.com/amacneil/dbmate/v2/pkg/dbmate"
	_ "github.com/amacneil/dbmate/v2/pkg/driver/postgres"
)

//export CreateAndMigrate
func CreateAndMigrate(path string) *C.char {
	u, _ := url.Parse(path)
	db := dbmate.New(u)
	err := db.CreateAndMigrate()
	if err != nil {
		return C.CString(err.Error())
	}
	return C.CString("success")
}

func main() {}
