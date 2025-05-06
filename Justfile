BASE_GO_FLAGS := "-trimpath -ldflags=-buildmode=c-archive -ldflags=-s -ldflags=-w"

default:
  @just --list

# Build the native library
build-native-library:
  @echo "Building release executables (incl. cross compilation) ..."
  mkdir -p build
  cd golang && CGO_ENABLED=1 go build -buildmode=c-archive -o ../build/libdbmaters.a -ldflags="-s -w" main.go

# Build the Rust library  
build-rust:
  @echo "Building Rust library..."
  cargo build --release

# Build and release the multi-arch library
release: clean build-native-library build-rust
  @echo "Building and releasing multi-arch library..."
  cargo release

# Clean up build artifacts
clean:
  cargo clean
  rm -rf build
