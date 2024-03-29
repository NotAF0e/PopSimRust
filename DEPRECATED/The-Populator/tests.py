# This is a test version of the game to add larger experimental features
# These features could break the game when added incorrectly, so I will code them in here instead
import random
import time
# import json
from rich.console import Console
from rich.progress import Progress

c = Console()

# Declaration of variables ------------------------------------------------------------------------
tax_percentage = 1001  # (0.1%)

food = 0

happiness = 2  # 0 --> 4 (0 = hated, 1 = disliked 2 = neutral, 3 = liked, 4 = loved)
happiness_emoji = ["😡", "😠", "😐", "🙂", "😍"]


evolution_rate = 10

weeks_passed = 0
months = ["January", "February", "March", "April", "May", "June", "July", "August", "September",
          "October", "November", "December"]

BREAK = False
tmp0 = None
tmp1 = None
tmp2 = None
tmp3 = None


class Biome:
    pass


tempBiome = []


def clearTerminal():
    c.clear()


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


def intInput(s_str):
    while True:
        try:
            val = int(input(s_str))
            break
        except ValueError:
            print(f"Please enter an int!")
    return val


# Pop functions ----------------------------------------------------------------------------

class Pop:
    pop_name = [""]
    population = [0]
    born = [0]
    dead = [0]
    money = [0]
    player_born_temp = 0
    player_dead_temp = 0
    death_rate_a = 0
    death_rate_b = 2

    def create(self, population_name, base_money, develop_time, time_multiplier):
        clearTerminal()

        print("Creating population...")
        c.print(f"Starting money amount: {base_money}\n"
                f"Amount of weeks for population to develop: {develop_time}\n\n")

        self.pop_name[0] = population_name
        self.money[0] = base_money

        # Pop calculation ----------------------------------------------------------------------
        start_process = time.process_time()  # Start time calculation
        time_step_interval = (develop_time / 100).__round__()

        # Using the rich library to create progress bar bellow
        with Progress() as progress:
            task = progress.add_task("", total=100)

            if develop_time < 151:
                for i in range(develop_time):
                    self.doXStepsInTime(1, False)
                    progress.update(task, advance=100 / develop_time)
            else:
                while not progress.finished:
                    self.doXStepsInTime(1, False)
                    if time_step_interval == 0:
                        time_step_interval = (develop_time / 100).__round__()
                        # Bellow updates the progress percentage
                        progress.update(task, advance=1)
                    develop_time -= 1
                    time_step_interval -= 1

            self.born[0] += percentageIncrease(self.born[0], 5).__round__() * time_multiplier  # Increases born by 5%
            self.dead[0] += percentageIncrease(self.dead[0], 1).__round__() * time_multiplier  # Increases dead by 1%
        c.print(f"\n[bold]Population: [bold]{self.population[0]}[/]\n"
                f"People born: [bold]{self.born[0]}[/]\n"
                f"People dead: [bold]{self.dead[0]}[/]")

        end_process = time.process_time()  # End time calculation

        # Time calculation ----------------------------------------------------------------------------
        time_of_process = end_process - start_process
        if time_of_process == 0:
            print("\nCompleted very quickly...")
        else:
            print(f"\nCompleted in {time_of_process} seconds...")

    def doXStepsInTime(self, x, calc_weeks=True):
        global weeks_passed

        temp0 = 1
        if calc_weeks:
            temp0 = 0
        while x != 0:
            self.player_born_temp = random.randint(1, 4)
            self.player_dead_temp = random.randint(self.death_rate_a, self.death_rate_b)
            self.born[0] += self.player_born_temp  # Born
            self.dead[0] += self.player_dead_temp  # Dead
            weeks_passed += temp0
            x -= 1
        self.population[0] = self.born[0] - self.dead[0]  # Pop


# World functions ---------------------------------------------------------------------------------



