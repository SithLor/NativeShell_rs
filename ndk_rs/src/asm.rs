/*++ NDK Version: 0095

Copyright (c) Alex Ionescu.  All rights reserved.

Header Name:

asm.h

Abstract:

ASM Offsets for dealing with de-referencing structures in registers.
C-compatible version of the file ks386.inc present in the newest WDK.

Author:

Alex Ionescu (alex.ionescu@reactos.com)   06-Oct-2004

--*/




// asm.rs

// NEW_SCHEDULER

// PCR Access
#[cfg(feature = "__asm__")]
#[cfg(feature = "config_smp")]
const PCR: &str = "fs:";

#[cfg(feature = "__asm__")]
#[cfg(not(feature = "config_smp"))]
const PCR: &str = "ds:[0xFFDFF000]";

//
// CPU Modes
//
const KernelMode: usize = 0x0;
const UserMode: usize = 0x1;

//
// CPU Types
//
const CPU_INTEL: usize = 0x1;
const CPU_AMD: usize = 0x2;

//
// Selector Names
//
#[cfg(feature = "__asm__")]
const RPL_MASK: usize = 0x0003;
const MODE_MASK: usize = 0x0001;
const KGDT_R0_CODE: usize = (0x8);
const KGDT_R0_DATA: usize = (0x10);
const KGDT_R3_CODE: usize = (0x18);
const KGDT_R3_DATA: usize = (0x20);
const KGDT_TSS: usize = (0x28);
const KGDT_R0_PCR: usize = (0x30);
const KGDT_R3_TEB: usize = (0x38);
const KGDT_LDT: usize = (0x48);
const KGDT_DF_TSS: usize = (0x50);
const KGDT_NMI_TSS: usize = (0x58);

//
// KV86M_REGISTERS Offsets
//
const KV86M_REGISTERS_EBP: usize = 0x0;
const KV86M_REGISTERS_EDI: usize = 0x4;
const KV86M_REGISTERS_ESI: usize = 0x8;
const KV86M_REGISTERS_EDX: usize = 0xC;
const KV86M_REGISTERS_ECX: usize = 0x10;
const KV86M_REGISTERS_EBX: usize = 0x14;
const KV86M_REGISTERS_EAX: usize = 0x18;
const KV86M_REGISTERS_DS: usize = 0x1C;
const KV86M_REGISTERS_ES: usize = 0x20;
const KV86M_REGISTERS_FS: usize = 0x24;
const KV86M_REGISTERS_GS: usize = 0x28;
const KV86M_REGISTERS_EIP: usize = 0x2C;
const KV86M_REGISTERS_CS: usize = 0x30;
const KV86M_REGISTERS_EFLAGS: usize = 0x34;
const KV86M_REGISTERS_ESP: usize = 0x38;
const KV86M_REGISTERS_SS: usize = 0x3C;
const TF_SAVED_EXCEPTION_STACK: usize = 0x8C;
const TF_REGS: usize = 0x90;
const TF_ORIG_EBP: usize = 0x94;

//
// TSS Offsets
//
const KTSS_ESP0: usize = 0x4;
const KTSS_CR3: usize = 0x1C;
const KTSS_EFLAGS: usize = 0x24;
const KTSS_IOMAPBASE: usize = 0x66;
const KTSS_IO_MAPS: usize = 0x68;

