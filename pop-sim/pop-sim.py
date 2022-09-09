import random
import timeit

male_names = open("male-names.txt").readlines()
female_names = open("female-names.txt").readlines()

population = -1

def debugTimer(mode):
    _debug_start = 0
    _debug_end = 0
    if mode == "s":
        _debug_start = timeit.timeit()
    elif mode == "e":
        _debug_end = timeit.timeit()
        _time = _debug_end - _debug_start
        print(_time)


class Sim:
    people = []


    def __init__(self):
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

        # Gives person a starting affection rating
        self.temp_person.append(100)  # 100 is a test!!!

        self.love_lst = [None, None]
        # Gives person a lover
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

    # def passTime(self, amount_of_time):

    def updateSim(self, amount_of_time):
        if amount_of_time == "": amount_of_time = 1
        amount_of_time = int(amount_of_time)

        for self.time in range(amount_of_time):

            # Adds age to all people
            for self.p in self.people:
                self.p[2] += 1

            # Calculates who will reproduce
            for self.p in self.people:

                # Chooses lover unless person already has one
                if self.p[2] > 16*12 and not self.p[5][1]:
                    if not self.p[5][0]:
                        choice_of_lover = None
                        while choice_of_lover is None or choice_of_lover == self.p[0]:
                            choice_of_lover = random.choice(self.people)[0]
                            print(choice_of_lover)

                        if self.people[choice_of_lover][2] > 16*12:
                            self.p[5][0] = choice_of_lover

                    else:
                        for self.temp_person in self.people:
                            if self.p[5][0] == self.temp_person[0] and random.randint(0, 100) < 10:
                                self.temp_person[5][1] = True
                                self.p[5][1] = True

                for self.temp_person in self.people:
                    if self.p[5][0] == self.temp_person[0] and random.randint(0, 100) < 8:
                        # Creates a baby!!!
                        self.createPerson()


Sim.createPerson(Sim())
Sim.createPerson(Sim())
# Sim.people[0][5] = 1
while True:
    Sim.printPeople(Sim())
    time_amount = input(">>> ").strip()
    debugTimer("s")
    Sim.updateSim(Sim(), time_amount)
    debugTimer("e")
    print(Sim.people)
