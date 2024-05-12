"""
The template of the main script of the machine learning process
"""

from sklearn.neighbors import KNeighborsClassifier
from sklearn import preprocessing
from sklearn import metrics
import pandas as pd
import numpy as np
import os

# """
# input: current_x, current_y, platform_x, platform_y, state(vec4)

# output: a number between -1 and 1
# """
# def make_network():
#     network = nn.Sequential(
#         nn.Linear(8, 12), nn.ReLU(), 
#         nn.Linear(12, 12), nn.ReLU(),
#         nn.Linear(12, 1), nn.Tanh())
    
#     return network

level=os.environ.get("LV")
datapath=f"dataset/{level}.csv"

dataset = pd.read_csv(datapath, header = 0)
feature, label = dataset.iloc[:, :-1], dataset.iloc[:, [-1]]
neigh = KNeighborsClassifier(n_neighbors = 1)  
scaler=preprocessing.StandardScaler().fit(feature)

neigh.fit(scaler.transform(feature),label["label"])

last_x = 0
last_y = 0

game_ended = False

class MLPlay:
    def __init__(self,ai_name, *args, **kwargs):
        print(str(kwargs['game_params']['level']))

    def update(self, scene_info, *args, **kwargs):
        global last_x,last_y,game_ended
        if game_ended:
            os._exit(0)

        command="NONE"
        if (scene_info["status"] ==  "GAME_OVER" or
                scene_info["status"] == "GAME_PASS"):
            game_ended = True
            return "RESET"

        frame=scene_info["frame"]
        current_x=scene_info["ball"][0];
        current_y=scene_info["ball"][1]
        platform_x=scene_info["platform"][0]
        platform_y=scene_info["platform"][1]

        feature = np.array([frame,current_x-last_x,current_y-last_y, current_x, current_y, platform_x, platform_y]).reshape(1, -1)
        command = neigh.predict(scaler.transform(feature))[0]
    
        last_x=current_x
        last_y=current_y

        return command

    def reset(self):
        """
        Reset the status
        """
        self.ball_served = False
