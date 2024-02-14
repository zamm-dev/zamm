#!/bin/bash

SESSION_NAME="zamm"

tmux has-session -t $SESSION_NAME

if [ $? != 0 ]; then
    tmux new-session -d -s $SESSION_NAME -n 'processes' 'yarn workspace gui storybook --ci'
    tmux split-window -h 'yarn tauri dev'
    tmux new-window -t $SESSION_NAME:1 -n 'shell'
    tmux split-window -t $SESSION_NAME:1
    tmux send-keys -t $SESSION_NAME:1.1 'cd src-svelte' Enter
    tmux split-window -t $SESSION_NAME:1.1
    tmux send-keys -t $SESSION_NAME:1.2 'cd src-tauri' Enter
    tmux select-layout -t $SESSION_NAME:1 even-horizontal
fi

tmux attach-session -t $SESSION_NAME
