#!/bin/bash
cd $1
./configure --prefix=$HOME/.rustv/$2
make
make install
