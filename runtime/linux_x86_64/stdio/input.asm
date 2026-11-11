; Bounded scanf parsing over the shared peek/consume input stream.
; The scanner owns this frame; conversion helpers receive its address in R15.
%define LCC_IN_ARGS       0
%define LCC_IN_INDEX      40
%define LCC_IN_STACK      48
%define LCC_IN_COUNT      56
%define LCC_IN_WIDTH      64
%define LCC_IN_SUPPRESS   72
%define LCC_IN_LENGTH     80
%define LCC_IN_SPEC       88
%define LCC_IN_DEST       96
%define LCC_IN_USED       104
%define LCC_IN_NEG        112
%define LCC_IN_BASE       120
%define LCC_IN_DIGITS     128
%define LCC_IN_STATE      136
%define LCC_IN_TOKEN      160
%define LCC_IN_FRAME      4264
%include "runtime/linux_x86_64/stdio/input_format.asm"
%include "runtime/linux_x86_64/stdio/input_integer.asm"
%include "runtime/linux_x86_64/stdio/input_float.asm"
