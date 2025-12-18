# mini-os

A minimal x86_64 operating system written in Rust, following and extending [Phil Opp's excellent tutorial](https://os.phil-opp.com/).

## What I've Added (Or better say plan to add hehe)?

- **Custom bootloader** (GDT, mode transitions, kernel loading)
- **System calls** (syscall/sysret, vsyscall, vDSO)
- **Synchronization primitives** (spinlocks, mutexes, semaphores, RCU)
- **Timers & time management** (clocksource, clockevents)
- **Advanced interrupts** (softirq, tasklets, workqueues)
- **Real scheduler** (beyond async/await)
- **SMP support** (per-CPU variables, CPU masks)
- **Process management** (cgroups)
- **Kernel data structures** (linked lists, radix trees, bit arrays)
- **(Maybe??) ACPI Power Management**

## Resources

- **Phil Opp's Blog**: [Writing an OS in Rust](https://os.phil-opp.com/)
- **Linux Insides**: [GitHub](https://github.com/0xAX/linux-insides)
- **Bootloader ASM**: [GitHub](https://github.com/Stefan20162016/linux-insides-code/blob/master/bootloader.asm)
- **Rustonomicon**: [Rustonomicon](https://doc.rust-lang.org/nomicon/intro.html)
- **x86_64 Rust Repo** [x86_64](https://github.com/rust-osdev/x86_64)

---
