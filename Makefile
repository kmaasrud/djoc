VER := $(shell git describe --tags --abbrev=0)

all:
	# Linux
	for arch in "amd64" "arm64"; do \
		CGO_ENABLED=0 GOOS=linux GOARCH=$$arch go build -ldflags="-s -w -X 'main.VERSION=$(VER)'" -o bin/bins/doctor_linux_$$arch ; \
		cp bin/bins/*linux_$$arch* bin/doctor ; \
		tar -czvf bin/doctor_linux_$$arch.tar.gz bin/doctor ; \
		rm bin/doctor ; \
	done
	# MacOS
	for arch in "amd64" "arm64"; do \
		CGO_ENABLED=0 GOOS=darwin GOARCH=$$arch go build -ldflags="-s -w -X 'main.VERSION=$(VER)'" -o bin/bins/doctor_darwin_$$arch ; \
		cp bin/bins/*darwin_$$arch* bin/doctor ; \
		zip bin/doctor_darwin_$$arch.zip bin/doctor ; \
		rm bin/doctor ; \
	done ; \
	# Windows
	for arch in "amd64" "arm"; do \
		CGO_ENABLED=0 GOOS=windows GOARCH=$$arch go build -ldflags="-s -w -X 'main.VERSION=$(VER)'" -o bin/bins/doctor_windows_$$arch.exe ; \
		cp bin/bins/*windows_$$arch*.exe bin/doctor.exe ; \
		zip bin/doctor_windows_$$arch.zip bin/doctor.exe ; \
		rm bin/doctor.exe ; \
	done ; \

equinox:
	equinox release --config ../equinox/doctor.yaml --channel stable --version $(VER) github.com/kmaasrud/doctor
	# TODO: Investigate how to include build flags, like this
	# equinox release --config ../equinox/doctor.yaml --channel stable --version $(VER) github.com/kmaasrud/doctor -ldflags="-s -w -X 'main.VERSION=$(VER)'"

.PHONY: clean fmt

clean:
	rm -rf bin

fmt:
	@go fmt
	@cd cmd && go fmt
	@cd lua && go fmt
	@cd core && go fmt
	@cd utils && go fmt
	@cd msg && go fmt
