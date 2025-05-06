mkbuilddir:
	mkdir -p build

build-go-static: mkbuilddir
	cd golang && CGO_ENABLED=1 GOOS=darwin GOARCH=arm64 go build -buildmode=c-archive -o ../build/libdbmaters.a -ldflags="-s -w" main.go

build: build-go-static
	cargo build --release

publish: build
	git tag -a v0.1.0 -m "Initial release"
	git push origin v0.1.0

clean:
	rm -rf build
