_tspin() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="tspin"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        tspin)
            opts="-f -p -c -h -V --follow --print --config-path --listen-command --words-red --words-green --words-yellow --words-blue --words-magenta --words-cyan --enable --disable --no-builtin-keywords --generate-bash-completions --generate-fish-completions --generate-zsh-completions --help --version [FILE]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --config-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --listen-command)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --words-red)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --words-green)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --words-yellow)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --words-blue)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --words-magenta)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --words-cyan)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --enable)
                    COMPREPLY=($(compgen -W "numbers urls pointers dates paths quotes key-value-pairs uuids ip-addresses processes json" -- "${cur}"))
                    return 0
                    ;;
                --disable)
                    COMPREPLY=($(compgen -W "numbers urls pointers dates paths quotes key-value-pairs uuids ip-addresses processes json" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _tspin -o nosort -o bashdefault -o default tspin
else
    complete -F _tspin -o bashdefault -o default tspin
fi