//
// KTHREAD Offsets
//
const KTHREAD_DEBUG_ACTIVE: usize = 0x03;
const KTHREAD_INITIAL_STACK: usize = 0x18;
const KTHREAD_STACK_LIMIT: usize = 0x1C;
const KTHREAD_TEB: usize = 0x74;
const KTHREAD_KERNEL_STACK: usize = 0x20;
const KTHREAD_ALERTED: usize = 0x5E;
const KTHREAD_APCSTATE_PROCESS: usize = 0x28 + 0x10;
const KTHREAD_PENDING_USER_APC: usize = 0x28 + 0x16;
const KTHREAD_PENDING_KERNEL_APC: usize = 0x28 + 0x15;
const KTHREAD_CONTEXT_SWITCHES: usize = 0x48;
const KTHREAD_STATE_: usize = 0x4C;
const KTHREAD_NPX_STATE: usize = 0x4D;
const KTHREAD_WAIT_IRQL: usize = 0x4E;
const KTHREAD_NEXT_PROCESSOR: usize = 0x40;
const KTHREAD_WAIT_REASON: usize = 0x5A;
const KTHREAD_PRIORITY: usize = 0x5B;
const KTHREAD_SWAP_BUSY: usize = 0x5D;
const KTHREAD_SERVICE_TABLE: usize = 0x118;
const KTHREAD_PREVIOUS_MODE: usize = 0xD7;
const KTHREAD_COMBINED_APC_DISABLE: usize = 0x70;
const KTHREAD_SPECIAL_APC_DISABLE: usize = 0x72;
const KTHREAD_LARGE_STACK: usize = 0x107;
const KTHREAD_TRAP_FRAME: usize = 0x110;
const KTHREAD_CALLBACK_STACK: usize = 0x114;
const KTHREAD_APC_STATE_INDEX: usize = 0x11C;
const KTHREAD_STACK_BASE: usize = 0x158;
const KTHREAD_QUANTUM: usize = 0x15D;
const KTHREAD_KERNEL_TIME: usize = 0x160;
const KTHREAD_USER_TIME: usize = 0x18C;

//
// KPROCESS Offsets
//
const KPROCESS_DIRECTORY_TABLE_BASE: usize = 0x18;
const KPROCESS_LDT_DESCRIPTOR0: usize = 0x20;
const KPROCESS_LDT_DESCRIPTOR1: usize = 0x24;
const KPROCESS_INT21_DESCRIPTOR0: usize = 0x28;
const KPROCESS_INT21_DESCRIPTOR1: usize = 0x2C;
const KPROCESS_IOPM_OFFSET: usize = 0x30;
const KPROCESS_ACTIVE_PROCESSORS: usize = 0x34;
const EPROCESS_VDM_OBJECTS: usize = 0x144;

//
// KTIMER_TABLE Offsets
//
#[cfg(feature = "__asm__")]
const KTIMER_TABLE_ENTRY: usize = 0x00;
const KTIMER_TABLE_TIME: usize = 0x08;
const TIMER_ENTRY_SIZE: usize = 0x10;
const TIMER_TABLE_SIZE: usize = 0x200;

//
// KPRCB Offsets
//
const KPRCB_DR0: usize = 0x2F8;
const KPRCB_DR1: usize = 0x2FC;
const KPRCB_DR2: usize = 0x300;
const KPRCB_DR3: usize = 0x304;
const KPRCB_DR6: usize = 0x308;
const KPRCB_DR7: usize = 0x30C;
const KPRCB_TIMER_HAND: usize = 0x964;
const KPRCB_TIMER_REQUEST: usize = 0x968;

