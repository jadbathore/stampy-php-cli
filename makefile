TARGET_ARM64=aarch64-unknown-linux-gnu
ARCHI_ARM64=linux/arm64
TARGET_AMD64=x86_64-unknown-linux-gnu
ARCHI_AMD64=linux/amd64
TARGET_WIN_X86_64=x86_64-pc-windows-gnu
TARGET_APPLE_DARWIN=x86_64-apple-darwin

self:
# will build for(personally for target arm64 / aarch64)
	cargo build --release 

x86_64-apple:
	cargo build --release --target=$(TARGET_APPLE_DARWIN) 


arm64:
	mkdir -p target/release/$(TARGET_ARM64) && \
	DOCKER_BUILDKIT=0 docker buildx build \
	-f docker-compile/dockerfile.linux \
	--platform=$(ARCHI_ARM64) . \
	-t stampy/php-cli-lib:v1 && \
	docker create --name stampy-temp stampy/php-cli-lib:v1 && \
	docker cp stampy-temp:/app/target/release/libstampy_php_cli.so ./target/release/$(TARGET_ARM64)/libstampy_php_cli.so && \
	docker rm stampy-temp && \
	docker rmi -f stampy/php-cli-lib:v1

amd64:
	mkdir -p target/release/$(TARGET_AMD64) && \
	DOCKER_BUILDKIT=0 docker buildx build \
	-f docker-compile/dockerfile.linux \
	--platform=$(ARCHI_AMD64) . \
	-t stampy/php-cli-lib:v1 && \
	docker create --name stampy-temp stampy/php-cli-lib:v1 && \
	docker cp stampy-temp:/app/target/release/libstampy_php_cli.so ./target/release/$(TARGET_AMD64)/libstampy_php_cli.so && \
	docker rm stampy-temp && \
	docker rmi -f stampy/php-cli-lib:v1
linux: arm64 amd64

x84_64-win:
	mkdir -p target/release/$(TARGET_WIN_X86_64) && \
	DOCKER_BUILDKIT=0 docker buildx build --progress=plain \
	-f docker-compile/dockerfile.window \
	--build-arg TARGET=$(TARGET_WIN_X86_64)  \
	--platform=$(ARCHI_AMD64) . \
	-t stampy/php-cli-lib:v1 && \
	docker create --name stampy-temp stampy/php-cli-lib:v1 && \
	docker cp stampy-temp:/php-cli/target/$(TARGET_WIN_X86_64)/release/ ./target/release/$(TARGET_WIN_X86_64)/ && \
	docker rm stampy-temp && \
	docker rmi -f stampy/php-cli-lib:v1

clean:


# Cible par défaut
all: self linux x84_64-win