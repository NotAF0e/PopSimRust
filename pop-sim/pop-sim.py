import random
import time
import matplotlib.pyplot as plt
from rich.progress import track


male_names = open("male-names.txt").readlines()
female_names = open("female-names.txt").readlines()

population = -1
months_passed = 0

def debugTimer(mode):
    _debug_start = 0
    _debug_end = 0
    if mode == "s":
        _debug_start = time.process_time()
    elif mode == "e":
        _debug_end = time.process_time()
        _time = (_debug_end - _debug_start) / 10
        print(_time)


class Sim:
    people = []
    dead_people = []


    def __init__(self):
        self.months_passed = 0
        self.love_lst = None
        self.p = None
        self.temp_person = None


    def createPerson(self):
        global population
        population += 1

        # These names are taken from 2 text files
        global male_names, female_names
        both_gender_names = [random.choice(male_names).replace("\n", ""),
                             random.choice(female_names).replace("\n", "")]

        self.temp_person = []

        # Gives person a gender
        # 0 is male, 1 is female
        if random.randint(0, 100) >= 50:
            gender = 0
        else:
            gender = 1
        self.temp_person.append(population)

        # Gives person a name
        self.temp_person.append(both_gender_names[gender])

        # Gives person a starting age of 0 months old
        # Time in pop-sim is in months throughout
        self.temp_person.append(0)

        self.temp_person.append(gender)

        # Gives person someone they will love, if they are in a relationship and affection
        self.love_lst = [None, None, 100]  # 100 is a test !!!
        self.temp_person.append(self.love_lst)

        # Appends the person to people
        self.people.append(self.temp_person)

    def printPeople(self):
        gender = ["Male", "Female"]

        for self.p in self.people:
            age = self.p[2]

            # Calculates years and months, out of months
            age_years, age_months = divmod(age, 12)
            print(f"Name: {self.p[1]}\n"
                  f"Age: {age_years} years, {age_months} months\n"
                  f"Gender: {gender[self.p[3]]}\n")

    def updateSim(self, amount_of_time):
        global months_passed
        debugTimer("s")

        if amount_of_time == "": amount_of_time = 1
        amount_of_time = int(amount_of_time)
        months_passed += amount_of_time

        for self.time in track(range(amount_of_time), "Total load\n"):
            # Adds age to all people or kills them
            for self.p in self.people:
                self.p[2] += 1
                if self.p[2] > 35*12:
                    temp_person = self.people.remove(self.p)
                    self.dead_people.append(temp_person)
                    # del self.people[self.p[0]]

            # Calculates who will reproduce
            for self.p in self.people:

                # Chooses lover unless person already has one
                if self.p[2] > 15*12 and not self.p[4][1]:
                    if not self.p[4][0]:
                        choices_of_lovers = []
                        # print(self.people)
                        for temp_lover in self.people:
                            if temp_lover != self.p[0]:
                                choices_of_lovers.append(temp_lover[0])
                                # print(choices_of_lovers)

                        choice_of_lover = None

                        # Chooses random lover from list of applicable
                        if choices_of_lovers:
                            for x in range(10):
                                choice_of_lover = random.choice(choices_of_lovers)

                        if choice_of_lover and choice_of_lover < len(self.people) and self.people[choice_of_lover][2] > 15*12:
                            self.p[4][0] = choice_of_lover

                    else:
                        # Sets lover
                        for self.temp_person in self.people:
                            if self.p[4][0] == self.temp_person[0] and random.randint(0, 100) < 10:
                                self.temp_person[4][1] = True
                                self.p[4][1] = True

                # Checks if baby should be born
                for self.temp_person in self.people:
                    if self.p[4][0] == self.temp_person[0] and self.p[4][1] and random.randint(0, 100) < 8:
                        # Creates a baby!!!
                        self.createPerson()



Sim.createPerson(Sim())
Sim.createPerson(Sim())
Sim.printPeople(Sim())

while True:
    time_amount = input(">>> ").strip()
    if time_amount == "q":
        mp = []
        pop = []
        p = 0
        for a in Sim.people:

            p += 1
            mp.append(a[2])
            pop.append(p)

        pop.reverse()
        print(mp, pop)
        fig, ax = plt.subplots()
        ax.plot(mp, pop)

        ax.set(xlabel='time (months)', ylabel='population',
               title='Pop-sim population/time graph')
        ax.grid()

        plt.show()
        time_amount = 0
    elif time_amount == "p":
        print(f"Population: {len(Sim.people)}\n")
        time_amount = 0

    Sim.updateSim(Sim(), time_amount)
    Sim.printPeople(Sim())
    debugTimer("e")
    # print(Sim.people)
