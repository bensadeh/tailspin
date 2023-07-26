#compdef spin

autoload -U is-at-least

_spin() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-c+[Path to a custom configuration file]:CONFIG_PATH: ' \
'--config-path=[Path to a custom configuration file]:CONFIG_PATH: ' \
'-t+[Tails the output of the provided command]:TAIL_COMMAND: ' \
'--tail-command=[Tails the output of the provided command]:TAIL_COMMAND: ' \
'--z-generate-shell-completions=[Print completions to stdout]:GENERATE_SHELL_COMPLETIONS: ' \
'-f[Follow (tail) the contents of the file]' \
'--follow[Follow (tail) the contents of the file]' \
'(-f --follow)-p[Print the output to stdout]' \
'(-f --follow)--print[Print the output to stdout]' \
'--create-default-config[Generate a new configuration file]' \
'(--create-default-config)--show-default-config[Print the default configuration]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
'::FILE -- Filepath:' \
&& ret=0
}

(( $+functions[_spin_commands] )) ||
_spin_commands() {
    local commands; commands=()
    _describe -t commands 'spin commands' commands "$@"
}

if [ "$funcstack[1]" = "_spin" ]; then
    _spin "$@"
else
    compdef _spin spin
fi
