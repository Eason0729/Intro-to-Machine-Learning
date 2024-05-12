"""
The template of the main script of the machine learning process
"""
import math
import random
import os

last_x = 0
last_y = 0

class MLPlay:
    def __init__(self,ai_name, *args, **kwargs):
        """
        Constructor
        """
        print(ai_name)

    def update(self, scene_info, *args, **kwargs):
        global last_x,last_y
        frame=scene_info["frame"]
        current_x=scene_info["ball"][0];
        current_y=scene_info["ball"][1]
        platform_x=scene_info["platform"][0]
        platform_y=scene_info["platform"][1]
        command="NONE"
        if scene_info["status"] == "GAME_OVER":
            print("Game Status: Over",flush=True)
            os._exit(1)
        if scene_info["status"] == "GAME_PASS":
            print("Game Status: Pass",flush=True)
            os._exit(0)
        if not scene_info["ball_served"]:
            command = "SERVE_TO_LEFT"
        else:
            target=100
            if current_y>last_y:
                target=400+current_x-(current_x-last_x)*(current_y-platform_y)/(current_y-last_y)
                if math.floor(target/200)%2==0:
                    target=target%200
                else:
                    target=200-(target%200)
            target-=20
            delta=target-platform_x
            if abs(delta)>5 or random.random()>0.5:
                if delta>0:
                    command = "MOVE_RIGHT"
                else:
                    command = "MOVE_LEFT"
        print(f"{frame},{current_x-last_x},{current_y-last_y},{current_x},{current_y},{platform_x},{platform_y},{command}")
        last_x=current_x
        last_y=current_y
        return command
    def reset(self):
        """
        Reset the status
        """
        self.ball_served = False
