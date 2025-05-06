BASE_GO_FLAGS := "-trimpath -ldflags=-buildmode=c-archive -ldflags=-s -ldflags=-w"
VERSION := `grep '^version = ' Cargo.toml | cut -d'"' -f2`

default:
  @just --list

# Build the native library
build-native-library:
  @echo "Building release executables (incl. cross compilation) ..."
  mkdir -p build
  cd golang && CGO_ENABLED=1 go build -buildmode=c-archive -o ../build/libdbmaters.a -ldflags="-s -w" main.go
  mv build/libdbmaters.h build/libdbmaters.h

# Build the Rust library  
build-rust:
  @echo "Building Rust library..."
  cargo build --release

# Build and release the multi-arch library
release: clean build-native-library build-rust
  @echo "Building and releasing multi-arch library..."
  # Ensure build directory exists and has the right files
  mkdir -p build
  cargo release

# Clean up build artifacts
clean:
  cargo clean
  rm -rf build