def printBiomeDetails(biome_info_lst, detailed_info=False):
    # Biomes 0-7 are normal. Biomes 8-10 are dangerous --------------------------------------------
    Biome.biomes = ["[#00bf2d]grassland", "[#998642]savanna", "[#d1cdc2]taiga", "[green]forest",
                    "[#f7f372]beach", "[#7691e8]mountains", "[green]hills", "[#b8ab1d]desert",
                    "[#ff4d00]lava lake", "[#6b8a89]treacherous cliffs", "[bold white]icy arctic"]

    Biome.temperatures = ["[#5468ff]-25", "[#8a96f2]-10", "[#bfc7ff]10", "[#ffd0bf]20",
                          "[#ff6c47]25", "[#ff3b21]35", "[#ff3636]40"]

    formatted_biome = Biome.biomes[biome_info_lst[1]]
    if detailed_info:
        if biome_info_lst[4] is True:
            c.print("[red]Very dangerous!")
        c.print("Biome:", formatted_biome,
                f"\nAverage temperature: {Biome.temperatures[biome_info_lst[2]]}" + "°C" + "[/]",
                f"\nAltitude: [bold]{biome_info_lst[3]}m[/]")
    else:
        c.print(f"[white]{biome_info_lst[0]}.[/] {formatted_biome}")


def createBiome(biome_rand, biome_num):
    tempBiome.clear()
    tempBiome.append(biome_num)
    # Adds name to Biome.info[0]
    tempBiome.append(biome_rand)

    # Temperature calculation
    Biome.average_temperatures = [3, 4, 3, 3, 2, 1, 2, 5, 6, 1, 0]
    tempBiome.append(Biome.average_temperatures[biome_rand])

    # Elevation calculation
    Biome.low_elevations = [610, 100, 100, 900, 0, 1500, 30, 150, 0, 2000, 0]
    Biome.high_elevations = [1220, 500, 300, 1500, 5, 8850, 150, 2600, 500, 7000, 500]
    tempBiome.append(random.randint(Biome.low_elevations[biome_rand], Biome.high_elevations[biome_rand]))

    # Check if biome is dangerous
    Biome.dangerous_biomes = [8, 9, 10]
    if biome_rand in Biome.dangerous_biomes:
        tempBiome.append(True)
    else:
        tempBiome.append(False)

    returner = tempBiome
    return returner


def printAsciiWorld():
    c.print(*World.ascii_world, sep='\n')


class World:
    world_name = ""
    biomes = []
    ascii_world = []
    start_biome = 0

    def create(self, world_name, biome_amount):
        self.world_name = world_name
        biome_num = 0
        while biome_amount != 0:
            returner = createBiome(random.randint(0, 10), biome_num)
            self.biomes.append(returner[:])
            # Debug lines
            # print(returner)
            # print(self.biomes)
            biome_num += 1
            biome_amount -= 1
        # print(self.biomes)

    # Bellow function is modified code from: https://youtu.be/YS-5ezQPWuU Thanks Dennis
    def createAsciiWorld(self, width=145, height=50, land_amount=2500):
        drunk = {
            'landAmount': land_amount,
            'padding': 2,
            'x': int(width / 2),
            'y': int(height / 2)
        }

        def getLevelRow():
            return ['[blue]#'] * width

        level = [getLevelRow() for _ in range(height)]

        while drunk['landAmount'] >= 0:
            x = drunk['x']
            y = drunk['y']

            if level[y][x] == '[blue]#':
                level[y][x] = '[green]0'
                drunk['landAmount'] -= 1

            roll = random.randint(1, 4)

            if roll == 1 and x > drunk['padding']:
                drunk['x'] -= 1

            if roll == 2 and x < width - 1 - drunk['padding']:
                drunk['x'] += 1

            if roll == 3 and y > drunk['padding']:
                drunk['y'] -= 1

            if roll == 4 and y < height - 1 - drunk['padding']:
                drunk['y'] += 1

        for row in level:
            self.ascii_world.append(''.join(row))

        return self.ascii_world

    def print(self, detailed_info=False, display_current_biome=False):
        c.print(f"World name: [bold]{self.world_name}[/]\n")
        x = 0
        for biome in self.biomes:
            if display_current_biome and self.biomes[x][0] == current_biome:
                c.print("[bold](Current location)[/]")
            printBiomeDetails(biome, detailed_info=detailed_info)
            if detailed_info: print("\n")
            x += 1


