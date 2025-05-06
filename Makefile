mkbuilddir:
	mkdir -p build

build-go-static: mkbuilddir
	cd golang && go build -buildmode=c-archive -o ../build/libdbmaters.a main.go

build: build-go-static
	cargo build --release

clean:
	rm -rf build
