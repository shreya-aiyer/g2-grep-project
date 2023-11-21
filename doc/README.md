# PoPL Project G2

Problem Statement
-----------------
grep (global regular expression print) is a standard command-line utility and a powerful tool for searching and manipulating text within files, and is typically available on Unix-like operating systems.

Our goal is to implement a stripped down version of grep in C and Rust preserving its core functionality, and to perform a comparative study between Rust and C with respect to Performance, and Ease of Use.

Performance - Comparison of the time taken to process files of increasing size, and plotting the same. Also comparing the memory usage of both implementations.

Ease of Use - Size of code in both implementations, and qualitative understanding of the experience writing in both languages.

An analysis of whether the functional or imperative programming approach to a CLI tool like grep would be an overall outcome of the project - this analysis would be on the basis of the theoretical and quantitative understanding of the two paradigms, and how they apply to grep’s use case.

Though there are existing implementations of grep using Rust, this comparative study incorporating memory utilisation has not been done before. This distinction in method of analysis sets our project apart from what we have currently seen existing. 


Software Architecture
---------------------
There are multiple software architectures being implemented. There is a CLI (Command Line Interface) that is being used for the input and output of the program.
Another major architecture being used is file handling (file open, read, etc.). The grep method searches for a particular word in a file. This requires a .txt file to be loaded into memory and opened. 
This implementation was relatively straightforward, so we wrote the code ourselves (for C and Rust). 
The testing component was locally present, and primarily used the inbuilt system monitor to track the use of memory and time of execution of the program. 

The code was run on a MacBook Air running macOS Sonoma on the Apple M1 chipset. To pull statistics on memory and CPU utilization, we used the Activity Monitor, which comes as a default in macOS. This application gives live data about all running processes on the system.

![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/8b0bf0c1-7263-4cab-bae1-bd82911fe329)


POPL Aspects
------------
- #### Imperitive vs Functional (Declarative programming language)
----------------------------------------------------------
- Imperitive programming languages like C describes a sequence of steps (algorithms) followed to reach a particular result. The focus here is on the 'How?'.
Functional programming languages like Rust are more focused on 'What?' is to be obtained rather than how it is obtained.

- Declarative programming really shines when the user is only concerned with the “what” and not the “how”. This makes sense in cases like when designing an API layer on top of a more complex framework.

- The reality is that somewhere underneath any declarative system there will be imperative programming driving it.

- #### Ease of Use
------------------
- There is a certain level of abstraction that is associated with functional programming. This is evident in the usage of functions like **map()(line 28 of grep.rs)** and **filter()(line 19 of grep.rs)** in this code. 

- For someone new to the language, such functions lack clarity and lead to additional cognitive load. However once the programmer gets used to the language, these fuctionalities prove to be highly efficient and more concise than an imperitive language like C.
- Coding with the memory management restrictions in Rust can be very challenging when familiar with an imperitive language as the code frequently throws errors in the compile stage. Overcoming this is a major challenge for new users as well. Variables getting dropped when the owner goes out of scopes is also a fresh concept that needs a good deal of time to adapt to.

- #### Memory Management
-------------------------
- The advantage of utilising Rust here is the additional memory safety. Rust employs a unique ownership system where each value in Rust has a variable that is its "owner." There are strict rules governing ownership, borrowing, and references that prevent common issues like null pointer dereferencing, dangling pointers, and memory leaks.
- These rules make the language much harder to use as getting to the run stage requires removing all these issues with owndership and the various memory management tactics at the compile stage.
- Ownership utilisation example -
  - **Grep Args struct in constructors.rs** - The GrepArgs struct contains fields of type String. In Rust, String is a growable, heap-allocated data structure, and owns the memory it points to. In the GrepArgs struct, search and file_path are owned String fields implying that when a GrepArgs instance is created, it will own the memory used to store the String data.
  - **main.rs** - The args vector fetched from the command line (args()) is collected into a Vec<String>. Ownership of the Vec<String> is transferred to the args variable within the main function, and then transferred to the run_grep function when it is passed as an argument.
  - Once any of these go out of scope, the memory allocated to them is dropped.
- Temporary allocation for the modified strings (to_lowercase(), format!()) and the vector created by collect::<Vec<String>>(), are handled by Rust's ownership and borrowing rules, ensuring they are freed when they are no longer needed. Thus for every line of input, additional double memory is not retained when converting to lowercase and storing the duplicate string .
- In C, it is easy for the programmer's mistakes like not freeing allocated memory to cause differences in the memory utilisation.(refer **lines 78,79 grep_impli.c** and the below analysis of the removal of the manual deallocation)