//
// KPCR Offsets
//
const KPCR_EXCEPTION_LIST: usize = 0x0;
const KPCR_INITIAL_STACK: usize = 0x4;
const KPCR_STACK_LIMIT: usize = 0x8;
const KPCR_PERF_GLOBAL_GROUP_MASK: usize = 0x8;
const KPCR_CONTEXT_SWITCHES: usize = 0x10;
const KPCR_SET_MEMBER_COPY: usize = 0x14;
const KPCR_TEB: usize = 0x18;
const KPCR_SELF: usize = 0x1C;
const KPCR_PRCB: usize = 0x20;
const KPCR_IRQL: usize = 0x24;
const KPCR_IRR: usize = 0x28;
const KPCR_IRR_ACTIVE: usize = 0x2C;
const KPCR_IDR: usize = 0x30;
const KPCR_KD_VERSION_BLOCK: usize = 0x34;
const KPCR_IDT: usize = 0x38;
const KPCR_GDT: usize = 0x3C;
const KPCR_TSS: usize = 0x40;
const KPCR_STALL_SCALE_FACTOR: usize = 0x4C;
const KPCR_SET_MEMBER: usize = 0x48;
const KPCR_NUMBER: usize = 0x51;
const KPCR_VDM_ALERT: usize = 0x54;
const KPCR_PRCB_DATA: usize = 0x120;
const KPCR_CURRENT_THREAD: usize = 0x124;
const KPCR_PRCB_NEXT_THREAD: usize = 0x128;
const KPCR_PRCB_IDLE_THREAD: usize = 0x12C;
const KPCR_PROCESSOR_NUMBER: usize = 0x130;
const KPCR_PRCB_SET_MEMBER: usize = 0x134;
const KPCR_PRCB_CPU_TYPE: usize = 0x138;
const KPCR_NPX_THREAD: usize = 0x640;
const KPCR_DR6: usize = 0x428;
const KPCR_DR7: usize = 0x42C;
const KPCR_PRCB_INTERRUPT_COUNT: usize = 0x644;
const KPCR_PRCB_KERNEL_TIME: usize = 0x648;
const KPCR_PRCB_USER_TIME: usize = 0x64C;
const KPCR_PRCB_DPC_TIME: usize = 0x650;
const KPCR_PRCB_DEBUG_DPC_TIME: usize = 0x654;
const KPCR_PRCB_INTERRUPT_TIME: usize = 0x658;
const KPCR_PRCB_ADJUST_DPC_THRESHOLD: usize = 0x65C;
const KPCR_PRCB_SKIP_TICK: usize = 0x664;
const KPCR_SYSTEM_CALLS: usize = 0x6B8;
const KPCR_PRCB_DPC_QUEUE_DEPTH: usize = 0xA4C;
const KPCR_PRCB_DPC_COUNT: usize = 0xA50;
const KPCR_PRCB_DPC_STACK: usize = 0xA68;
const KPCR_PRCB_MAXIMUM_DPC_QUEUE_DEPTH: usize = 0xA6C;
const KPCR_PRCB_DPC_REQUEST_RATE: usize = 0xA70;
const KPCR_PRCB_DPC_INTERRUPT_REQUESTED: usize = 0xA78;
const KPCR_PRCB_DPC_ROUTINE_ACTIVE: usize = 0xA7A;
const KPCR_PRCB_DPC_LAST_COUNT: usize = 0xA80;
const KPCR_PRCB_TIMER_REQUEST: usize = 0xA88;
const KPCR_PRCB_QUANTUM_END: usize = 0xAA1;
const KPCR_PRCB_DEFERRED_READY_LIST_HEAD: usize = 0xC10;
const KPCR_PRCB_POWER_STATE_IDLE_FUNCTION: usize = 0xEC0;

//
// KINTERRUPT Offsets
//
const KINTERRUPT_SERVICE_ROUTINE: usize = 0x0C;
const KINTERRUPT_SERVICE_CONTEXT: usize = 0x10;
const KINTERRUPT_TICK_COUNT: usize = 0x18;
const KINTERRUPT_ACTUAL_LOCK: usize = 0x1C;
const KINTERRUPT_IRQL: usize = 0x20;
const KINTERRUPT_VECTOR: usize = 0x24;
const KINTERRUPT_SYNCHRONIZE_IRQL: usize = 0x29;
const KINTERRUPT_DISPATCH_COUNT: usize = 0x38;

//
// KGDTENTRY Offsets
//
const KGDT_BASE_LOW: usize = 0x2;
const KGDT_BASE_MID: usize = 0x4;
const KGDT_BASE_HI: usize = 0x7;
const KGDT_LIMIT_HI: usize = 0x6;
const KGDT_LIMIT_LOW: usize = 0x0;

