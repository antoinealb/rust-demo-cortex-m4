# ARM Cortex-Mx common makefile scripts and rules.

##############################################################################
# Processing options coming from the upper Makefile.
#

# Compiler options
OPT = $(USE_OPT)
COPT = $(USE_COPT)
CPPOPT = $(USE_CPPOPT)

# Garbage collection
ifeq ($(USE_LINK_GC),yes)
  OPT += -ffunction-sections -fdata-sections -fno-common
  LDOPT := ,--gc-sections
else
  LDOPT :=
endif

# Linker extra options
ifneq ($(USE_LDOPT),)
  LDOPT := $(LDOPT),$(USE_LDOPT)
endif

# Link time optimizations
ifeq ($(USE_LTO),yes)
  OPT += -flto
endif

# FPU-related options
ifeq ($(USE_FPU),)
  USE_FPU = no
endif
ifneq ($(USE_FPU),no)
  OPT += -mfloat-abi=$(USE_FPU) -mfpu=fpv4-sp-d16 -fsingle-precision-constant
  DDEFS += -DCORTEX_USE_FPU=TRUE
  DADEFS += -DCORTEX_USE_FPU=TRUE
else
  DDEFS += -DCORTEX_USE_FPU=FALSE
  DADEFS += -DCORTEX_USE_FPU=FALSE
endif

# Process stack size
ifeq ($(USE_PROCESS_STACKSIZE),)
  LDOPT := $(LDOPT),--defsym=__process_stack_size__=0x400
else
  LDOPT := $(LDOPT),--defsym=__process_stack_size__=$(USE_PROCESS_STACKSIZE)
endif

# Exceptions stack size
ifeq ($(USE_EXCEPTIONS_STACKSIZE),)
  LDOPT := $(LDOPT),--defsym=__main_stack_size__=0x400
else
  LDOPT := $(LDOPT),--defsym=__main_stack_size__=$(USE_EXCEPTIONS_STACKSIZE)
endif

# Output directory and files
ifeq ($(BUILDDIR),)
  BUILDDIR = build
endif
ifeq ($(BUILDDIR),.)
  BUILDDIR = build
endif
OUTFILES = $(BUILDDIR)/$(PROJECT).elf \
		   $(BUILDDIR)/$(PROJECT).hex \
		   $(BUILDDIR)/$(PROJECT).bin \
		   $(BUILDDIR)/$(PROJECT).dmp \
		   $(BUILDDIR)/$(PROJECT).list

ifdef SREC
OUTFILES += $(BUILDDIR)/$(PROJECT).srec
endif

# Source files groups and paths
ifeq ($(USE_THUMB),yes)
  TCSRC += $(CSRC)
  TCPPSRC += $(CPPSRC)
else
  ACSRC += $(CSRC)
  ACPPSRC += $(CPPSRC)
endif
ASRC      = $(ACSRC)$(ACPPSRC)
TSRC      = $(TCSRC)$(TCPPSRC)
SRCPATHS  = $(sort $(dir $(ASMXSRC)) $(dir $(ASMSRC)) $(dir $(ASRC)) $(dir $(TSRC)))

# Various directories
OBJDIR    = $(BUILDDIR)/obj
LSTDIR    = $(BUILDDIR)/lst

# Object files groups
ACOBJS    = $(addprefix $(OBJDIR)/, $(notdir $(ACSRC:.c=.o)))
ACPPOBJS  = $(addprefix $(OBJDIR)/, $(notdir $(ACPPSRC:.cpp=.o)))
TCOBJS    = $(addprefix $(OBJDIR)/, $(notdir $(TCSRC:.c=.o)))
TCPPOBJS  = $(addprefix $(OBJDIR)/, $(notdir $(TCPPSRC:.cpp=.o)))
ASMOBJS   = $(addprefix $(OBJDIR)/, $(notdir $(ASMSRC:.s=.o)))
ASMXOBJS  = $(addprefix $(OBJDIR)/, $(notdir $(ASMXSRC:.S=.o)))
RUSTOBJS  = $(addprefix $(OBJDIR)/, $(notdir $(RUSTSRC:.rs=.o)))
OBJS      = $(ASMXOBJS) $(ASMOBJS) $(ACOBJS) $(TCOBJS) $(ACPPOBJS) $(TCPPOBJS) $(RUSTOBJS)

# Paths
IINCDIR   = $(patsubst %,-I%,$(INCDIR) $(DINCDIR) $(UINCDIR))
LLIBDIR   = $(patsubst %,-L%,$(DLIBDIR) $(ULIBDIR))

