[ -z "$GOPATH" ] && GOPATH="$HOME/go"

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

echo "Configuring your shell..."
touch "$shell_profile"
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

mkdir -p "${GOPATH}/"{src,pkg,bin}

echo "Installing Doctor with Go tool..."
go install github.com/kmaasrud/doctor@latest

echo -e "\nMake sure to relogin into your shell or run:"
echo -e "\n\tsource $shell_profile\n\nto update your environment variables."
