#compdef tspin

autoload -U is-at-least

_tspin() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'--config-path=[Provide a custom path to a configuration file]:CONFIG_PATH:_files' \
'-e+[Run command and view the output in a pager]:EXEC:_default' \
'--exec=[Run command and view the output in a pager]:EXEC:_default' \
'*--highlight=[Highlights in the form color\:word1,word2]:COLOR_WORD:_default' \
'*--enable=[Enable specific highlighters]:ENABLED_HIGHLIGHTERS:(numbers urls pointers dates paths quotes key-value-pairs uuids ip-addresses processes json)' \
'*--disable=[Disable specific highlighters]:DISABLED_HIGHLIGHTERS:(numbers urls pointers dates paths quotes key-value-pairs uuids ip-addresses processes json)' \
'--pager=[Override the default pager command used by tspin. (e.g. \`--pager="ov -f \[FILE\]"\`)]:PAGER:_default' \
'-f[Follow the contents of a file]' \
'--follow[Follow the contents of a file]' \
'-p[Print the output to stdout]' \
'--print[Print the output to stdout]' \
'--disable-builtin-keywords[Disable the highlighting of all builtin keyword groups (booleans, nulls, log severities and common REST verbs)]' \
'--generate-bash-completions[Print bash completions to stdout]' \
'--generate-fish-completions[Print fish completions to stdout]' \
'--generate-zsh-completions[Print zsh completions to stdout]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
'-V[Print version]' \
'--version[Print version]' \
'::FILE -- Filepath:_files' \
&& ret=0
}

(( $+functions[_tspin_commands] )) ||
_tspin_commands() {
    local commands; commands=()
    _describe -t commands 'tspin commands' commands "$@"
}

if [ "$funcstack[1]" = "_tspin" ]; then
    _tspin "$@"
else
    compdef _tspin tspin
fi
