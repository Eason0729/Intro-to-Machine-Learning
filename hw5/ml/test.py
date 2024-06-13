"""
The template of the main script of the machine learning process
"""

from ctypes import *

import pygame


class Player(Structure):
    _fields_ = [
        ("x", c_int32),
        ("y", c_int32),
        ("speed", c_int32),
        ("score", c_int32),
        ("power", c_int32),
        ("oil", c_float),
        ("lives", c_int32),
        ("angle", c_int32),
        ("gun_angle", c_int32),
        ("cooldown", c_int32),
    ]

    def from_dict(self, d: dict):
        self.x = d["x"]
        self.y = d["y"]
        self.speed = d["speed"]
        self.score = d["score"]
        self.power = d["power"]
        self.oil = d["oil"]
        self.lives = d["lives"]
        self.angle = d["angle"]
        self.gun_angle = d["gun_angle"]
        self.cooldown = d["cooldown"]
        return self


class Bullet(Structure):
    _fields_ = [("x", c_int32), ("y", c_int32)]

    def from_dict(self, d: dict):
        self.x = d["x"]
        self.y = d["y"]
        return self


class Station(Structure):
    _fields_ = [
        ("x", c_int32),
        ("y", c_int32),
        ("power", c_int32),
    ]

    def from_dict(self, d: dict):
        self.x = d["x"]
        self.y = d["y"]
        self.power = d["power"]
        return self


class Wall(Structure):
    _fields_ = [
        ("x", c_int32),
        ("y", c_int32),
        ("lives", c_int32),
    ]

    def from_dict(self, d: dict):
        self.x = d["x"]
        self.y = d["y"]
        self.lives = d["lives"]
        return self


class Info(Structure):
    _fields_ = [
        ("player", POINTER(Player)),
        ("teammates", POINTER(Player)),
        ("teammates_len", c_uint32),
        ("enemies", POINTER(Player)),
        ("enemies_len", c_uint32),
        ("bullets", POINTER(Bullet)),
        ("bullet_len", c_uint32),
        ("bullet_stations", POINTER(Station)),
        ("bullet_stations_len", c_uint32),
        ("oil_stations", POINTER(Station)),
        ("oil_stations_len", c_uint32),
        ("walls", POINTER(Wall)),
        ("walls_len", c_uint32),
    ]

    def from_dict(self, d: dict):
        self.player = pointer(Player().from_dict(d))

        self.teammates = (Player * len(d["teammate_info"]))()
        for i, t in enumerate(d["teammate_info"]):
            self.teammates[i] = Player().from_dict(t)
        self.teammates_len = len(d["teammate_info"])

        self.enemies = (Player * len(d["competitor_info"]))()
        for i, e in enumerate(d["competitor_info"]):
            self.enemies[i] = Player().from_dict(e)
        self.enemies_len = len(d["competitor_info"])

        self.bullets = (Bullet * len(d["bullets_info"]))()
        for i, b in enumerate(d["bullets_info"]):
            self.bullets[i] = Bullet().from_dict(b)
        self.bullet_len = len(d["bullets_info"])

        self.bullet_stations = (Station * len(d["bullet_stations_info"]))()
        for i, b in enumerate(d["bullet_stations_info"]):
            self.bullet_stations[i] = Station().from_dict(b)
        self.bullet_stations_len = len(d["bullet_stations_info"])

        self.oil_stations = (Station * len(d["oil_stations_info"]))()
        for i, o in enumerate(d["oil_stations_info"]):
            self.oil_stations[i] = Station().from_dict(o)
        self.oil_stations_len = len(d["oil_stations_info"])

        self.walls = (Wall * len(d["walls_info"]))()
        for i, w in enumerate(d["walls_info"]):
            self.walls[i] = Wall().from_dict(w)
        self.walls_len = len(d["walls_info"])

        return self


tank_rs = cdll.LoadLibrary("../libtank_rust.so")
tank_rs.init.argtypes = [c_char_p, c_int32]
tank_rs.init.restype = c_void_p
tank_rs.tick.argtypes = [c_void_p, POINTER(Info)]
tank_rs.tick.restype = c_int32
tank_rs.flush.argtypes = [c_void_p]

app = tank_rs.init(c_char_p("../output2".encode()), 10)


class MLPlay:
    def __init__(self, ai_name, *args, **kwargs):
        """
        Constructor

        @param ai_name A string "1P" or "2P" indicates that the `MLPlay` is used by
               which side.
        """
        self.side = ai_name
        print(f"Initial Game {ai_name} ml script")
        self.time = 0

    def update(self, scene_info: dict, keyboard=[], *args, **kwargs):
        """
        Generate the command according to the received scene information
        """
        if scene_info["status"] != "GAME_ALIVE":
            tank_rs.flush(app)

        command = "command"

        if pygame.K_d in keyboard:
            command = "TURN_RIGHT"
        elif pygame.K_a in keyboard:
            command = "TURN_LEFT"
        elif pygame.K_w in keyboard:
            command = "FORWARD"
        elif pygame.K_s in keyboard:
            command = "BACKWARD"

        if pygame.K_q in keyboard:
            command = "AIM_LEFT"
        elif pygame.K_e in keyboard:
            command = "AIM_RIGHT"

        if pygame.K_f in keyboard:
            command = "SHOOT"
        if command == "command":
            info = Info().from_dict(scene_info)

            action = tank_rs.tick(app, info)
            if action == 0:
                command = "FORWARD"
            elif action == 1:
                command = "BACKWARD"
            elif action == 2:
                command = "TURN_RIGHT"
            elif action == 3:
                command = "TURN_LEFT"
            elif action == 4:
                command = "AIM_RIGHT"
            elif action == 5:
                command = "AIM_LEFT"
            elif action == 6:
                command = "SHOOT"
            elif action == 7:
                command = "NONE"

        return [command]

    def reset(self):
        """
        Reset the status
        """
        print(f"reset Game {self.side}")
