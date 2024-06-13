#!/bin/bash

export LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libstdc++.so.6
if [ "$XDG_SESSION_TYPE" == "x11" ]; then
    export DISPLAY=:0.0
fi

for i in  $(seq 1 $1);
do
   echo "epoch $i"
   cd TankMan
   export EPSILON=$i
   timeout 200 python -m mlgame -f 3000 \
   -i ../ml/collect1.py -i ../ml/collect2.py \
   . --green_team_num 1 --blue_team_num 1 --is_manual 1 \
   --frame_limit 1000 > /dev/null
   cd ..

   MODEL_PATH="output1" ./train
   MODEL_PATH="output2" ./train
   
   rm output1/dataset
   rm output2/dataset
done