# Macros
DEFS      = $(DDEFS) $(UDEFS)
ADEFS     = $(DADEFS) $(UADEFS)

# Libs
LIBS      = $(DLIBS) $(ULIBS)

# Various settings
MCFLAGS   = -mcpu=$(MCU)
ODFLAGS   = -x --syms
ASFLAGS   = $(MCFLAGS) -Wa,-amhls=$(LSTDIR)/$(notdir $(<:.s=.lst)) $(ADEFS)
ASXFLAGS  = $(MCFLAGS) -Wa,-amhls=$(LSTDIR)/$(notdir $(<:.S=.lst)) $(ADEFS)
CFLAGS    = $(MCFLAGS) $(OPT) $(COPT) $(CWARN) -Wa,-alms=$(LSTDIR)/$(notdir $(<:.c=.lst)) $(DEFS)
CPPFLAGS  = $(MCFLAGS) $(OPT) $(CPPOPT) $(CPPWARN) -Wa,-alms=$(LSTDIR)/$(notdir $(<:.cpp=.lst)) $(DEFS)
LDFLAGS   = $(MCFLAGS) $(OPT) -nostartfiles $(LLIBDIR) -Wl,-Map=$(BUILDDIR)/$(PROJECT).map,--cref,--no-warn-mismatch,--library-path=$(RULESPATH),--script=$(LDSCRIPT)$(LDOPT)

# Thumb interwork enabled only if needed because it kills performance.
ifneq ($(TSRC),)
  CFLAGS   += -DTHUMB_PRESENT
  CPPFLAGS += -DTHUMB_PRESENT
  ASFLAGS  += -DTHUMB_PRESENT
  ifneq ($(ASRC),)
	# Mixed ARM and THUMB mode.
	CFLAGS   += -mthumb-interwork
	CPPFLAGS += -mthumb-interwork
	ASFLAGS  += -mthumb-interwork
	LDFLAGS  += -mthumb-interwork
  else
	# Pure THUMB mode, THUMB C code cannot be called by ARM asm code directly.
	CFLAGS   += -mno-thumb-interwork -DTHUMB_NO_INTERWORKING
	CPPFLAGS += -mno-thumb-interwork -DTHUMB_NO_INTERWORKING
	ASFLAGS  += -mno-thumb-interwork -DTHUMB_NO_INTERWORKING -mthumb
	LDFLAGS  += -mno-thumb-interwork -mthumb
  endif
else
  # Pure ARM mode
  CFLAGS   += -mno-thumb-interwork
  CPPFLAGS += -mno-thumb-interwork
  ASFLAGS  += -mno-thumb-interwork
  LDFLAGS  += -mno-thumb-interwork
endif

# Generate dependency information
ASFLAGS  += -MD -MP -MF .dep/$(@F).d
CFLAGS   += -MD -MP -MF .dep/$(@F).d
CPPFLAGS += -MD -MP -MF .dep/$(@F).d

# Paths where to search for sources
VPATH     = $(SRCPATHS)

