# **Format of patch_list.md:**

- type [ Date ] Feature/Fix info

## **Types:**

- **B**: Bug fix
- **F**: Feature addition
- **O**: Other

---
**These are changes regarding: *pop_sim_rust_gui*!**

## **Actual file:**

- **O** [07/11/23] Added *patch_list.md* to *pop_sim_rust_gui*
- **B** [07/11/23] Person table will no longer show will a population of 0
- **F** [07/11/23] Added spacers into the person table
- **F** [07/11/23] Fixed a bug with adding values to graph before removing dead people
- **O** [07/11/23] Added *male_names.txt* and *female_names.txt* to *pop_sim_rust_gui*
- **F** [07/11/23] People are now given a random name from either *male_names.txt* or *female_names.txt* on birth dependent on their sex
- **O** [10/11/23] Formatted all of *pop_sim_rust_gui/main.rs*
- **O** [17/01/23] Added todo to *pop_sim_rust_gui/main.rs*
- **B** [18/01/23] Fixed people not spawning in set pairs
- **O** [18/01/23] Changed code structure for easier programming
- **F** [18/01/23] Added temporary close table button as it was causing performance issues
- **F** [18/01/23] Table v2 is out! Table looks nicer, and info is easier to read
- **F** [18/01/23] Added a frame time counter to show simulation performance
- **B** [20/01/23] Fixed a strange table id problem
- **F** [21/01/23] Optimised the app a ton. It is now up to 1500x faster.
- **F** [21/01/23] Improved the frame time counter: it is now on the bottom bar. Also adjust the bottom bar text size to accommodate the frame counter

- **F** [25/01/23] Began the immigration system
- **B** [25/01/23] Fixed id issues
- **F** [27/01/23] Finished the immigration system
- **F** [28/01/23] Added an icon
- **O** [01/02/23] Streamiled the code a bit
- **F** [01/02/23] Added simulation end window

