# f1 language
File extentsion is .f1l (**f1 l**anguage)  
Made for f1 fans (i guess)  
Nah but fr I found it funny how box_opposite really is branch if not equal (bne).

# Comments
Comments are done by the \# character. Notice that the program panics if comment character is *NOT* after a whitespace.  
For example:  
GOOD -> box_box 1 #skips next line  
BAD -> box_box 1#skips next line

# Registers
There are 4 registers, all named after a recent f1 driver. Each driver is 2 bit. In order: 
alonso - Only one thats actually special. The only driver who works with I/O supposed to be $v0. Named alonso since he returns.
verstappen - Meant to be $a0 but doesnt really matter. Named verstappen since he sends it into the braking zone.
ricciardo - Meant to be $a1 but doesnt really matter. Named ricciardo since he sends it into the braking zone.
mazepin - Meant to be $t0 but doesnt really matter. Named mazepin because he was temporary. 

---
# Operations
There are 8 operations. 2 are jump instructions. 1 is I/O. 2 are Register immidiate. 3 are register only.  
Each operations is 3 bits instruction.

### Register only
plan - adds or sub. driver 2 to driver. Formatted: <driver> plan <driver2>. (Uses 7 bits and 1 useless bit)  
quali_mode - Pushes the value of driver to the top of the stack. Formatted: quali_mode <driver>. (Uses only 5 bits)  
cooldown_lap - Retrieves the value on top of the stack and assigns it to driver. Formatted: cooldown_lap <driver>. (Uses only 5 bits)  

### Register immidiate
target_lap - sets a driver to a 3 bit imm -4 <-> 3. Formatted: <driver> target_lap <imm>. (8 bit)  
target_plus - adds or subs an imm to the driver for an 3 bit imm <-> 3. Format: <driver> target_plus <imm>. (8 bit)  

### Jump instruction
box_opposite - if the drivers are not equal skip next line. Formatted: <driver1> box_opposite <driver2>. Skips next line if <driver1> != <driver2>. (7 bit, last bit useless)  
box_box - go to the n:th line relative to this statement unconditionally. Formatted: box_box <imm>. For a 5 bit imm. -16 <-> 15. (Uses 8 bit)  

### Syscall
fia - handles I/O and exit call. Formatted: fia <imm>  if <imm> = 0 -> it reads input to alonso
                                                    if <imm> > 0 -> it prints alonso
                                                    if <imm> < 0 -> it exits.

# Notice
There is no stack_pointer. It only goes off when pushed/popped.

# TO RUN FACTORIAL FUNCTION
Execute main.rs in src/interpreter/main.rs and input a number n and it will output n! Wouldn't advise to try for n > 8
There is no input validation so please dont abuse it. If the variable file_name is not set to "file.f1l". Set it to it!
  
# TO RUN YOUR OWN CODE
Write your code in a file (preferably extension .f1l). Go into main.rs and change variable file_name to the file name :skull: and execute main.rs
