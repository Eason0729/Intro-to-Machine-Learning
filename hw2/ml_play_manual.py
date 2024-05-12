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
        if pygame.K_w in keyboard or pygame.K_UP in keyboard:
            self.control_list["left_PWM"] = 200
            self.control_list["right_PWM"] = 200
        elif pygame.K_a in keyboard or pygame.K_LEFT in keyboard:
            self.control_list["left_PWM"] = -150
            self.control_list["right_PWM"] = 150
        elif pygame.K_d in keyboard or pygame.K_RIGHT in keyboard:
            self.control_list["left_PWM"] = 150
            self.control_list["right_PWM"] = -150
        elif pygame.K_s in keyboard or pygame.K_DOWN in keyboard:
            self.control_list["left_PWM"] = -150
            self.control_list["right_PWM"] = -150
        else:

            self.control_list["left_PWM"] = 100
            self.control_list["right_PWM"] = 100
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
        data.flush()
        pass