//
// FPU Save Area Offsets
//
const FP_CONTROL_WORD: usize = 0x0;
const FP_STATUS_WORD: usize = 0x4;
const FP_TAG_WORD: usize = 0x8;
const FP_ERROR_OFFSET: usize = 0xC;
const FP_ERROR_SELECTOR: usize = 0x10;
const FP_DATA_OFFSET: usize = 0x14;
const FP_DATA_SELECTOR: usize = 0x18;
const FN_CR0_NPX_STATE: usize = 0x20C;
const SIZEOF_FX_SAVE_AREA: usize = 528;
const NPX_FRAME_LENGTH: usize = 0x210;

//
// FX Save Area Offsets
//
const FX_CONTROL_WORD: usize = 0x0;
const FX_STATUS_WORD: usize = 0x2;
const FX_TAG_WORD: usize = 0x4;
const FX_ERROR_OPCODE: usize = 0x6;
const FX_ERROR_OFFSET: usize = 0x8;
const FX_ERROR_SELECTOR: usize = 0xC;
const FX_DATA_OFFSET: usize = 0x10;
const FX_DATA_SELECTOR: usize = 0x14;
const FX_MXCSR: usize = 0x18;

//
// NPX States
//
const NPX_STATE_NOT_LOADED: usize = 0xA;
const NPX_STATE_LOADED: usize = 0x0;

//
// Trap Frame Offsets
//
const KTRAP_FRAME_DEBUGEBP: usize = 0x0;
const KTRAP_FRAME_DEBUGEIP: usize = 0x4;
const KTRAP_FRAME_DEBUGARGMARK: usize = 0x8;
const KTRAP_FRAME_DEBUGPOINTER: usize = 0xC;
const KTRAP_FRAME_TEMPCS: usize = 0x10;
const KTRAP_FRAME_TEMPESP: usize = 0x14;
const KTRAP_FRAME_DR0: usize = 0x18;
const KTRAP_FRAME_DR1: usize = 0x1C;
const KTRAP_FRAME_DR2: usize = 0x20;
const KTRAP_FRAME_DR3: usize = 0x24;
const KTRAP_FRAME_DR6: usize = 0x28;
const KTRAP_FRAME_DR7: usize = 0x2C;
const KTRAP_FRAME_GS: usize = 0x30;
const KTRAP_FRAME_RESERVED1: usize = 0x32;
const KTRAP_FRAME_ES: usize = 0x34;
const KTRAP_FRAME_RESERVED2: usize = 0x36;
const KTRAP_FRAME_DS: usize = 0x38;
const KTRAP_FRAME_RESERVED3: usize = 0x3A;
const KTRAP_FRAME_EDX: usize = 0x3C;
const KTRAP_FRAME_ECX: usize = 0x40;
const KTRAP_FRAME_EAX: usize = 0x44;
const KTRAP_FRAME_PREVIOUS_MODE: usize = 0x48;
const KTRAP_FRAME_EXCEPTION_LIST: usize = 0x4C;
const KTRAP_FRAME_FS: usize = 0x50;
const KTRAP_FRAME_RESERVED4: usize = 0x52;
const KTRAP_FRAME_EDI: usize = 0x54;
const KTRAP_FRAME_ESI: usize = 0x58;
const KTRAP_FRAME_EBX: usize = 0x5C;
const KTRAP_FRAME_EBP: usize = 0x60;
const KTRAP_FRAME_ERROR_CODE: usize = 0x64;
const KTRAP_FRAME_EIP: usize = 0x68;
const KTRAP_FRAME_CS: usize = 0x6C;
const KTRAP_FRAME_EFLAGS: usize = 0x70;
const KTRAP_FRAME_ESP: usize = 0x74;
const KTRAP_FRAME_SS: usize = 0x78;
const KTRAP_FRAME_RESERVED5: usize = 0x7A;
const KTRAP_FRAME_V86_ES: usize = 0x7C;
const KTRAP_FRAME_RESERVED6: usize = 0x7E;
const KTRAP_FRAME_V86_DS: usize = 0x80;
const KTRAP_FRAME_RESERVED7: usize = 0x82;
const KTRAP_FRAME_V86_FS: usize = 0x84;
const KTRAP_FRAME_RESERVED8: usize = 0x86;
const KTRAP_FRAME_V86_GS: usize = 0x88;
const KTRAP_FRAME_RESERVED9: usize = 0x8A;
const KTRAP_FRAME_SIZE: usize = 0x8C;
const KTRAP_FRAME_LENGTH: usize = 0x8C;
const KTRAP_FRAME_ALIGN: usize = 0x04;
const FRAME_EDITED: usize = 0xFFF8;

