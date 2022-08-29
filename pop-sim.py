import random

male_names = open("male-names.txt").readlines()
female_names = open("female-names.txt").readlines()


def giveName():
    # Returns 2 names: a male and female
    # These names are taken from 2 text files
    global male_names, female_names
    both_gender_names = [random.choice(male_names), random.choice(female_names)]
    return both_gender_names


class Sim:
    people = []

    def __init__(self):
        self.temp_person = None

    def createPerson(self, name_given):
        self.temp_person = []
        # Gives person a gender
        # 0 is male, 1 is female
        if random.randint(0, 100) >= 50:
            gender = 0
        else:
            gender = 1

        # Gives person a name
        self.temp_person.append(name_given[gender])

        # Gives person a starting age of 0 months old
        # Time in pop-sim is in months throughout
        self.temp_person.append(0)

        self.temp_person.append(gender)

        # Appends the person to people
        self.people.append(self.temp_person)


    def printPeople(self):
        print(self.people)


Sim.createPerson(Sim(), giveName())
Sim.createPerson(Sim(), giveName())
Sim.printPeople(Sim())
