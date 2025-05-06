//go:build linux || darwin
// +build linux darwin

package main

/*
#include <stdlib.h>
#include <string.h>
*/
import "C"
import (
	"net/url"
	"unsafe"

	"github.com/amacneil/dbmate/v2/pkg/dbmate"
	_ "github.com/lib/pq"
)

//export CreateAndMigrate
func CreateAndMigrate(path unsafe.Pointer) unsafe.Pointer {
	goPath := C.GoString((*C.char)(path))
	u, err := url.Parse(goPath)
	if err != nil {
		return unsafe.Pointer(C.CString(err.Error()))
	}
	db := dbmate.New(u)
	db.AutoDumpSchema = false
	err = db.Migrate()
	if err != nil {
		return unsafe.Pointer(C.CString(err.Error()))
	}
	return unsafe.Pointer(C.CString("success"))
}

func main() {}
