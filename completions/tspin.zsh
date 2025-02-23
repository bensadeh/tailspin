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
'--config-path=[Provide a custom path to a configuration file]:CONFIG_PATH:_default' \
'(-f --follow)-c+[Capture the output (stdout) of a command and view it in \`less\`]:LISTEN_COMMAND:_default' \
'(-f --follow)--listen-command=[Capture the output (stdout) of a command and view it in \`less\`]:LISTEN_COMMAND:_default' \
'*--words-red=[Highlight the provided words in red]:WORDS_RED:_default' \
'*--words-green=[Highlight the provided words in green]:WORDS_GREEN:_default' \
'*--words-yellow=[Highlight the provided words in yellow]:WORDS_YELLOW:_default' \
'*--words-blue=[Highlight the provided words in blue]:WORDS_BLUE:_default' \
'*--words-magenta=[Highlight the provided words in magenta]:WORDS_MAGENTA:_default' \
'*--words-cyan=[Highlight the provided words in cyan]:WORDS_CYAN:_default' \
'*--enable=[Enable specific highlighters]:ENABLE:(numbers urls pointers dates paths quotes key-value-pairs uuids ip-addresses processes json)' \
'*--disable=[Disable specific highlighters]:DISABLE:(numbers urls pointers dates paths quotes key-value-pairs uuids ip-addresses processes json)' \
'-f[Follow the contents of a file]' \
'--follow[Follow the contents of a file]' \
'-e[Start at the end of the file]' \
'--start-at-end[Start at the end of the file]' \
'-p[Print the output to stdout]' \
'--print[Print the output to stdout]' \
'--no-builtin-keywords[Disable the highlighting of all builtin keyword groups (booleans, nulls, log severities and common REST verbs)]' \
'--suppress-output[Suppress all output (for debugging and benchmarking)]' \
'--generate-bash-completions[Print bash completions to stdout]' \
'--generate-fish-completions[Print fish completions to stdout]' \
'--generate-zsh-completions[Print zsh completions to stdout]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
'::FILE -- Path to file or folder:_files' \
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
