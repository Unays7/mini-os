# Mini OS

## Resources

- **Phil Opp's Blog**: [Writing an OS in Rust](https://os.phil-opp.com/)
- **Linux Insides**: [GitHub](https://github.com/0xAX/linux-insides)
- **Bootloader ASM**: [GitHub](https://github.com/Stefan20162016/linux-insides-code/blob/master/bootloader.asm)
- **Rustonomicon**: [Rustonomicon](https://doc.rust-lang.org/nomicon/intro.html)

---

## Phase 2: Core Kernel Features

### Phil Opp

- [ ] Testing
- [ ] CPU Exceptions
- [ ] Double Faults

### Linux Insides (Read Alongside)

- [ ] **Interrupts** → Introduction
- [ ] **Interrupts** → Interrupt handlers
- [ ] **Interrupts** → Implementation of some exception handlers

**Milestone**: Handle CPU exceptions (divide by zero, page fault, etc.)

---

## Phase 3: Hardware Interrupts & Input

### Phil Opp

- [ ] Hardware Interrupts
- [ ] Introduction to Paging

### Linux Insides (Read Alongside)

- [ ] **Interrupts** → Dive into external hardware interrupts
- [ ] **Theory** → Paging (skim for concepts)

**Milestone**: Handle keyboard interrupts, understand paging theory

---

## Phase 4: Memory Management

### Phil Opp

- [ ] Paging Implementation
- [ ] Heap Allocation
- [ ] Allocator Designs

### Linux Insides (Read Alongside)

- [ ] **Memory Management** → Memblock
- [ ] **Memory Management** → Fixmaps and ioremap
- [ ] **Theory** → Paging (deep dive)

**Milestone**: Implement virtual memory, heap allocator (dynamic memory!)

---

## Phase 5: Multitasking

### Phil Opp

- [ ] Async/Await

### Linux Insides (Read Alongside)

- [ ] **Initialization** → Scheduler initialization
- [ ] **SMP** → Concepts

**Milestone**: Cooperative multitasking with async/await

---

## Phase 6: Build Your Own Bootloader 🔥

**Switch Focus**: Remove `bootloader` crate dependency

### Linux Insides - Booting (In Order)

- [ ] From bootloader to kernel
- [ ] First steps in the kernel setup code
- [ ] Video mode initialization and transition to protected mode
- [ ] Transition to 64-bit mode
- [ ] Kernel decompression

### Your Implementation

- [ ] Write bootloader in Rust/Assembly
- [ ] Set up GDT (Global Descriptor Table)
- [ ] Switch from Real Mode → Protected Mode → Long Mode
- [ ] Load kernel into memory
- [ ] Jump to kernel's `_start()`

**Milestone**: Boot your kernel WITHOUT the bootloader crate

---

## Phase 7: Advanced Kernel Features

### Linux Insides - System Calls

- [ ] Introduction to system calls
- [ ] How the Linux kernel handles a system call
- [ ] vsyscall and vDSO
- [ ] How the Linux kernel runs a program
- [ ] Implementation of the open system call

### Linux Insides - Synchronization

- [ ] Introduction to spinlocks
- [ ] Queued spinlocks
- [ ] Semaphores
- [ ] Mutex
- [ ] Reader/Writer semaphores
- [ ] RCU

### Linux Insides - Timers

- [ ] Clocksource framework
- [ ] Introduction to timers
- [ ] Clockevents framework

### Linux Insides - Data Structures

- [ ] Doubly linked list
- [ ] Radix tree
- [ ] Bit arrays

**Milestone**: Implement syscalls, locks, timers, efficient data structures

---

## Phase 8: SMP & Process Management

### Linux Insides

- [ ] **SMP** → Per-CPU variables
- [ ] **SMP** → Cpumasks
- [ ] **SMP** → The initcall mechanism
- [ ] **Cgroups** → Introduction to Control Groups

**Milestone**: Multi-core support, process isolation

---

## Learning Strategy

### Phase 1-5 (Phil Opp Era)

1. **Code Phil's tutorial first** - get it working
2. **Read Linux Insides sections** - understand real implementation
3. **Take notes** - concepts, not code
4. **Don't implement Linux's approach** - just learn

### Phase 6+ (Linux Insides Era)

1. **Linux Insides is your guide** - reference material
2. **Implement selectively** - pick features that interest you
3. **Compare with Linux** - see how pros do it
4. **Iterate** - refactor your code with new knowledge
