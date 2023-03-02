# **Format of patch_list.md:**

- type [ Date ] Feature/Fix info

## **Types:**

- **B**: Bug fix
- **F**: Feature addition
- **O**: Other

---
**These are changes regarding: *pop_sim_gui*!**

## **Actual file:**

- **O** [07/11/23] Added *patch_list.md* to *pop_sim_gui*
- **B** [07/11/23] Person table will no longer show will a population of 0
- **F** [07/11/23] Added spacers into the person table
- **F** [07/11/23] Fixed a bug with adding values to graph before removing dead people
- **O** [07/11/23] Added *male_names.txt* and *female_names.txt* to *pop_sim_gui*
- **F** [07/11/23] People are now given a random name from either *male_names.txt* or *female_names.txt* on birth dependent on their sex
- **O** [10/11/23] Formatted all of *pop_sim_gui/main.rs*
- **O** [17/01/23] Added todo to *pop_sim_gui/main.rs*
- **B** [18/01/23] Fixed people not spawning in set pairs
- **O** [18/01/23] Changed code structure for easier programming
- **F** [18/01/23] Added temporary close table button as it was causing performance issues
- **F** [18/01/23] Table v2 is out! Table looks nicer, and info is easier to read
- **F** [18/01/23] Added a frame time counter to show simulation performance
- **B** [20/01/23] Fixed a strange table id problem
- **F** [21/01/23] Optimised the app a ton. It is now up to 1500x faster.
- **F** [21/01/23] Improved the frame time counter: it is now on the bottom bar. Also adjust the bottom bar text size to accommodate the frame counter

- **F** [25/01/23] Began adding the immigration system
- **B** [25/01/23] Fixed id issues
- **F** [27/01/23] Finished the immigration system
- **F** [28/01/23] Added an icon
- **O** [01/02/23] Streamlined the code a bit
- **F** [01/02/23] Added simulation end window
- **F** [07/02/23] Began adding the random death causes system
- **O** [07/02/23] Removed imigration system as it was causing many issues
- **B** [07/02/23] Finally fixed a lover check and remove bug which has been plaguing the project for a month

- **F** [19/02/23] Added a button which disables the lover check fix as it is incredibly badly optimised
- **O** [19/02/23] Updated some project dependencies in *Cargo.toml*
- **F** [20/02/23] Implemented a better way to do the lover fixes - Now each 100 months
- **F** [20/02/23] Added better buttons which are easier to code with and show 2 different states
- **F** [23/02/23] Made progress on the new simulation start screen
- **F** [23/02/23] Improved the frame time calculator and added the average frame time to the end screen
- **O** [23/02/23] Changed some code structure which improved performance of the simulation a bit
- **O** [23/02/23] Began adding the code structure for more sim stats
- **O** [25/02/23] Changed code structure further: the code is now split into 2 files
- **O** [25/02/23] Deprecated *pop_sim_gui_threaded*
- **O** [27/02/23] Even more code structure changes
- **F** [28/02/23] Began adding the structure neede for the epidemic system
- **F** [28/02/23] Continued with the epidemic system with: epidemic initialization and epidemic ending
- **F** [01/03/23] Fixed the readme a bit
- **B** [02/03/23] Fixed many issues which occured with the epidemic system
- **F** [02/03/23] An epidemic can now be started and will automatically stop when nobody is infected
