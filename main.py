import random
import time
import keyboard
# import json
import os
from rich import print

# Declaration of variables ------------------------------------------------------------------------
population = 0
born = 0
dead = 0

money = 0
tax_percentage = 1001  # (0.1%)
food = 0
happiness = 2  # 0 --> 4 (0 = hated, 1 = disliked 2 = neutral, 3 = liked, 4 = loved)
happiness_emoji = ["😡", "😠", "😐", "🙂", "😍"]
biome = []

# Decorative variables
weeks_passed = 0
months = ["January", "February", "March", "April", "May", "June", "July", "August", "September",
          "October", "November", "December"]

BREAK = False
temp0 = 0
temp1 = 0
temp2 = 0
temp3 = 0

biome_info = []


# keyboard.press('f11')  # Puts terminal in full screen mode

class Biome:
    pass


Biome.info = []


def clearTerminal():
    os.system('cls' if os.name == 'nt' else 'clear')

def clearInput():
    # A very bad way to clear input, but it works (lol)
    num = 150
    while num != 0:
        keyboard.press('backspace')
        num -= 1


clearTerminal()


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

    end_process = time.process_time()  # End time calculation

    # Time calculation ----------------------------------------------------------------------------
    time_of_process = end_process - start_process
    if time_of_process == 0:
        print("\nCompleted very quickly...")
    else:
        print(f"\nCompleted in {time_of_process} seconds...")



def doXStepsInTime(x):
    global population
    global born
    global dead
    global weeks_passed
    while x != 0:
        born += random.randint(1, 4)  # Born
        dead += random.randint(0, 2)  # Dead
        weeks_passed += 1
        x -= 1
    population = born - dead  # Population


def biomeDetailsPrinter(biome_info_lst):
    # Biomes 0-7 are normal. Biomes 8-10 are dangerous --------------------------------------------
    Biome.biomes = ["[#00bf2d]grassland", "[#998642]savanna", "[#d1cdc2]taiga", "[green]forest",
                    "[#f7f372]beach", "[#7691e8]mountains", "[green]hills", "[#b8ab1d]desert",
                    "[#ff4d00]lava lake", "[#6b8a89]treacherous cliffs", "[bold white]icy arctic"]

    Biome.temperatures = ["[#5468ff]-25", "[#8a96f2]-10", "[#bfc7ff]10", "[#ffd0bf]20",
                          "[#ff6c47]25", "[#ff3b21]35", "[#ff3636]40"]

    formatted_biome = Biome.biomes[biome_info_lst[0]]
    if biome_info_lst[0] == 8 or biome_info_lst[0] == 9 or biome_info_lst[0] == 10:
        print("[red]Very dangerous!")
    print("Biome:", formatted_biome)
    print(f"Average temperature: {Biome.temperatures[biome_info_lst[1]]}" + "°C")
    print(f"Altitude: [bold]{biome_info_lst[2]}m[/]")


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


# User input to start game ------------------------------------------------------------------------
# Biome creation
tempSave = []
while not BREAK:
    print("Before you create your population you will need to create a biome.\n")
    biome_info = createLandscape(random.randint(0, 10))
    print("\nPress [green]y[/] to create this biome, or [red]n[/] to generate another.")
    while not BREAK:
        if keyboard.read_key() == 'y':
            for b in biome_info:
                tempSave.append(b)
            BREAK = True
        if keyboard.read_key() == 'n':
            biome_info.clear()
            clearTerminal()
            biome_info = createLandscape(random.randint(0, 10))
            print("\nPress [green]y[/] to create this biome, or [red]n[/] to generate another.")

# Population creation
BREAK = False
while not BREAK:
    clearTerminal()
    clearInput()
    temp0 = int(input("Enter the population start size: "))
    temp1 = int(input("Enter the population base money: "))
    temp3 = int(input("Enter the amount of weeks for your population to grow: "))
    weeks_passed = temp3

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
            break

        if keyboard.read_key() == 'n':
            break

# This list holds the population info
createPopulation(temp0, temp1, temp3, 15)
clearInput()
input("\nPress enter to continue...")

game_playing = True


# Main game loop ----------------------------------------------------------------------------------
while game_playing:
    # Main node(Shows most info in one place)
    clearTerminal()
    printLogo()
    # Prints amount of time passed
    if weeks_passed > 12:
        print(f"[bold]Year: [bold]{weeks_passed // 12}[/]\n"
              f"Month: [bold]{months[weeks_passed % 12]}[/]\n")
    else:
        print(f"Week: [bold]{weeks_passed}[/]\n"
              f"Month: [bold]{months[weeks_passed % 12]}[/]\n")
    # Prints population info
    print(f"[bold]Population: [bold]{population}[/]\n"
          f"People born: [bold]{born}[/]\n"
          f"People dead: [bold]{dead}[/]\n")
    print(f"[bold]Money: [green]${money}[/]\n")

    if keyboard.read_key() == 'e':
        clearTerminal()
        print("Evolution node")
        while True:
            if keyboard.read_key() == 'enter':
                clearTerminal()
                doXStepsInTime(10)
                print(f"[bold]Population: [bold]{population}[/]\n"
                      f"People born: [bold]{born}[/]\n"
                      f"People dead: [bold]{dead}[/]")
                # Calculates money from taxes
                money += (born // tax_percentage)
                print(f"[bold]Money: [green]${money}[/]\n")
            elif keyboard.read_key() == 'backspace':
                break

    if keyboard.read_key() == 'b':
        clearTerminal()
        biomeDetailsPrinter(biome_info)
        while True:
            if keyboard.read_key() == 'backspace':
                break

    if keyboard.read_key() == 'l':
        clearTerminal()
        print("Law node")
        while True:
            if keyboard.read_key() == 't':
                clearTerminal()
                print("Taxes node")

            if keyboard.read_key() == 'm':
                clearTerminal()
                print("Money node")

            elif keyboard.read_key() == 'backspace':
                break

    if keyboard.read_key() == 'w':
        clearTerminal()
        print("War node")
        while True:
            if keyboard.read_key() == 'backspace':
                break

    if keyboard.read_key() == 's':
        clearTerminal()
        print("Settings node")
        while True:
            if keyboard.read_key() == 'backspace':
                break

    # Shows all keybindings
    if keyboard.read_key() == 'h':
        clearTerminal()
        print("Welcome to the help node!\n"
              "Here you can find all the keybindings for the game.\n"
              "Evolution node: [bold]e[/]\n"
              "Biome node: [bold]b[/]\n"
              "Law node: [bold]l[/]\n"
              "War node: [bold]w[/]\n"
              "Settings node: [bold]s[/]\n"
              "Quit game: [bold]q[/]\n"
              "\n"
              "Press [bold]backspace[/] to exit any node.")
        while True:
            if keyboard.read_key() == 'backspace':
                break

    # Quits game
    if keyboard.read_key() == 'q':
        game_playing = False