COLOR       = \033[1;34m
COLOR_CLEAR = \033[0m

COLOR_PRINTF = printf "$(COLOR)%s$(COLOR_CLEAR)\n"

#
# Makefile rules
#

all: PRE_MAKE_ALL_RULE_HOOK $(OBJS) $(OUTFILES) POST_MAKE_ALL_RULE_HOOK

PRE_MAKE_ALL_RULE_HOOK:

POST_MAKE_ALL_RULE_HOOK:

$(OBJS): | $(BUILDDIR) $(OBJDIR) $(LSTDIR)

$(BUILDDIR):
ifneq ($(USE_VERBOSE_COMPILE),yes)
	@echo Compiler Options
	@echo $(CC) -c $(CFLAGS) -I. $(IINCDIR) main.c -o main.o
	@echo
endif
	@mkdir -p $(BUILDDIR)

$(OBJDIR):
	@mkdir -p $(OBJDIR)

$(LSTDIR):
	@mkdir -p $(LSTDIR)

$(ACPPOBJS) : $(OBJDIR)/%.o : %.cpp Makefile $(GLOBAL_SRC_DEP)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	@echo
	$(CPPC) -c $(CPPFLAGS) $(AOPT) -I. $(IINCDIR) $< -o $@
else
	@$(COLOR_PRINTF) "Compiling $(<F)"
	@$(CPPC) -c $(CPPFLAGS) $(AOPT) -I. $(IINCDIR) $< -o $@
endif

$(TCPPOBJS) : $(OBJDIR)/%.o : %.cpp Makefile $(GLOBAL_SRC_DEP)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	@echo
	$(CPPC) -c $(CPPFLAGS) $(TOPT) -I. $(IINCDIR) $< -o $@
else
	@$(COLOR_PRINTF) "Compiling $(<F)"
	@$(CPPC) -c $(CPPFLAGS) $(TOPT) -I. $(IINCDIR) $< -o $@
endif

$(ACOBJS) : $(OBJDIR)/%.o : %.c Makefile $(GLOBAL_SRC_DEP)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	@echo
	$(CC) -c $(CFLAGS) $(AOPT) -I. $(IINCDIR) $< -o $@
else
	@$(COLOR_PRINTF) "Compiling $(<F)"
	@$(CC) -c $(CFLAGS) $(AOPT) -I. $(IINCDIR) $< -o $@
endif

$(TCOBJS) : $(OBJDIR)/%.o : %.c Makefile $(GLOBAL_SRC_DEP)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	@echo
	$(CC) -c $(CFLAGS) $(TOPT) -I. $(IINCDIR) $< -o $@
else
	@$(COLOR_PRINTF) "Compiling $(<F)"
	@$(CC) -c $(CFLAGS) $(TOPT) -I. $(IINCDIR) $< -o $@
endif

$(ASMOBJS) : $(OBJDIR)/%.o : %.s Makefile $(GLOBAL_SRC_DEP)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	@echo
	$(AS) -c $(ASFLAGS) -I. $(IINCDIR) $< -o $@
else
	@$(COLOR_PRINTF) "Compiling $(<F)"
	@$(AS) -c $(ASFLAGS) -I. $(IINCDIR) $< -o $@
endif

$(ASMXOBJS) : $(OBJDIR)/%.o : %.S Makefile $(GLOBAL_SRC_DEP)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	@echo
	$(CC) -c $(ASXFLAGS) $(TOPT) -I. $(IINCDIR) $< -o $@
else
	@$(COLOR_PRINTF) "Compiling $(<F)"
	@$(CC) -c $(ASXFLAGS) $(TOPT) -I. $(IINCDIR) $< -o $@
endif

$(RUSTOBJS) : $(OBJDIR)/%.o : %.rs Makefile
ifeq ($(USE_VERBOSE_COMPILE),yes)
	@echo
	$(RUSTC) $(RUSTFLAGS) $< -o $@
else
	@echo Compiling $(<F)
	@$(RUSTC) $(RUSTFLAGS) $< -o $@
endif

%.elf: $(OBJS) $(LDSCRIPT)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	@echo
	$(LD) $(OBJS) $(LDFLAGS) $(LIBS) -o $@
else
	@$(COLOR_PRINTF) "Linking $@"
	@$(LD) $(OBJS) $(LDFLAGS) $(LIBS) -o $@
endif

%.hex: %.elf $(LDSCRIPT)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	$(HEX) $< $@
else
	@$(COLOR_PRINTF) "Creating $@"
	@$(HEX) $< $@
endif

%.bin: %.elf $(LDSCRIPT)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	$(BIN) $< $@
else
	@$(COLOR_PRINTF) "Creating $@"
	@$(BIN) $< $@
endif

%.srec: %.elf $(LDSCRIPT)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	$(SREC) $< $@
else
	@$(COLOR_PRINTF) "Creating $@"
	@$(SREC) $< $@
endif

%.dmp: %.elf $(LDSCRIPT)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	$(OD) $(ODFLAGS) $< > $@
	$(SZ) $<
else
	@$(COLOR_PRINTF) "Creating $@"
	@$(OD) $(ODFLAGS) $< > $@
	@echo
	@$(SZ) $<
endif

%.list: %.elf $(LDSCRIPT)
ifeq ($(USE_VERBOSE_COMPILE),yes)
	$(OD) -d $< > $@
else
	@$(COLOR_PRINTF) "Creating $@"
	@$(OD) -d $< > $@
	@echo
	@$(COLOR_PRINTF) "Done"
endif

lib: $(OBJS) $(BUILDDIR)/lib$(PROJECT).a

$(BUILDDIR)/lib$(PROJECT).a: $(OBJS)
	@$(AR) -r $@ $^
	@echo
	@echo Done

clean:
	@echo Cleaning
	-rm -fR .dep $(BUILDDIR)
	@echo
	@echo Done

#
# Include the dependency files, should be the last of the makefile
#
-include $(shell mkdir .dep 2>/dev/null) $(wildcard .dep/*)

# *** EOF ***
