VER := $(shell git describe --tags --abbrev=0)

all:
	equinox release --config ../equinox/doctor.yaml --channel stable --version $(VER) github.com/kmaasrud/doctor