//
// KUSER_SHARED_DATA Offsets
//
#[cfg(feature = "__asm__")]
const USER_SHARED_DATA: usize = 0xFFDF0000;
const USER_SHARED_DATA_INTERRUPT_TIME: usize = 0x8;
const USER_SHARED_DATA_SYSTEM_TIME: usize = 0x14;
const USER_SHARED_DATA_TICK_COUNT: usize = 0x320;

//
// KUSER_SHARED_DATA Offsets (this stuff is trash)
//
const KERNEL_USER_SHARED_DATA: usize = 0x7FFE0000;
const KUSER_SHARED_PROCESSOR_FEATURES: usize = KERNEL_USER_SHARED_DATA + 0x274;
const KUSER_SHARED_SYSCALL: usize = KERNEL_USER_SHARED_DATA + 0x300;
const KUSER_SHARED_SYSCALL_RET: usize = KERNEL_USER_SHARED_DATA + 0x304;
const PROCESSOR_FEATURE_FXSR: usize = KUSER_SHARED_PROCESSOR_FEATURES + 0x4;

//
// CONTEXT Offsets
//
const CONTEXT_FLAGS: usize = 0x0;
const CONTEXT_DR6: usize = 0x14;
const CONTEXT_FLOAT_SAVE: usize = 0x1C;
const CONTEXT_SEGGS: usize = 0x8C;
const CONTEXT_SEGFS: usize = 0x90;
const CONTEXT_SEGES: usize = 0x94;
const CONTEXT_SEGDS: usize = 0x98;
const CONTEXT_EDI: usize = 0x9C;
const CONTEXT_ESI: usize = 0xA0;
const CONTEXT_EBX: usize = 0xA4;
const CONTEXT_EDX: usize = 0xA8;
const CONTEXT_ECX: usize = 0xAC;
const CONTEXT_EAX: usize = 0xB0;
const CONTEXT_EBP: usize = 0xB4;
const CONTEXT_EIP: usize = 0xB8;
const CONTEXT_SEGCS: usize = 0xBC;
const CONTEXT_EFLAGS: usize = 0xC0;
const CONTEXT_ESP: usize = 0xC4;
const CONTEXT_SEGSS: usize = 0xC8;
const CONTEXT_FLOAT_SAVE_CONTROL_WORD: usize = CONTEXT_FLOAT_SAVE + FP_CONTROL_WORD;
const CONTEXT_FLOAT_SAVE_STATUS_WORD: usize = CONTEXT_FLOAT_SAVE + FP_STATUS_WORD;
const CONTEXT_FLOAT_SAVE_TAG_WORD: usize = CONTEXT_FLOAT_SAVE + FP_TAG_WORD;
const CONTEXT_ALIGNED_SIZE: usize = 0x2CC;

//
// EXCEPTION_RECORD Offsets
//
const EXCEPTION_RECORD_EXCEPTION_CODE: usize = 0x0;
const EXCEPTION_RECORD_EXCEPTION_FLAGS: usize = 0x4;
const EXCEPTION_RECORD_EXCEPTION_RECORD: usize = 0x8;
const EXCEPTION_RECORD_EXCEPTION_ADDRESS: usize = 0xC;
const EXCEPTION_RECORD_NUMBER_PARAMETERS: usize = 0x10;
const SIZEOF_EXCEPTION_RECORD: usize = 0x14;
const EXCEPTION_RECORD_LENGTH: usize = 0x50;