# User input to start game ------------------------------------------------------------------------
# World creation
clearTerminal()
print("Before you create your population you will need to create a world.\n")
while tmp0 is None:
    tmp0 = input("Enter the name of your world: ").strip()
World.world_name = tmp0

tmp1 = True
while True:
    clearTerminal()
    if tmp1 is True:
        World.create(World(), "Name", 5)
        World.createAsciiWorld(World())
        clearTerminal()
    World.print(World())
    c.print("\nEnter [green]y[/] to create this world, or [red]n[/] to generate another.")
    c.print("You can press [bold]enter[/] to show world map [red](work in progress...)[/]")
    tmp0 = input(">>>").strip().lower()
    if tmp0 == "y":
        clearTerminal()
        break
    elif tmp0 == "n":
        tmp1 = True
        clearTerminal()
        World.biomes.clear()  # Resets biomes in ascii_world
        World.ascii_world.clear()
        pass
    else:
        tmp1 = False
        clearTerminal()
        c.print(*World.ascii_world, sep='\n')
        input("Press b to exit world preview...")

while True:
    clearTerminal()
    World.print(World())
    c.print("\nWhat will be your starting biome? [red]You can not change this later![/]")
    start_biome = intInput(">>>")
    clearTerminal()
    printBiomeDetails(World.biomes[start_biome:][0], detailed_info=True)  # Starting biome info
    c.print("\nAre you sure want to start [white]here([green]y[/], [red]n[/])?..")
    tmp0 = input(">>>").strip().lower()
    if tmp0 == "y":
        break
    elif tmp0 == "n":
        clearTerminal()
        pass



# Pop creation
while not BREAK:
    clearTerminal()
    tmp0 = str(input("Enter the name of your population: ").strip())
    tmp1 = intInput("Enter your starting money: ")
    tmp2 = intInput("Enter the amount of weeks for your population to grow: ")
    weeks_passed = tmp2

    c.print("\nAre you sure you want to create a population with the "
            "following [white]stats([green]y[/], [red]n[/])?.."
            f"\nName: [bold]{tmp0}[/]\n"
            f"Starting money amount: {tmp1}\n"
            f"Amount of weeks for population to develop: {tmp2}")
    if tmp2 > 100000000:
        c.print(f"[red]Caution! This many weeks may take a long time to complete!")
    print("\n")
    BREAK = False
    while not BREAK:
        tmp3 = input(">>>").strip().lower()
        if tmp3 == "y":
            BREAK = True
            break

        elif tmp3 == "n":
            break

clearTerminal()
Pop.create(Pop(), tmp0, tmp1, tmp2, 15)
input("\nPress enter to continue...")

# Variables setup
game_playing = True
current_biome = start_biome
if World.biomes[current_biome][4]:
    death_rate_b = 5

