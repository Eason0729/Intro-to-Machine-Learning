import pygame

data = open('dataset/1.csv', 'a', buffering=4096)

class MLPlay:
    def __init__(self, ai_name,*args,**kwargs):
        self.player_no = ai_name
        self.r_sensor_value = 0
        self.l_sensor_value = 0
        self.f_sensor_value = 0
        self.control_list = {"left_PWM": 0, "right_PWM": 0}

    def update(self, scene_info: dict, keyboard: list = [], *args, **kwargs):
        """
        Generate the command according to the received scene information
        """
        if scene_info["status"] != "GAME_ALIVE":
            data.flush()
            return "RESET"
        
        min_value=100
        sensor=["L_sensor","L_T_sensor","R_T_sensor","R_sensor","F_sensor"]
        for i in range(5):
            min_value=min(min_value,scene_info[sensor[i]])

        rf=scene_info["R_sensor"]*0.9+scene_info["R_T_sensor"]
        lf=scene_info["L_sensor"]*0.9+scene_info["L_T_sensor"]
        ff=scene_info["F_sensor"]

        self.control_list["right_PWM"]=300/rf+ff-17
        self.control_list["left_PWM"]=300/lf+ff-16

        if min_value<4:
            avg=(self.control_list["right_PWM"]+self.control_list["left_PWM"])/2
            if scene_info["F_sensor"]>10:
                avg=avg/3
            self.control_list["right_PWM"]-=avg
            self.control_list["left_PWM"]-=avg
            self.control_list["right_PWM"]*=2
            self.control_list["left_PWM"]*=2
        
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
