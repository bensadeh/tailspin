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
            opts="-f -t -p -c -l -h -V --follow --tail --print --config-path --follow-command --words-red --words-green --words-yellow --words-blue --words-magenta --words-cyan --disable-builtin-keywords --disable-booleans --disable-severity --disable-rest --z-generate-shell-completions --bucket-size --help --version [FILE]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --config-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --follow-command)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
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
                --z-generate-shell-completions)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --bucket-size)
                    COMPREPLY=($(compgen -f "${cur}"))
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

complete -F _tspin -o nosort -o bashdefault -o default tspin
