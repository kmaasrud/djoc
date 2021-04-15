GOVERSION="1.16"

OS="$(uname -s)"
ARCH="$(uname -m)"

case $OS in
	"Linux")
		case $ARCH in
		"x86_64")
			ARCH=amd64
			;;
		"aarch64")
			ARCH=arm64
			;;
		"armv6" | "armv7l")
			ARCH=armv6l
			;;
		"armv8")
			ARCH=arm64
			;;
		.*386.*)
			ARCH=386
			;;
		esac
		PLATFORM="linux-$ARCH"
	;;
	"Darwin")
		PLATFORM="darwin-amd64"
	;;
esac

if [ -z "$PLATFORM" ]; then
	echo "Your operating system is not supported by the script."
	exit 1
fi

# Find shell profile and make sure it exists
if [ -n "$($SHELL -c 'echo $ZSH_VERSION')" ]; then
    shell_profile="$HOME/.zshrc"
elif [ -n "$($SHELL -c 'echo $BASH_VERSION')" ]; then
    shell_profile="$HOME/.bashrc"
elif [ -n "$($SHELL -c 'echo $FISH_VERSION')" ]; then
    shell="fish"
    if [ -d "$XDG_CONFIG_HOME" ]; then
        shell_profile="$XDG_CONFIG_HOME/fish/config.fish"
    else
        shell_profile="$HOME/.config/fish/config.fish"
    fi
fi
touch "$shell_profile"

# Check if Go is available
if ! command -v go &> /dev/null
then
	# Fix variables
	[ -z "$GOROOT" ] && GOROOT="$HOME/.go"
	PACKAGE_NAME="go$GOVERSION.$PLATFORM.tar.gz"
	TEMP_DIRECTORY=$(mktemp -d)

	# Download Go
	echo "Downloading $PACKAGE_NAME..."
	if hash wget 2>/dev/null; then
		wget https://storage.googleapis.com/golang/$PACKAGE_NAME -O "$TEMP_DIRECTORY/go.tar.gz"
	else
		curl -o "$TEMP_DIRECTORY/go.tar.gz" https://storage.googleapis.com/golang/$PACKAGE_NAME
	fi
	if [ $? -ne 0 ]; then
		echo "Download failed! Exiting."
		exit 1
	fi

	# Extract file
	echo "Extracting File..."
	mkdir -p "$GOROOT"
	tar -C "$GOROOT" --strip-components=1 -xzf "$TEMP_DIRECTORY/go.tar.gz"

	# Make sure GOROOT is in PATH
	echo "Ensuring Go is in PATH..."
	case ":$PATH:" in
	*:$GOROOT/bin:*) echo "GOROOT is in your PATH already, moving on.";;
	*)
		if [ "$shell" == "fish" ]; then
			{
				echo '# GoLang'
				echo "set GOROOT '${GOROOT}'"
				echo 'set PATH $GOROOT/bin $PATH'
			} >> "$shell_profile"
		else
			{
				echo '# GoLang'
				echo "export GOROOT=${GOROOT}"
				echo 'export PATH=$GOROOT/bin:$PATH'
			} >> "$shell_profile"
		fi
		;;
	esac
fi

[ -z "$GOPATH" ] && GOPATH="$HOME/go"


# Make sure GOPATH is in PATH
echo "Making sure GOPATH is in your PATH..."
case ":$PATH:" in
*:$GOPATH/bin:*) echo "GOPATH is in your PATH already, moving on.";;
*)
	if [ "$shell" == "fish" ]; then
		{
			echo '# GoLang'
			echo "set GOPATH '$GOPATH'"
			echo 'set PATH $GOPATH/bin $PATH'
		} >> "$shell_profile"
	else
		{
			echo '# GoLang'
			echo "export GOPATH=$GOPATH"
			echo 'export PATH=$GOPATH/bin:$PATH'
		} >> "$shell_profile"
	fi
	;;
esac

# Ensuring directories exist
mkdir -p "${GOPATH}/"{src,pkg,bin}


# Install Doctor
echo "Installing Doctor with Go tool..."
source $shell_profile
go install github.com/kmaasrud/doctor@latest

if [ -e "$GOPATH/bin/doctor" ]
then
	echo -e "\nDoctor was installed successfully!\n"
	echo -e "\nMake sure to relogin into your shell or run:"
	echo -e "\n\tsource $shell_profile\n\nto update your environment variables.\n"
else
	echo -e "\nDoctor did not get installed.\n"
	echo "Check that you are running the latest version of Go and try again."
	echo "If that is not the issue, check alternative install methods on the Doctor installation page."
fi
