mkbuilddir:
	mkdir -p build

build-go-static: mkbuilddir
	cd golang && CGO_ENABLED=1 go build -buildmode=c-archive -o ../build/libdbmaters.a -ldflags="-s -w" main.go

build: build-go-static
	@cargo build --release

clean:
	rm -rf build

release:
	@cargo release tag --execute
	@git cliff -o CHANGELOG.md
	@git commit -a -n -m "Update CHANGELOG.md" || true
	@git push origin main
	@cargo release push --execute

.PHONY: build clean release
