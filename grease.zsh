#compdef grease

autoload -U is-at-least

_grease() {
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
'-e+[Execute inline code]:EVAL:_default' \
'--eval=[Execute inline code]:EVAL:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
'-V[Print version]' \
'--version[Print version]' \
'::file -- File to execute:_default' \
":: :_grease_commands" \
"*::: :->grease" \
&& ret=0
    case $state in
    (grease)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:grease-command-$line[2]:"
        case $line[2] in
            (completions)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
':shell -- Shell to generate completions for:(bash elvish fish powershell zsh)' \
&& ret=0
;;
(manpage)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_grease__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:grease-help-command-$line[1]:"
        case $line[1] in
            (completions)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(manpage)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_grease_commands] )) ||
_grease_commands() {
    local commands; commands=(
'completions:Generate shell completions' \
'manpage:Generate manpage' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'grease commands' commands "$@"
}
(( $+functions[_grease__completions_commands] )) ||
_grease__completions_commands() {
    local commands; commands=()
    _describe -t commands 'grease completions commands' commands "$@"
}
(( $+functions[_grease__help_commands] )) ||
_grease__help_commands() {
    local commands; commands=(
'completions:Generate shell completions' \
'manpage:Generate manpage' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'grease help commands' commands "$@"
}
(( $+functions[_grease__help__completions_commands] )) ||
_grease__help__completions_commands() {
    local commands; commands=()
    _describe -t commands 'grease help completions commands' commands "$@"
}
(( $+functions[_grease__help__help_commands] )) ||
_grease__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'grease help help commands' commands "$@"
}
(( $+functions[_grease__help__manpage_commands] )) ||
_grease__help__manpage_commands() {
    local commands; commands=()
    _describe -t commands 'grease help manpage commands' commands "$@"
}
(( $+functions[_grease__manpage_commands] )) ||
_grease__manpage_commands() {
    local commands; commands=()
    _describe -t commands 'grease manpage commands' commands "$@"
}

if [ "$funcstack[1]" = "_grease" ]; then
    _grease "$@"
else
    compdef _grease grease
fi
