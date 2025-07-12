#!/bin/zsh

# Custom completion functions for donna dynamic values
_donna_alias_groups() {
    local groups=(${(f)"$(donna _autocompletion-values alias-groups 2>/dev/null)"})
    _describe 'alias groups' groups
}

_donna_libraries() {
    local libs=(${(f)"$(donna _autocompletion-values libraries 2>/dev/null)"})
    _describe 'libraries' libs
}

_donna_project_types() {
    local types=(${(f)"$(donna _autocompletion-values project-types 2>/dev/null)"})
    _describe 'project types' types
}

_donna_projects() {
    local projects=(${(f)"$(donna _autocompletion-values projects 2>/dev/null)"})
    _describe 'projects' projects
}

# Override specific completions
_donna_override() {
    local context curcontext="$curcontext" state line
    typeset -A opt_args

    # Handle specific cases that need dynamic completion
    case "$words[$CURRENT-1]" in
        -g|--alias-groups)
            if [[ "$words[*]" == *"create project"* ]]; then
                _donna_alias_groups
                return 0
            fi
            ;;
        -l|--library)
            if [[ "$words[*]" == *"create project"* ]] || [[ "$words[*]" == *"open project"* ]]; then
                _donna_libraries
                return 0
            fi
            ;;
        -t|--project-type)
            if [[ "$words[*]" == *"create project"* ]]; then
                _donna_project_types
                return 0
            fi
            ;;
    esac

    # Handle positional arguments
    case "$words[*]" in
        "donna open project "*|*"open project "*)
            if [[ $CURRENT -eq 4 ]]; then
                _donna_projects
                return 0
            fi
            ;;
        "donna forget alias-group "*|*"forget alias-group "*)
            if [[ $CURRENT -eq 4 ]]; then
                _donna_alias_groups
                return 0
            fi
            ;;
        "donna forget library "*|*"forget library "*)
            if [[ $CURRENT -eq 4 ]]; then
                _donna_libraries
                return 0
            fi
            ;;
        "donna forget project-type "*|*"forget project-type "*)
            if [[ $CURRENT -eq 4 ]]; then
                _donna_project_types
                return 0
            fi
            ;;
        "donna set default-lib "*|*"set default-lib "*)
            if [[ $CURRENT -eq 4 ]]; then
                _donna_libraries
                return 0
            fi
            ;;
    esac

    # Let the original clap completion handle everything else
    _donna_original "$@"
}

# Save and replace the completion function
functions[_donna_original]=$functions[_donna]
compdef _donna_override donna