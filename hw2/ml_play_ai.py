import pygame
import numpy as np
import pickle

with open('model.pkl', 'rb') as f:
    clf = pickle.load(f)

class MLPlay:
    def __init__(self, ai_name,*args,**kwargs):
        self.player_no = ai_name
        self.r_sensor_value = 0
        self.l_sensor_value = 0
        self.f_sensor_value = 0
        self.control_list = {"left_PWM": 0, "right_PWM": 0}
        # print("Initial ml script")

    def update(self, scene_info: dict, keyboard: list = [], *args, **kwargs):
        """
        Generate the command according to the received scene information
        """
        if scene_info["status"] != "GAME_ALIVE":
            return "RESET"
        
        feature = np.array([scene_info["L_sensor"],scene_info["R_sensor"],scene_info["L_T_sensor"],scene_info["R_T_sensor"],scene_info["F_sensor"]]).reshape(1, -1)
            
        speed=clf.predict(feature)[0]

        self.control_list["right_PWM"]=speed[1]
        self.control_list["left_PWM"]=speed[0]

        return self.control_list

    def reset(self):
        """
        Reset the status
        """
        # print("reset ml script")
        pass
