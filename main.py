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

gameSave = []
populationInfo = []
biomeInfo = []


# keyboard.press('f11')  # Puts terminal in fullscreen mode

class Biome:
    pass


Biome.info = []


def clearTerminal():
    os.system('cls' if os.name == 'nt' else 'clear')


clearTerminal()


# Saves game into save.json
def SAVE(save_enabled):
    global gameSave

    if save_enabled:
        json.dump(gameSave, open('save.json', 'w'))

    if os.path.exists('save.json'):
        return True
    else:
        return False


# Loads game save file
def LOAD():
    global gameSave
    global populationInfo
    global biomeInfo

    try:
        gameSave.clear()
        gameSave = json.load(open('save.json', 'r'))
        populationInfo = gameSave[:len(gameSave) - 3]
        biomeInfo = gameSave[len(gameSave) - 3:]

    except FileNotFoundError:
        print("[red]No save file found!")


def printLogo():
    print("d888888P dP     dP   88888888b     888888ba   .88888.   888888ba  dP     dP dP       "
          "  .d888888  d888888P  .88888.   888888ba\n",
          "  88    88     88   88            88    `8b d8'   `8b  88    `8b 88     88 88        "
          "d8'    88     88    d8'   `8b  88    `8b   \n",
          "  88    88aaaaa88a a88aaaa       a88aaaa8P' 88     88 a88aaaa8P' 88     88 88        "
          "88aaaaa88a    88    88     88 a88aaaa8P'   \n",
          "  88    88     88   88            88        88     88  88        88     88 88        "
          "88     88     88    88     88  88   `8b.   \n",
          "  88    88     88   88            88        Y8.   .8P  88        Y8.   .8P 88        "
          "88     88     88    Y8.   .8P  88     88   \n",
          "  dP    dP     dP   88888888P     dP         `8888P'   dP        `Y88888P' 88888888P "
          "88     88     dP     `8888P'   dP     dP   \n")


def percentageIncrease(number, percentage_increase):
    percentage_increase = (percentage_increase / 100) + 1
    result = number * percentage_increase
    return result


def percentageDecrease(number, percentage_decrease):
    percentage_decrease = number * ((100 - percentage_decrease) / 100)
    result = percentage_decrease
    return result


# Main functions ----------------------------------------------------------------------------------
def createPopulation(base_population_size, base_money, develop_time, time_multiplier):
    global population
    global born
    global dead
    global food
    global money

    clearTerminal()

    print("Creating population...")
    print(f"\nStarting population size: {base_population_size}\n"
          f"Starting money amount: {base_money}\n"
          f"Amount of weeks for population to develop: {develop_time}\n\n")

    money = base_money

    start_process = time.process_time()  # Start time calculation

    # Population calculation ----------------------------------------------------------------------
    percent = develop_time
    time_step_interval = (develop_time / 100).__round__()
    print("100% left...", end="\r")
    while develop_time != 0:
        born += random.randint(1, 4)
        dead += random.randint(0, 2)
        if time_step_interval == 0:
            # Optimised population percentage
            # Instead of printing the percentage every time, it will print when the percent changes
            time_step_interval = (develop_time / 100).__round__()
            print(f"{int(round(develop_time / percent, 2) * 100)}% left..", end="\r")
        develop_time -= 1
        time_step_interval -= 1

    born += percentageIncrease(born, 5).__round__() * time_multiplier  # Increases born by 5%
    dead += percentageIncrease(dead, 1).__round__() * time_multiplier  # Increases dead by 1%
    population = born - dead + base_population_size
    print(f"[bold]Population: [bold]{population}[/]\n"
          f"People born: [bold]{born}[/]\n"
          f"People dead: [bold]{dead}[/]")
    returner = [population, born, dead]

    end_process = time.process_time()  # End time calculation

    # Time calculation ----------------------------------------------------------------------------
    time_of_process = end_process - start_process
    if time_of_process == 0:
        print("\nCompleted very quickly...")
    else:
        print(f"\nCompleted in {time_of_process} seconds...")
    return returner


def doXStepsInTime(x):
    global populationInfo
    while x != 0:
        populationInfo[1] += random.randint(1, 4)  # Born
        populationInfo[2] += random.randint(0, 2)  # Dead
        x -= 1
    populationInfo[0] = populationInfo[1] - populationInfo[2]  # Population
    returner = populationInfo
    return returner


def biomeDetailsPrinter(biome_info):
    # Biomes 0-7 are normal. Biomes 8-10 are dangerous --------------------------------------------
    Biome.biomes = ["[#00bf2d]grassland", "[#998642]savanna", "[#d1cdc2]taiga", "[green]forest",
                    "[#f7f372]beach", "[#7691e8]mountains", "[green]hills", "[#b8ab1d]desert",
                    "[#ff4d00]lava lake", "[#6b8a89]treacherous cliffs", "[bold white]icy arctic"]

    Biome.temperatures = ["[#5468ff]-25", "[#8a96f2]-10", "[#bfc7ff]10", "[#ffd0bf]20",
                          "[#ff6c47]25", "[#ff3b21]35", "[#ff3636]40"]

    formatted_biome = Biome.biomes[biome_info[0]]
    if biome_info[0] == 8 or biome_info[0] == 9 or biome_info[0] == 10:
        print("[red]Very dangerous!")
    print("Biome:", formatted_biome)
    print(f"Average temperature: {Biome.temperatures[biome_info[1]]}" + "°C")
    print(f"Altitude: [bold]{biome_info[2]}m[/]")


