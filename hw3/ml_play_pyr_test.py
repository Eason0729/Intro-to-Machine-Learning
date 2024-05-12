import random
from pprint import pprint

import orjson
import pygame

from ctypes import *

libpyr = cdll.LoadLibrary("./pyr/target/release/libpyr.so")

app_ptr = c_ulonglong(0)

class Overall(Structure):
    _fields_ = [
        ("frame", c_uint64),
        # ("collision_mode", c_bool),
        ("score", c_int64),
        ("score_to_pass", c_int64),
        ("self_x", c_int64),
        ("self_y", c_int64),
        ("self_w", c_int64),
        ("self_h", c_int64),
        ("self_vel", c_int64),
        ("self_lv", c_int64),
        ("opponent_x", c_int64),
        ("opponent_y", c_int64),
        ("opponent_lv", c_int64),
    ]

class Food(Structure):
    _fields_= [
        ("h", c_int64),
        ("w", c_int64),
        ("x", c_int64),
        ("y", c_int64),
        ("score", c_int64),
        ("kind", c_int32),
    ]

libpyr.tick.argtypes = [c_ulonglong, POINTER(Overall), POINTER(Food), c_uint64]
libpyr.tick.restype = c_int32

libpyr.new_app.argtypes = []
libpyr.new_app.restype = c_ulonglong

libpyr.drop_app.argtypes = [c_ulonglong]
libpyr.drop_app.restype = None

libpyr.check_point.argtypes = [c_ulonglong]
libpyr.check_point.restype = None

def new_app():
    global app_ptr
    app_ptr=c_ulonglong(libpyr.new_app())

new_app()

def drop_app():
    global app_ptr
    libpyr.drop_app(app_ptr)

def check_point():
    global app_ptr
    libpyr.check_point(app_ptr)

def tick(scene_info: dict):
    overall=Overall()
    overall.frame=c_uint64(scene_info["frame"])
    overall.score=c_int64(scene_info["score"])
    overall.score_to_pass=c_int64(scene_info["score_to_pass"])
    overall.self_x=c_int64(scene_info["self_x"])
    overall.self_y=c_int64(scene_info["self_y"])
    overall.self_w=c_int64(scene_info["self_w"])
    overall.self_h=c_int64(scene_info["self_h"])
    overall.self_vel=c_int64(scene_info["self_vel"])
    overall.self_lv=c_int64(scene_info["self_lv"])
    overall.opponent_x=c_int64(scene_info["opponent_x"])
    overall.opponent_y=c_int64(scene_info["opponent_y"])
    overall.opponent_lv=c_int64(scene_info["opponent_lv"])

    n=len(scene_info["foods"])

    foods= (Food * n)()
    for i in range(n):
        foods[i]=Food()
        foods[i].h=c_int64(scene_info["foods"][i]["h"])
        foods[i].w=c_int64(scene_info["foods"][i]["w"])
        foods[i].x=c_int64(scene_info["foods"][i]["x"])
        foods[i].y=c_int64(scene_info["foods"][i]["y"])
        foods[i].score=c_int64(scene_info["foods"][i]["score"])
        kind=scene_info["foods"][i]["type"]
        if kind=="FOOD_1":
            foods[i].kind=c_int(1)
        elif kind=="FOOD_2":
            foods[i].kind=c_int(2)
        elif kind=="FOOD_3":
            foods[i].kind=c_int(3)
        elif kind=="GARBAGE_1":
            foods[i].kind=c_int(4)
        elif kind=="GARBAGE_2":
            foods[i].kind=c_int(5)
        elif kind=="GARBAGE_3":
            foods[i].kind=c_int(6)
    
    result=libpyr.tick(app_ptr,(pointer(overall)),(POINTER(Food))(foods),c_uint64(n))
    if result==1:
        return ["UP"]
    if result==2:
        return ["DOWN"]
    if result==3:
        return ["LEFT"]
    if result==4:
        return ["RIGHT"]
    return ["NONE"]

class MLPlay:
    def __init__(self,ai_name,*args,**kwargs):
        print("Initial ml script")

    def update(self, scene_info: dict, keyboard:list=[], *args, **kwargs):
        """
        Generate the command according to the received scene information
        """

        if scene_info["status"] == "GAME_ALIVE":
            return tick(scene_info)
        else:
            check_point()
        
    def reset(self):
        """
        Reset the status
        """
        print("reset ml script")
        pass
