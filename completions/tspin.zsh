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
'--config-path=[Provide a custom path to a configuration file]:CONFIG_PATH: ' \
'(-f --follow)-c+[Capture the output (stdout) of a command and view it in \`less\`]:LISTEN_COMMAND: ' \
'(-f --follow)--listen-command=[Capture the output (stdout) of a command and view it in \`less\`]:LISTEN_COMMAND: ' \
'*--words-red=[Highlight the provided words in red]:WORDS_RED: ' \
'*--words-green=[Highlight the provided words in green]:WORDS_GREEN: ' \
'*--words-yellow=[Highlight the provided words in yellow]:WORDS_YELLOW: ' \
'*--words-blue=[Highlight the provided words in blue]:WORDS_BLUE: ' \
'*--words-magenta=[Highlight the provided words in magenta]:WORDS_MAGENTA: ' \
'*--words-cyan=[Highlight the provided words in cyan]:WORDS_CYAN: ' \
'--hidden-generate-shell-completions=[Print completions to stdout]:GENERATE_SHELL_COMPLETIONS: ' \
'-f[Follow the contents of a file]' \
'--follow[Follow the contents of a file]' \
'-e[Start at the end of the file]' \
'--start-at-end[Start at the end of the file]' \
'-p[Print the output to stdout]' \
'--print[Print the output to stdout]' \
'--disable-builtin-keywords[Disable the highlighting of all builtin keyword groups (booleans, severity and REST)]' \
'--disable-booleans[Disable the highlighting of booleans and nulls]' \
'--disable-severity[Disable the highlighting of severity levels]' \
'--disable-rest[Disable the highlighting of REST verbs]' \
'--enable-numbers[Enable the highlighting of numbers]' \
'--disable-numbers[Disable the highlighting of numbers]' \
'--enable-dates[Enable the highlighting of dates]' \
'--disable-dates[Disable the highlighting of dates]' \
'--enable-urls[Enable the highlighting of URLs]' \
'--disable-urls[Disable the highlighting of URLs]' \
'--enable-paths[Enable the highlighting of paths]' \
'--disable-paths[Disable the highlighting of paths]' \
'--enable-quotes[Enable the highlighting of quotes]' \
'--disable-quotes[Disable the highlighting of quotes]' \
'--enable-key-value-pairs[Enable the highlighting of key value pairs]' \
'--disable-key-value-pairs[Disable the highlighting of key value pairs]' \
'--enable-uuids[Enable the highlighting of UUIDs]' \
'--disable-uuids[Disable the highlighting of UUIDs]' \
'--enable-ip-addresses[Enable the highlighting of IP addresses]' \
'--disable-ip-addresses[Disable the highlighting of IP addresses]' \
'--enable-pointers[Enable the highlighting of pointers]' \
'--disable-pointers[Disable the highlighting of pointers]' \
'--enable-processes[Enable the highlighting of unix processes]' \
'--disable-processes[Disable the highlighting of unix processes]' \
'--hidden-suppress-output[Suppress all output (for debugging and benchmarking)]' \
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