//
// Exception types
//
#[cfg(feature = "__asm__")]
const EXCEPTION_NONCONTINUABLE: usize = 0x0001;
const EXCEPTION_UNWINDING: usize = 0x0002;
const EXCEPTION_EXIT_UNWIND: usize = 0x0004;
const EXCEPTION_STACK_INVALID: usize = 0x0008;
const EXCEPTION_NESTED_CALL: usize = 0x00010;
const EXCEPTION_TARGET_UNWIND: usize = 0x00020;
const EXCEPTION_COLLIDED_UNWIND: usize = 0x00040;
const EXCEPTION_UNWIND: usize = 0x00066;
const EXCEPTION_EXECUTE_HANDLER: usize = 0x00001;
const EXCEPTION_CONTINUE_SEARCH: usize = 0x00000;
const EXCEPTION_CONTINUE_EXECUTION: usize = 0xFFFFFFFF;
const EXCEPTION_CHAIN_END: usize = 0xFFFFFFFF;

//
// TEB Offsets
//
const TEB_EXCEPTION_LIST: usize = 0x0;
const TEB_STACK_BASE: usize = 0x4;
const TEB_STACK_LIMIT: usize = 0x8;
const TEB_FIBER_DATA: usize = 0x10;
const TEB_PEB: usize = 0x30;
const TEB_EXCEPTION_CODE: usize = 0x1A4;
const TEB_ACTIVATION_CONTEXT_STACK_POINTER: usize = 0x1A8;
const TEB_DEALLOCATION_STACK: usize = 0xE0C;
const TEB_GDI_BATCH_COUNT: usize = 0xF70;
const TEB_GUARANTEED_STACK_BYTES: usize = 0xF78;
const TEB_FLS_DATA: usize = 0xFB4;

//
// PEB Offsets
//
const PEB_KERNEL_CALLBACK_TABLE: usize = 0x2C;

//
// FIBER Offsets
//
const FIBER_PARAMETER: usize = 0x0;
const FIBER_EXCEPTION_LIST: usize = 0x4;
const FIBER_STACK_BASE: usize = 0x8;
const FIBER_STACK_LIMIT: usize = 0xC;
const FIBER_DEALLOCATION_STACK: usize = 0x10;
const FIBER_CONTEXT: usize = 0x14;
const FIBER_GUARANTEED_STACK_BYTES: usize = 0x2E0;
const FIBER_FLS_DATA: usize = 0x2E4;
const FIBER_ACTIVATION_CONTEXT_STACK: usize = 0x2E8;
const FIBER_CONTEXT_FLAGS: usize = FIBER_CONTEXT + CONTEXT_FLAGS;
const FIBER_CONTEXT_EAX: usize = FIBER_CONTEXT + CONTEXT_EAX;
const FIBER_CONTEXT_EBX: usize = FIBER_CONTEXT + CONTEXT_EBX;
const FIBER_CONTEXT_ECX: usize = FIBER_CONTEXT + CONTEXT_ECX;
const FIBER_CONTEXT_EDX: usize = FIBER_CONTEXT + CONTEXT_EDX;
const FIBER_CONTEXT_ESI: usize = FIBER_CONTEXT + CONTEXT_ESI;
const FIBER_CONTEXT_EDI: usize = FIBER_CONTEXT + CONTEXT_EDI;
const FIBER_CONTEXT_EBP: usize = FIBER_CONTEXT + CONTEXT_EBP;
const FIBER_CONTEXT_ESP: usize = FIBER_CONTEXT + CONTEXT_ESP;
const FIBER_CONTEXT_DR6: usize = FIBER_CONTEXT + CONTEXT_DR6;
const FIBER_CONTEXT_FLOAT_SAVE_STATUS_WORD: usize = FIBER_CONTEXT + CONTEXT_FLOAT_SAVE_STATUS_WORD;
const FIBER_CONTEXT_FLOAT_SAVE_CONTROL_WORD: usize = FIBER_CONTEXT + CONTEXT_FLOAT_SAVE_CONTROL_WORD;
const FIBER_CONTEXT_FLOAT_SAVE_TAG_WORD: usize = FIBER_CONTEXT + CONTEXT_FLOAT_SAVE_TAG_WORD;

