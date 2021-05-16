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
		"armv8")
			ARCH=arm64
			;;
		esac
		PLATFORM="linux_$ARCH"
        FILENAME="doctor_linux_$ARCH.tar.gz"
	;;
	"Darwin")
		PLATFORM="darwin_amd64"
        FILENAME="doctor_darwin_$ARCH.zip"
	;;
esac

if [ -z "$PLATFORM" ]; then
	echo "Your operating system is not supported by the script."
	exit 1
fi

# Find shell profile and make sure it exists
echo "Finding shell..."
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

# Ensure local PATH addition
echo "Making sure ~/.local/bin is in your PATH..."
case ":$PATH:" in
*:$HOME/.local/bin:*) ;;
*)
	if [ "$shell" == "fish" ]; then
		{
			echo '# Added by Doctor'
			echo 'set PATH $HOME/.local/bin $PATH'
		} >> "$shell_profile"
	else
		{
			echo '# Added by Doctor'
			echo 'export PATH=$HOME/.local/bin:$PATH'
		} >> "$shell_profile"
	fi
	;;
esac

# Ensuring directory exists
mkdir -p "$HOME/.local/bin"

# Make temporary dir
tmp=$(mktemp -d)

# Install Doctor
echo "Installing latest version of Doctor..."
latest=$(curl -sL https://api.github.com/repos/kmaasrud/doctor/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
curl "https://github.com/kmaasrud/doctor/releases/download/v0.2.3/$FILENAME" --output "$tmp/$FILENAME"
# Equinox: curl "https://bin.equinox.io/c/fHpZLhLmi7c/doctor-stable-$PLATFORM.tgz" --output "$tmp/doctor.tgz"

# Extract into tmp dir
echo "Extracting archive..."
case $OS in
    "Linux")
        tar xvf "$tmp/$FILENAME" -C "$HOME/.local/bin"
    ;;
    "Darwin")
        unzip "$tmp/$FILENAME" -d "$HOME/.local/bin"
    ;;
esac

if [ -e "$HOME/.local/bin/doctor" ]
then
	echo -e "\nDoctor was installed successfully!\n"
	echo -e "\nMake sure to relogin into your shell or run:"
	echo -e "\n\tsource $shell_profile\n\nto update your environment variables.\n"
else
	echo -e "\nThe automatic install did not work, install Doctor manually.\n"
fi

# Remove temporary directory
rm -rf $tmp
