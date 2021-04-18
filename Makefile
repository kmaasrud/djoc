VER := $(shell git describe --tags --abbrev=0)

all:
	equinox release --config ../equinox/doctor.yaml --channel stable --version $(VER) github.com/kmaasrud/doctor
	# TODO: Investigate how to include build flags, like this
	# equinox release --config ../equinox/doctor.yaml --channel stable --version $(VER) github.com/kmaasrud/doctor -ldflags="-s -w -X 'main.VERSION=$(VER)'"