def createLandscape(biome_num):
    # Adds name to Biome.info[0]
    Biome.info.append(biome_num)

    # Temperature calculation ---------------------------------------------------------------------
    Biome.average_temperatures = [3, 4, 3, 3, 2, 1, 2, 5, 6, 1, 0]
    # Adds temperature to Biome.info[1]
    Biome.info.append(Biome.average_temperatures[biome_num])

    # Elevation calculation -----------------------------------------------------------------------
    Biome.low_elevations = [610, 100, 100, 900, 0, 1500, 30, 150, 0, 2000, 0]
    Biome.high_elevations = [1220, 500, 300, 1500, 5, 8850, 150, 2600, 500, 7000, 500]
    # Adds elevation to Biome.info[2]
    Biome.info.append(random.randint(Biome.low_elevations[biome_num], Biome.high_elevations[biome_num]))

    returner = Biome.info
    biomeDetailsPrinter(returner)  # Prints biome details
    return returner


# User input to create a population ---------------------------------------------------------------
if not SAVE(False):
    tempSave = []
    temp0 = 0
    while not BREAK:
        print("Before you create your population you will need to create a biome.\n")
        biomeInfo = createLandscape(random.randint(0, 10))
        print("\nPress [green]y[/] to create this biome, or [red]n[/] to generate another.")
        while not BREAK:
            if keyboard.read_key() == 'y':
                for b in biomeInfo:
                    tempSave.append(b)
                temp0 += 5
                BREAK = True

            if keyboard.read_key() == 'n':
                biomeInfo.clear()
                clearTerminal()
                biomeInfo = createLandscape(random.randint(0, 10))
                print("\nPress [green]y[/] to create this biome, or [red]n[/] to generate another.")
                temp0 += 5

    BREAK = False
    while not BREAK:
        clearTerminal()
        # A very bad way to clear input, but it works (lol)
        while temp0 != 0:
            keyboard.press('backspace')
            temp0 -= 1
        temp0 = int(input("Enter the population start size: "))
        temp1 = int(input("Enter the population base money: "))
        temp3 = int(input("Enter the amount of weeks for your population to grow: "))

        print("\nAre you sure you want to create a population with the "
              "following [white]stats([green]y[/], [red]n[/])?..")
        print(f"[bold]Starting population size: [bold]{temp0}[/]\n"
              f"Starting money amount: [bold]{temp1}[/]\n"
              f"Amount of weeks for population to develop: [bold]{temp3}[/]")
        if temp3 > 100000000:
            print(f"[red]Caution! This many weeks may take a long time to complete!")
        print("\n\n")

        BREAK = False
        while not BREAK:
            if keyboard.read_key() == 'y':
                BREAK = True

            if keyboard.read_key() == 'n':
                break

    populationInfo = createPopulation(temp0, temp1, temp3, 15)  # This list holds the population info
    for p in populationInfo:
        gameSave.append(p)
    for s in tempSave:
        gameSave.append(s)
    SAVE(True)
    print("\n[blink]Press enter to continue...")
    keyboard.wait('enter')

# os.system("exit()")  # Closes terminal
LOAD()
clearTerminal()
GAME_PLAYING = True
printLogo()

# Main game loop ----------------------------------------------------------------------------------
while GAME_PLAYING:
    LEAVENODE = False
    if keyboard.read_key() == 'e' and LEAVENODE is False:
        clearTerminal()
        print("Evolution node")
        while True:
            if keyboard.read_key() == 'enter':
                clearTerminal()
                stepsInTime = doXStepsInTime(10)
                print(f"[bold]Population: [bold]{stepsInTime[0]}[/]\n"
                      f"People born: [bold]{stepsInTime[1]}[/]\n"
                      f"People dead: [bold]{stepsInTime[2]}[/]")
            elif keyboard.read_key() == 'backspace':
                clearTerminal()
                printLogo()
                LEAVENODE = True
                break
    if keyboard.read_key() == 'b':
        clearTerminal()
        biomeDetailsPrinter(biomeInfo)

    if keyboard.read_key() == 'l':
        clearTerminal()
        print("Law node")

    if keyboard.read_key() == 'w':
        clearTerminal()
        print("War node")

    if keyboard.read_key() == 's':
        clearTerminal()
        print("Settings node")

    # Shows all keybindings
    if keyboard.read_key() == 'h':
        clearTerminal()
        print("Help node")

    # Quits game
    if keyboard.read_key() == 'q':
        SAVE(True)
        GAME_PLAYING = False
