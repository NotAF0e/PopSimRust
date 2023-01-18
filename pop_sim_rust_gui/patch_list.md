# **Format of patch_list.md:**

- type [ Date ] Feature/Fix info

## **Types:**

- **B**: Bug fix
- **F**: Feature addition
- **O**: Other

---

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
- **F** [18/01/23] Added a time between frames counter to show simulation performance