Instructions for compilation and running
-----------------------------------------
Cargo:

cargo build 

cargo run < word to search for> <file in which to search > [ any character for case sensitive/not sensitive ]

C:

gcc grep_impli.c

./a.out [-c for case sensitivity ] < word to search for> <file in which to search >

Results
---------
We implemented a testing and data collection strategy comprising five different sizes of input files to ensure the coherence of our gathered data. These levels serve to validate and cross-confirm patterns identified by the others. The datasets for testing are structured as text (.txt) files, with the first file containing 40 lines, the second containing 70 lines, and the remaining three escalating in size to 100,000 lines, 500,000 lines, and 1,000,000 lines, respectively. This multifaceted approach allowed us to capture diverse data across varying input dataset sizes. Detailed results and specifications for each input are presented in the tables below.

40 lines  – Rust

Result - more time but lesser CPU utilisation
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/63d7aec2-7802-45e6-8be1-6b4bfc1464c0)

40 lines  – C 

Result - less time but more CPU utilization
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/92e23985-a34f-49a8-9862-dd9c9d9f40e3)

70 lines - Rust
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/71a9db84-cae1-4702-b443-eb396b1e6a3f)

70 lines - C
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/3a9b92ed-241b-4fd8-8494-5194f718a352)


100,000 lines - Rust
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/fa9c8276-3776-4fe0-92e9-f2df093f1c46)

100,000 lines - C
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/5d5cd5a2-166d-4fae-8170-a4679a149c5f)



500,000 lines - Rust 

Result - more time but lesser CPU utilisation
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/12a166d9-299b-4cd8-90c1-8f8dc985b0c7)



500,000 lines – C 

Result - less time but more CPU utilization
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/c2107eff-fe3a-4cd1-bca2-4b54711a73b5)


1 million+ lines - Rust 

Result - more time but lesser CPU utilisation
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/0a3e2e13-e98c-452d-9ed0-22f54b531f3e)


1 million+ lines – C 

Result - less time but more CPU utilization
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/2b4e13fb-0988-4b1f-b217-a622cc176d2a)

Tabulated Results For Time
---------------------------
![Screenshot from 2023-11-17 15-55-18](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/abf46738-333e-4f48-9b28-71d877c2e3d7)
![Screenshot from 2023-11-17 15-56-46](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/f2f34e89-6b36-42cf-a69c-eff3b77df50a)

We notice a trend where the Rust implementation takes greater time than C, even at higher levels of input. This could be due to some kind of load overheads in the crates utilised, or how the memory allocation and deallocation is working. 

Tabulated Results For Memory Utilisation
----------
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/b75becd1-afa7-4bae-bb56-dc07e0673ceb)
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/75fdbce6-a93c-4d80-977e-ce54b2eccdae)

We can see that the memory utilisation in Rust is better than that in C. The data leaked or left unallocated is supposed to be minimal, so no extra memory is used due to mismanagement. 

Difference in data when failing to deallocate memory in C - this is not possible to do in Rust due to how the langauge is designed
----
Both non deallocated and definitely leaked memory is seen, reflecting poorer memory management in C.
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/b1ef3aba-db81-4e1b-a6a6-165c40742fb6)


Future work
-----------
- Utilisation of Valgrind or a Valgrind like profiling tool to analyse the Rust code as well to develop an in depth understanding of the exact memory advantages of the Rust implemenation. Currently, Valrgind itself has poor interfacing with Rust and does not read properly DWARF5 as generated by Clang14 (https://bugs.kde.org/show_bug.cgi?id=452758), and hence we are reliant on system monitor data alone.
- Check if the above analysis holds for other types of CLI tools.
- Compare error handling mechanisms in Rust (Result, Option) with C (return codes, error flags, etc.) and evaluate how these mechanisms affect code readability, robustness, and ease of maintenance.


References
----------
Crate used for rich text (colours for found words and errors) - https://docs.rs/r3bl_rs_utils/latest/r3bl_rs_utils/

Resources for learning Rust - https://doc.rust-lang.org/book/

impl keyword - https://doc.rust-lang.org/std/keyword.impl.html

Accepting command line arguments in Rust and minigrep - https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html#:~:text=Reading%20the%20Argument%20Values,line%20arguments%20passed%20to%20minigrep%20.

Grep source code - https://github.com/git/git/blob/master/grep.c
