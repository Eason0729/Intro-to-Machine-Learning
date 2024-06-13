#!/bin/bash
export LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libstdc++.so.6
if [ "$XDG_SESSION_TYPE" == "x11" ]; then
    export DISPLAY=:0.0
fi
cd TankMan
python -m mlgame -f 400 \
-i ../ml/collect1.py -i ../ml/test.py \
. --green_team_num 1 --blue_team_num 1 --is_manual 1 \
--frame_limit 1000 > /dev/null
cd ..