//
// EFLAGS
//
#[cfg(feature = "__asm__")]
const EFLAGS_TF: usize = 0x100;
const EFLAGS_INTERRUPT_MASK: usize = 0x200;
const EFLAGS_NESTED_TASK: usize = 0x4000;
const EFLAGS_V86_MASK: usize = 0x20000;
const EFLAGS_ALIGN_CHECK: usize = 0x40000;
const EFLAGS_VIF: usize = 0x80000;
const EFLAGS_VIP: usize = 0x100000;
const EFLAG_SIGN: usize = 0x8000;
const EFLAG_ZERO: usize = 0x4000;
const EFLAG_SELECT: usize = (EFLAG_SIGN + EFLAG_ZERO);
const EFLAGS_USER_SANITIZE: usize = 0x3F4DD7;

//
// CR0
//
const CR0_PE: usize = 0x1;
const CR0_MP: usize = 0x2;
const CR0_EM: usize = 0x4;
const CR0_TS: usize = 0x8;
const CR0_ET: usize = 0x10;
const CR0_NE: usize = 0x20;
const CR0_WP: usize = 0x10000;
const CR0_AM: usize = 0x40000;
const CR0_NW: usize = 0x20000000;
const CR0_CD: usize = 0x40000000;
const CR0_PG: usize = 0x80000000;

//
// CR4
//
#[cfg(feature = "__asm__")]
const CR4_VME: usize = 0x1;
const CR4_PVI: usize = 0x2;
const CR4_TSD: usize = 0x4;
const CR4_DE: usize = 0x8;
const CR4_PSE: usize = 0x10;
const CR4_PAE: usize = 0x20;
const CR4_MCE: usize = 0x40;
const CR4_PGE: usize = 0x80;
const CR4_FXSR: usize = 0x200;
const CR4_XMMEXCPT: usize = 0x400;

//
// DR6 and 7 Masks
//
const DR6_LEGAL: usize = 0xE00F;
const DR7_LEGAL: usize = 0xFFFF0155;
const DR7_ACTIVE: usize = 0x55;
const DR7_OVERRIDE_V: usize = 0x04;
const DR7_RESERVED_MASK: usize = 0xDC00;
const DR7_OVERRIDE_MASK: usize = 0xF0000;

//
// Usermode callout frame definitions
//
const CBSTACK_STACK: usize = 0x0;
const CBSTACK_TRAP_FRAME: usize = 0x4;
const CBSTACK_CALLBACK_STACK: usize = 0x8;
const CBSTACK_EBP: usize = 0x18;
const CBSTACK_RESULT: usize = 0x20;
const CBSTACK_RESULT_LENGTH: usize = 0x24;

