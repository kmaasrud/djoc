VER := $(shell git describe --tags --abbrev=0)

all:
	equinox release --config ../equinox/doctor.yaml --channel stable --version $(VER) github.com/kmaasrud/doctor
	# TODO: Investigate how to include build flags, like this
	# equinox release --config ../equinox/doctor.yaml --channel stable --version $(VER) github.com/kmaasrud/doctor -ldflags="-s -w -X 'main.VERSION=$(VER)'"

build:
	for arch in "amd64" "arm64"; do \
		GOOS=linux GOARCH=$$arch go build -ldflags="-s -w -X 'main.VERSION=$(VER)'" -o bin/bins/doctor-$(VER)-linux-$$arch ; \
		cp bin/bins/*linux-$$arch* bin/doctor ; \
		tar -czvf bin/doctor-$(VER)-linux-$$arch.tar.gz bin/doctor ; \
		rm bin/doctor ; \
	done
	for arch in "amd64" "arm64"; do \
		GOOS=darwin GOARCH=$$arch go build -ldflags="-s -w -X 'main.VERSION=$(VER)'" -o bin/bins/doctor-$(VER)-macOS-$$arch ; \
		cp bin/bins/*macOS-$$arch* bin/doctor ; \
		zip bin/doctor-$(VER)-macOS-$$arch.zip bin/doctor ; \
		rm bin/doctor ; \
	done ; \
	for arch in "amd64" "arm"; do \
		GOOS=windows GOARCH=$$arch go build -ldflags="-s -w -X 'main.VERSION=$(VER)'" -o bin/bins/doctor-$(VER)-windows-$$arch.exe ; \
		cp bin/bins/*windows-$$arch*.exe bin/doctor.exe ; \
		zip bin/doctor-$(VER)-windows-$$arch.zip bin/doctor.exe ; \
		rm bin/doctor.exe ; \
	done ; \
	mv bin/*windows-amd64.zip bin/doctor-$(VER)-windows-x86.zip
	mv bin/bins/*windows-amd64.exe bin/bins/doctor-$(VER)-windows-x86.exe

.PHONY: clean

clean:
	rm -rf bin
