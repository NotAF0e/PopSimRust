from cython import *
import random
import time
import matplotlib.pyplot as plt

cdef list people
cdef list dead_people
people = []
dead_people = []


cdef list male_names = open("../male-names.txt").readlines()
cdef list female_names = open("../female-names.txt").readlines()

cdef long population = -1
cdef long months_passed = 0
cdef long event = -1

# Graphing variables
cdef list mp = []
cdef list pop = []
cdef long tp = -1

def debugTimer(mode):
    cdef float _debug_start = 0
    cdef float _debug_end = 0
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


cdef class Sim:
    def __init__(self):
        cdef long months_passed
        cdef list p
        cdef list temp_person
        months_passed = 0
        p = []
        temp_person = []

    cdef void createPerson(self, int gender_choice, long id_who_created=0):
        global population, people
        population += 1

        # These names are taken from 2 text files
        global male_names, female_names
        both_gender_names = [random.choice(male_names).replace("\n", ""),
                            random.choice(female_names).replace("\n", "")]
        temp_person = []

        # Gives person a gender
        # 0 is male, 1 is female
        gender = random.randint(0, 1)
        if gender_choice != -1:
            gender = gender_choice

        temp_person.append(population)

        # Gives person a name
        temp_person.append(both_gender_names[gender])

        # Gives person a starting age of 0 months old
        # Time in pop-sim is in months throughout
        temp_person.append(0)

        temp_person.append(gender)

        # Gives person someone they will love, if they are in a relationship and affection
        love_lst = [None, None, 100]  # 100 is a test !!!
        temp_person.append(love_lst)


        if id_who_created == 0:
            parents_and_children_lst = [[id_who_created, id_who_created], [0]]
        else:
            parents_and_children_lst = [[id_who_created, people[id_who_created][4]], [0]]

        # print(parents_and_children_lst)
        temp_person.append(0)

        # Appends the person to people
        people.append(temp_person)

    cdef void printPeople(self):
        gender = ["Male", "Female"]

        for p in people:
            age = p[2]

            # Calculates years and months, out of months
            age_years, age_months = divmod(age, 12)
            print(f"Name: {p[1]}\n"
                  f"Age: {age_years} years, {age_months} months\n"
                  f"Gender: {gender[p[3]]}\n")

    cdef void kill(self, long person):
        if len(people) == 1:
            graph()
            exit("The population has ceased")
        
        temp_person = people.index(people[person])
        # print(temp_person)
        dead_people.append(temp_person)
        # del people[temp_person]

    
    cdef void updateSim(self, long amount_of_time) except *:
        global population, months_passed, pop, mp, tp, event

        months_passed += amount_of_time

        cdef list choices_of_lovers
        cdef list temp_lover
        cdef long ptk
        cdef list p

        for time in range(amount_of_time):
            # Bellow is for graph and storing population at a certain time
            pop.append(len(people))
            tp += 1
            mp.append(tp)

            # ages_of_death = [2, 10, 20, 35, 50, 70, 80, 90]

            # Adds age to all people or kills them
            for p in people:
                ptk = p[0]
                p[2] += 1
                # random.choices(ages_of_death, [0.5, 0.005, 0.05, 1, 15, 40, 50, 20],
                #                k=10)[random.randint(0, 9)]
                if p[2] > 35 * 12:
                    # print(p[0])
                    self.kill(p[0])


            # Calculates who will reproduce
            for p in people:
                
                # Chooses lover unless person already has one
                if p[2] > 15 * 12 and not p[4][1]:
                    if not p[4][0]:
                        choices_of_lovers = []
                        # print(people)
                        for temp_lover in people:
                            if temp_lover != p[0] and temp_lover[3] != p[3]:
                                choices_of_lovers.append(temp_lover[0])
                                # print(choices_of_lovers)

                        choice_of_lover = None

                        # Chooses random lover from list of applicable
                        if choices_of_lovers:
                            for x in range(10):
                                choice_of_lover = random.choice(choices_of_lovers)

                        if choice_of_lover and choice_of_lover < len(people) \
                                and people[choice_of_lover][2] > 15 * 12:
                            p[4][0] = choice_of_lover

                    else:
                        # Sets lover
                        for temp_person in people:
                            if p[4][0] == temp_person[0] and random.randint(0, 100) < 10:
                                temp_person[4][1] = True
                                p[4][1] = True

                # Checks if baby should be born
                for temp_person in people:
                    if p[4][0] == temp_person[0] and p[4][1] \
                            and random.randint(0, 100) < 8:

                        # Creates a baby!!!
                        self.createPerson(-1, p[0])

            self.disaster()

    cdef void disaster(self):
        global event
        ml_for_disaster = -1

        if event == 1 and ml_for_disaster == -1:
            ml_for_disaster = random.randint(2, 25)


        if event == 1 and ml_for_disaster != 0:
            kill_multiple = random.randint(1, 40)

            for kills in range(kill_multiple):
                self.kill(random.choice(people))
            ml_for_disaster -= 1
        else:
            event = -1

class Cmds:
    pass


# "Adam and Eve" are created
Sim.createPerson(Sim(), 0, 0)
Sim.createPerson(Sim(), 1, 0)
Sim.printPeople(Sim())

cdef int print_every_person = 0  # For higher performance

cdef int time_amount

while True:
    time_amount = int(input(">>> ").strip())

    if time_amount == "q":
        graph()
        time_amount = 0
    elif time_amount == "p" or print_every_person is 1:
        print(f"Population: {len(people)}\n")
        time_amount = 0

    # elif time_amount == "d":
    #    print("disaster")
    #    event = 1
    #    time_amount = 0

    debugTimer("s")
    Sim.updateSim(Sim(), time_amount)

    if print_every_person == 0: Sim.printPeople(Sim())
    # print(Sim.people)
    print(dead_people)
    debugTimer("e")
