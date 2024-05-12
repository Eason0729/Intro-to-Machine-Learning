import pygame

data = open('dataset/1.csv', 'a', buffering=4096)

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
        
        action=["L","L","R","R","F"]
        offset=[0,2,2,0,0]
        sensor=["L_sensor","L_T_sensor","R_T_sensor","R_sensor","F_sensor"]

        max_value=0
        min_value=100
        max_action=""

        for i in range(5):
            current_value=scene_info[sensor[i]]+offset[i]
            min_value=min(min_value,current_value)
            if current_value>max_value:
                max_value=current_value
                max_action=action[i]
            self.control_list["right_PWM"] = 100
            self.control_list["left_PWM"] = 100

        if min_value<8 and scene_info["F_sensor"]<35:
            if max_action=="R":
                self.control_list["left_PWM"] = 20
                self.control_list["right_PWM"] = -20
            elif max_action=="L":
                self.control_list["right_PWM"] = 20
                self.control_list["left_PWM"] = -20

        # if scene_info["F_sensor"]>6:
        #     self.control_list["right_PWM"]=60
        #     self.control_list["left_PWM"]=60

        # if scene_info["F_sensor"]>7:
        #     self.control_list["right_PWM"]+=60
        #     self.control_list["left_PWM"]+=60
        # else:
        #     self.control_list["right_PWM"]-=30
        #     self.control_list["left_PWM"]-=30
        
        # if scene_info["R_T_sensor"]>30:
        #     self.control_list["left_PWM"]+=75
        #     self.control_list["right_PWM"]-=75

        # if scene_info["L_T_sensor"]>30:
        #     self.control_list["right_PWM"]+=70
        #     self.control_list["left_PWM"]-=70

        # steer=scene_info["R_sensor"]-scene_info["L_sensor"]
        # if steer>30:
        #     self.control_list["left_PWM"]+=50
        # elif steer<30:
        #     self.control_list["right_PWM"]+=50

        if pygame.K_q in keyboard:
            data.write("%.4f,%.4f,%.4f,%.4f,%.4f,%.4f,%.4f"%(
                scene_info["L_sensor"],scene_info["R_sensor"],scene_info["L_T_sensor"],scene_info["R_T_sensor"],scene_info["F_sensor"],
                self.control_list["left_PWM"],self.control_list["right_PWM"]
            ) + '\n')
            
        return self.control_list

    def reset(self):
        """
        Reset the status
        """
        # print("reset ml script")
        pass