# Main game loop ----------------------------------------------------------------------------------
while game_playing:
    # Main node(Shows most info in one place)
    clearTerminal()
    printLogo()
    c.print(f"\n[bold]{Pop.pop_name[0]}[/]\n")
    # Prints amount of time passed
    if weeks_passed > 12:
        print(f"Year: {weeks_passed // 12}\n"
              f"Month: {months[weeks_passed % 12]}\n")
    else:
        print(f"Week: {weeks_passed}\n"
              f"Month: {months[weeks_passed % 12]}\n")

    # Prints population info
    c.print(f"Population: {Pop.population[0]}\n"
            f"People born: {Pop.born[0]}\n"
            f"People dead: {Pop.dead[0]}\n"
            f"Money: [green]${Pop.money[0]}[/]\n")

    # Prints population happiness
    print(f"\nHappiness: {happiness_emoji[happiness]}\n")
    tmp0 = input(">>>").strip().lower()
    # End of main node
    if tmp0 == 'e':
        clearTerminal()
        print("Evolution node")
        c.print("Evolve: [bold]enter[/]\n"
                "Change evolution rate: [bold]r[/]\n")

        tmp0 = False
        tmp1 = False
        tmp2 = input(">>>").strip().lower()
        while True:
            if tmp2 == "":
                tmp0 = True
                break

            if tmp2 == "r":
                tmp1 = True
                break

        while tmp0:
            tmp3 = input(">>>").strip().lower()
            if tmp3 == "":
                clearTerminal()
                Pop.doXStepsInTime(Pop(), evolution_rate)
                c.print(f"[bold]Population: [bold]{Pop.population[0]}[/]\n"
                        f"People born: [bold]{Pop.born[0]}[/]\n"
                        f"People dead: [bold]{Pop.dead[0]}[/]")

                c.print(f"\n[green]+{Pop.player_born_temp} born[/]\n"
                        f"[red]-{Pop.player_dead_temp} dead[/]\n")

                # Calculates money from taxes
                Pop.money[0] += (Pop.population[0] // tax_percentage)
                c.print(f"[bold]Money: [green]${Pop.money[0]}[/]\n")
            elif tmp3 == 'b':
                break

        if tmp1:
            clearTerminal()
            c.print(f"The evolution rate is currently: [bold]{evolution_rate}[/]")
            time.sleep(1)
            while True:
                try:
                    clearTerminal()
                    evolution_rate = intInput("Change evolution rate: ")
                    break
                except ValueError:
                    pass  # oof
            clearTerminal()
            if evolution_rate < 10000:
                c.print(f"[red]Evolution rate higher than 10000 will cause the game to lag![/]")

            c.print(f"The evolution rate is now: [bold]{evolution_rate}[/]")
            c.print("Press [bold]enter[/] to continue...")
            input(">>>")

    if tmp0 == 'w':
        clearTerminal()
        World.print(World(), detailed_info=True, display_current_biome=True)
        c.print("\nYou can press [bold]enter[/] to show world map [red](work in progress...)[/]")
        while True:
            tmp1 = input(">>>").strip().lower()
            if tmp1 == 'b':
                break
            elif tmp1 == '':
                printAsciiWorld()

    if tmp0 == 'c':
        clearTerminal()
        c.print("This is the control node\n"
                "Change laws: [bold]j[/]\n"
                "Control taxes: [bold]t[/]\n")

        while True:
            tmp1 = input(">>>").strip().lower()
            if tmp1 == 'j':
                clearTerminal()
                print("Laws node")

            if tmp1 == 't':
                clearTerminal()
                c.print(f"Tax rate is currently: [bold]{tax_percentage - 1000}%[/]")
                tmp0 = intInput("New tax rate: ") + 1000
                tax_percentage = tmp0
                c.print(f"Tax rate is now: [bold]{tax_percentage - 1000}%[/]"
                        "Enter [bold]b[/] to continue...")

            elif tmp1 == 'b':
                break

    if tmp0 == 'f':
        clearTerminal()
        print("War node")
        while True:
            tmp1 = input(">>>").strip().lower()
            if tmp1 == 'b':
                break

    if tmp0 == 's':
        clearTerminal()
        print("Settings node")
        while True:
            tmp1 = input(">>>").strip().lower()
            if tmp1 == 'b':
                break

    # Shows all keybindings
    if tmp0 == 'h':
        clearTerminal()
        c.print("Welcome to the help node!\n"
                "Here you can find all the commands for the game.\n"
                "Evolution node: [bold]e[/]\n"
                "World node: [bold]w[/]\n"
                "Control node: [bold]c[/]\n"
                "War node: [bold]f[/]\n"
                "Settings node: [bold]s[/]\n"
                "Quit game: [bold]q[/]\n"
                "\n"
                "Enter [bold]b[/] to exit any node.")
        while True:
            tmp1 = input(">>>").strip().lower()
            if tmp1 == 'b':
                break

    # Quits game
    if tmp0 == 'q':
        game_playing = False
