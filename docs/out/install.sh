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

# Ensure local PATH addition
echo "Making sure ~/.local/bin is in your PATH..."
case ":$PATH:" in
*:$HOME/.local/bin:*) ;;
*)
	if [ "$shell" == "fish" ]; then
		{
			echo '# GoLang'
			echo 'set PATH $HOME/.local/bin $PATH'
		} >> "$shell_profile"
	else
		{
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
echo "Installing latest stable version of Doctor..."
curl "https://bin.equinox.io/c/fHpZLhLmi7c/doctor-stable-$PLATFORM.tgz" --output "$tmp/doctor.tgz"
tar xvf doctor.tgz -C "$HOME/.local/bin"

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
