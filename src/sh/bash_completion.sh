#!/bin/bash

# Custom completion functions for donna dynamic values
_donna_complete_alias_groups() {
    local groups=$(donna _autocompletion-values alias-groups 2>/dev/null || echo "")
    COMPREPLY=($(compgen -W "$groups" -- "${COMP_WORDS[COMP_CWORD]}"))
}

_donna_complete_libraries() {
    local libs=$(donna _autocompletion-values libraries 2>/dev/null || echo "")
    COMPREPLY=($(compgen -W "$libs" -- "${COMP_WORDS[COMP_CWORD]}"))
}

_donna_complete_project_types() {
    local types=$(donna _autocompletion-values project-types 2>/dev/null || echo "")
    COMPREPLY=($(compgen -W "$types" -- "${COMP_WORDS[COMP_CWORD]}"))
}

_donna_complete_projects() {
    local projects=$(donna _autocompletion-values projects 2>/dev/null || echo "")
    COMPREPLY=($(compgen -W "$projects" -- "${COMP_WORDS[COMP_CWORD]}"))
}

# Override specific completions only where we need dynamic data
_donna_override() {
    local cur prev words cword
    _init_completion || return

    # Only handle specific cases that need dynamic completion
    case "${prev}" in
        # For flags that expect dynamic values
        -g|--alias-groups)
            if [[ "${words[*]}" == *"create project"* ]]; then
                _donna_complete_alias_groups
                return 0
            fi
            ;;
        -l|--library)
            if [[ "${words[*]}" == *"create project"* ]] || [[ "${words[*]}" == *"open project"* ]]; then
                _donna_complete_libraries
                return 0
            fi
            ;;
        -t|--project-type)
            if [[ "${words[*]}" == *"create project"* ]]; then
                _donna_complete_project_types
                return 0
            fi
            ;;
    esac

    # Handle positional arguments that need dynamic completion
    case "${words[*]}" in
        "donna open project "*)
            if [[ $cword -eq 3 ]]; then
                _donna_complete_projects
                return 0
            fi
            ;;
        "donna forget alias-group "*)
            if [[ $cword -eq 3 ]]; then
                _donna_complete_alias_groups
                return 0
            fi
            ;;
        "donna forget library "*)
            if [[ $cword -eq 3 ]]; then
                _donna_complete_libraries
                return 0
            fi
            ;;
        "donna forget project-type "*)
            if [[ $cword -eq 3 ]]; then
                _donna_complete_project_types
                return 0
            fi
            ;;
        "donna set default-lib "*)
            if [[ $cword -eq 3 ]]; then
                _donna_complete_libraries
                return 0
            fi
            ;;
    esac

    # Let the original clap completion handle everything else
    _donna_original "$@"
}

# Save the original completion function
eval "$(declare -f _donna | sed 's/_donna/_donna_original/')"

# Replace with our override
complete -F _donna_override donna