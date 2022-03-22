import random
import time
import keyboard
import json
import os
from rich import print

# Declaration of variables ------------------------------------------------------------------------
population = 0
born = 0
dead = 0

money = 0
food = 0
biome = []

BREAK = False
temp0 = 0
temp1 = 0
temp2 = 0
temp3 = 0
os.system('cls')  # Clears old terminal lines

gameSave = []


# keyboard.press('f11')  # Puts terminal in fullscreen mode

class Biome:
    pass


Biome.info = []


def SAVE():
    global gameSave
    json.dump(gameSave, open('save.json', 'w'))


def LOAD():
    global gameSave
    try:
        gameSave = json.load(open('save.json', 'r'))
    except FileNotFoundError:
        print("[red]No save file found!")


def percentageIncrease(number, percentage_increase):
    percentage_increase = (percentage_increase / 100) + 1
    result = number * percentage_increase
    return result


def percentageDecrease(number, percentage_decrease):
    percentage_decrease = number * ((100 - percentage_decrease) / 100)
    result = percentage_decrease
    return result


def createPopulation(base_population_size, base_money, develop_time, time_multiplier):
    global population
    global born
    global dead
    global food
    global money

    os.system('cls' if os.name == 'nt' else 'clear')  # Clears terminal

    print("Creating population...")
    print(f"\nStarting population size: {base_population_size}\n"
          f"Starting money amount: {base_money}\n"
          f"Amount of time for population to develop: {develop_time}\n\n")

    money = base_money

    start_process = time.process_time()  # Start time calculation

    # Population calculation ----------------------------------------------------------------------
    percent = develop_time
    while develop_time != 0:
        born += random.randint(1, 4)
        dead += random.randint(0, 2)
        print(f"{int(round(develop_time / percent, 2) * 100)}% left..", end="\r")
        develop_time -= 1

    born += percentageIncrease(born, 5).__round__() * time_multiplier
    dead += percentageIncrease(dead, 1).__round__() * time_multiplier
    population = born - dead + base_population_size
    print(f"[bold]Population: [bold]{population}[/]\n"
          f"People born: [bold]{born}[/]\n"
          f"People dead: [bold]{dead}[/]")

    end_process = time.process_time()  # End time calculation

    # Time calculation ----------------------------------------------------------------------------
    time_of_process = end_process - start_process
    if time_of_process == 0:
        print("\nCompleted very quickly...")
    else:
        print(f"\nCompleted in {time_of_process} seconds...")


def createLandscape(biome_num):
    # Biomes 0-7 are normal. Biomes 8-10 are dangerous --------------------------------------------
    Biome.biomes = ["[#00bf2d]grassland", "[#998642]savanna", "[#d1cdc2]taiga", "[green]forest",
                    "[#f7f372]beach", "[#7691e8]mountains", "[green]hills", "[#b8ab1d]desert",
                    "[#ff4d00]lava lake", "[#6b8a89]treacherous cliffs", "[bold white]icy arctic"]
    Biome.info.append(biome_num)
    formatted_biome = Biome.biomes[Biome.info[0]]

    # Elevation calculation -----------------------------------------------------------------------
    Biome.low_elevations = [610, 100, 100, 900, 0, 1500, 30, 150, 0, 2000, 0]
    Biome.high_elevations = [1220, 500, 300, 1500, 5, 8850, 150, 2600, 500, 7000, 500]
    Biome.info.append(random.randint(Biome.low_elevations[biome_num], Biome.high_elevations[biome_num]))
    Biome.set_elevation = Biome.info[1]

    # Temperature calculation ---------------------------------------------------------------------
    Biome.temperatures = ["[#5468ff]-25", "[#8a96f2]-10", "[#bfc7ff]10", "[#ffd0bf]20",
                          "[#ff6c47]25", "[#ff3b21]35", "[#ff3636]40"]
    Biome.average_temperatures = [3, 4, 3, 3, 2, 1, 2, 5, 6, 1, 0]
    Biome.info.append(Biome.average_temperatures[biome_num])

    # Biome description and details ---------------------------------------------------------------
    print("Biome:", formatted_biome)
    if Biome.info[0] == 8 or Biome.info[0] == 9 or Biome.info[0] == 10:
        print("[red]Very dangerous!")
    print(f"Average temperature: {Biome.temperatures[Biome.info[2]]}" + "°C")
    print(f"Meters above sea level: [bold]{Biome.set_elevation}m[/]")


createLandscape(random.randint(0, 10))

# User input to create a population ---------------------------------------------------------------
while not BREAK:
    temp0 = int(input("\nEnter population start size: "))
    temp1 = int(input("Enter population base money: "))
    # temp2 = int(input("Enter population start food: "))
    temp3 = int(input("Enter population growth time: "))
    print("\nAre you sure you want to create a population with the "
          "following [white]stats([green]y[/], [red]n[/])?..")
    print(f"[bold]Starting population size: [bold]{temp0}[/]\n"
          f"Starting money amount: [bold]{temp1}[/]\n"
          f"Amount of time for population to develop: [bold]{temp3}[/]\n\n")
    while not BREAK:
        if keyboard.read_key() == 'y':
            BREAK = True

        if keyboard.read_key() == 'n':
            break

createPopulation(temp0, temp1, temp3, 15)
print("\n[blink]Press enter to continue...")
keyboard.wait('enter')
os.system("exit()")  # Closes terminal