//
// NTSTATUS and Bugcheck Codes
//
#[cfg(feature = "__asm__")]
const STATUS_ACCESS_VIOLATION: usize = 0xC0000005;
const STATUS_IN_PAGE_ERROR: usize = 0xC0000006;
const STATUS_GUARD_PAGE_VIOLATION: usize = 0x80000001;
const STATUS_PRIVILEGED_INSTRUCTION: usize = 0xC0000096;
const STATUS_STACK_OVERFLOW: usize = 0xC00000FD;
const KI_EXCEPTION_ACCESS_VIOLATION: usize = 0x10000004;
const STATUS_INVALID_SYSTEM_SERVICE: usize = 0xC000001C;
const STATUS_NO_CALLBACK_ACTIVE: usize = 0xC0000258;
const STATUS_CALLBACK_POP_STACK: usize = 0xC0000423;
const STATUS_ARRAY_BOUNDS_EXCEEDED: usize = 0xC000008C;
const STATUS_ILLEGAL_INSTRUCTION: usize = 0xC000001D;
const STATUS_INVALID_LOCK_SEQUENCE: usize = 0xC000001E;
const STATUS_BREAKPOINT: usize = 0x80000003;
const STATUS_SINGLE_STEP: usize = 0x80000004;
const STATUS_INTEGER_DIVIDE_BY_ZERO: usize = 0xC0000094;
const STATUS_INTEGER_OVERFLOW: usize = 0xC0000095;
const STATUS_FLOAT_DENORMAL_OPERAND: usize = 0xC000008D;
const STATUS_FLOAT_DIVIDE_BY_ZERO: usize = 0xC000008E;
const STATUS_FLOAT_INEXACT_RESULT: usize = 0xC000008F;
const STATUS_FLOAT_INVALID_OPERATION: usize = 0xC0000090;
const STATUS_FLOAT_OVERFLOW: usize = 0xC0000091;
const STATUS_FLOAT_STACK_CHECK: usize = 0xC0000092;
const STATUS_FLOAT_UNDERFLOW: usize = 0xC0000093;
const STATUS_FLOAT_MULTIPLE_FAULTS: usize = 0xC00002B4;
const STATUS_FLOAT_MULTIPLE_TRAPS: usize = 0xC00002B5;
const APC_INDEX_MISMATCH: usize = 0x01;
const IRQL_NOT_GREATER_OR_EQUAL: usize = 0x09;
const IRQL_NOT_LESS_OR_EQUAL: usize = 0x0A;
const TRAP_CAUSE_UNKNOWN: usize = 0x12;
const KMODE_EXCEPTION_NOT_HANDLED: usize = 0x13;
const IRQL_GT_ZERO_AT_SYSTEM_SERVICE: usize = 0x4A;
const UNEXPECTED_KERNEL_MODE_TRAP: usize = 0x7F;
const ATTEMPTED_SWITCH_FROM_DPC: usize = 0xB8;
const HARDWARE_INTERRUPT_STORM: usize = 0xF2;

//
// IRQL Levels
//
const PASSIVE_LEVEL: usize = 0x0;
const APC_LEVEL: usize = 0x1;
const DISPATCH_LEVEL: usize = 0x2;
const CLOCK2_LEVEL: usize = 0x1C;
const HIGH_LEVEL: usize = 0x1F;

//
// Quantum Decrements
//
const CLOCK_QUANTUM_DECREMENT: usize = 0x3;

//
// System Call Table definitions
//
const NUMBER_SERVICE_TABLES: usize = 0x0002;
const SERVICE_NUMBER_MASK: usize = 0x0FFF;
const SERVICE_TABLE_SHIFT: usize = 0x0008;
const SERVICE_TABLE_MASK: usize = 0x0010;
const SERVICE_TABLE_TEST: usize = 0x0010;
const SERVICE_DESCRIPTOR_BASE: usize = 0x0000;
const SERVICE_DESCRIPTOR_COUNT: usize = 0x0004;
const SERVICE_DESCRIPTOR_LIMIT: usize = 0x0008;
const SERVICE_DESCRIPTOR_NUMBER: usize = 0x000C;
const SERVICE_DESCRIPTOR_LENGTH: usize = 0x0010;

//
// VDM State Pointer
//
const FIXED_NTVDMSTATE_LINEAR_PC_AT: usize = 0x714;

//
// Machine types
//
#[cfg(feature = "__asm__")]
const MACHINE_TYPE_ISA: usize = 0x0000;
const MACHINE_TYPE_EISA: usize = 0x0001;
const MACHINE_TYPE_MCA: usize = 0x0002;

//
// Kernel Feature Bits
//
const KF_RDTSC: usize = 0x00000002;

//
// Kernel Stack Size
//
const KERNEL_STACK_SIZE: usize = 0x3000;

//
// Generic Definitions
//
const PRIMARY_VECTOR_BASE: usize = 0x30; // FIXME: HACK;
const MAXIMUM_IDTVECTOR: usize = 0xFF;

