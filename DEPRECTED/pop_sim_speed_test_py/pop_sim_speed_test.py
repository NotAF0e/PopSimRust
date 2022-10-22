from line_profiler import *
import random
import time
import matplotlib.pyplot as plt
from rich.progress import track
from rich.console import Console


c = Console()

male_names = open("../male_names.txt").readlines()
female_names = open("../female_names.txt").readlines()
population = -1
months_passed = 0
event = None

# Graphing variables
mp = []
pop = []
tp = -1

def debugTimer(mode):
    _debug_start = 0
    _debug_end = 0
    if mode == "s":
        _debug_start = time.process_time()
    elif mode == "e":
        _debug_end = time.process_time()
        _time = (_debug_end - _debug_start) / 10
        print(_time)

def graph():
    global mp, pop

    print(mp, pop)
    fig, ax = plt.subplots()
    ax.plot(mp, pop)
    ax.set(xlabel='time (months)', ylabel='population',
           title='Pop-sim population/time graph')
    ax.grid()

    plt.show()


class Sim:
    people = []
    dead_people = []
    
    @profile
    def __init__(self):
        self.months_passed = 0
        self.p = None
        self.temp_person = None
        
    @profile
    def createPerson(self, gender_choice=-1, id_who_created=0):
        global population
        population += 1

        # These names are taken from 2 text files
        global male_names, female_names
        both_gender_names = [random.choice(male_names).replace("\n", ""),
                             random.choice(female_names).replace("\n", "")]

        self.temp_person = []

        # Gives person a gender
        # 0 is male, 1 is female
        gender = random.randint(0, 1)
        if gender_choice != -1:
            gender = gender_choice

        self.temp_person.append(population)

        # Gives person a name
        self.temp_person.append(both_gender_names[gender])

        # Gives person a starting age of 0 months old
        # Time in pop-sim is in months throughout
        self.temp_person.append(0)

        self.temp_person.append(gender)

        # Gives person someone they will love, if they are in a relationship and affection
        love_lst = [None, None, 100]  # 100 is a test !!!
        self.temp_person.append(love_lst)


        if id_who_created == 0:
            parents_and_children_lst = [[id_who_created, id_who_created], [0]]
        else:
            parents_and_children_lst = [[id_who_created - 1, self.people[id_who_created -1][4]], [0]]

        # print(parents_and_children_lst)
        self.temp_person.append(0)

        # Appends the person to people
        self.people.append(self.temp_person)
        
    @profile
    def printPeople(self):
        gender = ["Male", "Female"]

        for self.p in self.people:
            age = self.p[2]

            # Calculates years and months, out of months
            age_years, age_months = divmod(age, 12)
            print(f"Name: {self.p[1]}\n"
                  f"Age: {age_years} years, {age_months} months\n"
                  f"Gender: {gender[self.p[3]]}\n")
            
    @profile
    def kill(self, person):
        if len(self.people) == 1:
            graph()
            exit("The population has ceased")
        temp_person = self.people.remove(person)
        self.dead_people.append(temp_person)
        
    @profile
    def updateSim(self, amount_of_time):
        global population, months_passed, pop, mp, tp, event

        if amount_of_time == "": amount_of_time = 1
        amount_of_time = int(amount_of_time)
        months_passed += amount_of_time

        for self.time in track(range(amount_of_time), "Total load\n"):
            # Bellow is for graph and storing population at a certain time
            pop.append(len(self.people))
            tp += 1
            mp.append(tp)

            # ages_of_death = [2, 10, 20, 35, 50, 70, 80, 90]

            # Adds age to all people or kills them
            for self.p in self.people:
                self.p[2] += 1
                # random.choices(ages_of_death, [0.5, 0.005, 0.05, 1, 15, 40, 50, 20],
                #                k=10)[random.randint(0, 9)]
                if self.p[2] > 35 * 12:
                    self.kill(self.p)

                    # del self.people[self.p[0]]

            # Calculates who will reproduce
            for self.p in self.people:

                # Chooses lover unless person already has one
                if self.p[2] > 12 * 12 and not self.p[4][1]:
                    if not self.p[4][0]:
                        choices_of_lovers = []
                        # print(self.people)
                        for temp_lover in self.people:
                            if temp_lover != self.p[0] and temp_lover[3] != self.p[3]:
                                choices_of_lovers.append(temp_lover[0])
                                # print(choices_of_lovers)

                        choice_of_lover = None

                        # Chooses random lover from list of applicable
                        if choices_of_lovers:
                            for x in range(10):
                                choice_of_lover = random.choice(choices_of_lovers)

                        if choice_of_lover and choice_of_lover < len(self.people) \
                                and self.people[choice_of_lover][2] > 12 * 12:
                            self.p[4][0] = choice_of_lover

                    else:
                        # Sets lover
                        for self.temp_person in self.people:
                            if self.p[4][0] == self.temp_person[0]:  # and random.randint(0, 100) < 10:
                                self.temp_person[4][1] = True
                                self.p[4][1] = True

                # Checks if baby should be born
                for self.temp_person in self.people:
                    if self.p[4][0] == self.temp_person[0] and self.p[4][1] \
                            and random.randint(0, 100) < 8:
                        # Creates a baby!!!
                        self.createPerson(id_who_created=self.p[0])

            self.disaster()
            
    @profile
    def disaster(self):
        global event
        ml_for_disaster = -1

        if event == 1 and ml_for_disaster == -1:
            ml_for_disaster = random.randint(2, 25)


        if event == 1 and ml_for_disaster != 0:
            kill_multiple = random.randint(1, 40)

            for kills in range(kill_multiple):
                self.kill(random.choice(self.people))
            ml_for_disaster -= 1
        else:
            event = None

class Cmds:
    pass


# "Adam and Eve" are created
Sim.createPerson(Sim(), 0)
Sim.createPerson(Sim(), 1)
Sim.printPeople(Sim())

print_every_person = True  # For higher performance

@profile
def main():
    time_amount = 500
    if time_amount == "g":
        graph()
        time_amount = 0
    elif time_amount == "p" or print_every_person is False:
        print(f"Population: {len(Sim.people)}\n")
        time_amount = 0

    elif time_amount == "d":
        print("disaster")
        event = 1
        time_amount = 0
    elif time_amount == "q":
        exit()

    debugTimer("s")
    Sim.updateSim(Sim(), time_amount)
    debugTimer("e")

    if print_every_person: Sim.printPeople(Sim())
    c.print()
    c.print(Sim.dead_people)
    exit()
main()

