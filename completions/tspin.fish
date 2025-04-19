complete -c tspin -l config-path -d 'Provide a custom path to a configuration file' -r -F
complete -c tspin -s e -l exec -d 'Run command and view the output in a pager' -r
complete -c tspin -l highlight -d 'Highlights in the form color:word1,word2' -r
complete -c tspin -l enable -d 'Enable specific highlighters' -r -f -a "numbers\t''
urls\t''
pointers\t''
dates\t''
paths\t''
quotes\t''
key-value-pairs\t''
uuids\t''
ip-addresses\t''
processes\t''
json\t''"
complete -c tspin -l disable -d 'Disable specific highlighters' -r -f -a "numbers\t''
urls\t''
pointers\t''
dates\t''
paths\t''
quotes\t''
key-value-pairs\t''
uuids\t''
ip-addresses\t''
processes\t''
json\t''"
complete -c tspin -l pager -d 'Override the default pager command used by tspin' -r
complete -c tspin -s f -l follow -d 'Follow the contents of a file'
complete -c tspin -s p -l print -d 'Print the output to stdout'
complete -c tspin -l disable-builtin-keywords -d 'Disable the highlighting of all builtin keyword groups (booleans, nulls, log severities and common REST verbs)'
complete -c tspin -l generate-bash-completions -d 'Print bash completions to stdout'
complete -c tspin -l generate-fish-completions -d 'Print fish completions to stdout'
complete -c tspin -l generate-zsh-completions -d 'Print zsh completions to stdout'
complete -c tspin -s h -l help -d 'Print help (see more with \'--help\')'
complete -c tspin -s V -l version -d 'Print version'
