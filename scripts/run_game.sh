#!/usr/bin/env bash

HALITE=halite
HALITE_CLIENT=hlt
ROOT_DIR=$(dirname $(dirname $(readlink -f $0)))
GAME_DIR=$ROOT_DIR/target/gameround
GAME_COUNT=$GAME_DIR/.count

init() {
    if [ ! -d $GAME_DIR ]; then
        mkdir -p $GAME_DIR
    fi

    if [ ! -e $GAME_COUNT ]; then
        touch $GAME_COUNT
        echo 0 >> $GAME_COUNT
    fi
}

has_executables() {
    if ! hash $HALITE 2>/dev/null; then
        echo "halite program not found"
        exit
    fi

}

increment_game_count() {
    oldnum=`cut -d ',' -f2 $GAME_COUNT`
    newnum=`expr $oldnum + 1`
    sed -i "s/$oldnum\$/$newnum/g" $GAME_COUNT
}

build_binary() {
    cargo rustc --release -q -- -Awarnings
}

execute() {
    tmp_game_dir=$GAME_DIR/$(cat $GAME_COUNT)
    mkdir -p $tmp_game_dir
    cd $tmp_game_dir
    $HALITE -d "240 160" "$ROOT_DIR/target/release/MyBot" "$ROOT_DIR/target/release/MyBot"
}

has_executables
init
build_binary
increment_game_count
execute
