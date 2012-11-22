/*
* Copyright (c) 2012, Mauro Scomparin
* All rights reserved.
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*     * Redistributions of source code must retain the above copyright
*       notice, this list of conditions and the following disclaimer.
*     * Redistributions in binary form must reproduce the above copyright
*       notice, this list of conditions and the following disclaimer in the
*       documentation and/or other materials provided with the distribution.
*     * Neither the name of Mauro Scomparin nor the
*       names of its contributors may be used to endorse or promote products
*       derived from this software without specific prior written permission.
*
* THIS SOFTWARE IS PROVIDED BY Mauro Scomparin ``AS IS'' AND ANY
* EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
* WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL Mauro Scomparin BE LIABLE FOR ANY
* DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
* (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
* LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
* ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
* (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
* SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*
* File:			LM4F.ld.
* Author:		Mauro Scomparin <http://scompoprojects.worpress.com>.
* Version:		1.0.0.
* Description:	Linker description file for LM4FXXX microcontrollers.
*/


/*
* Memory definition:
* FLASH:	start point 0x00,		lenght 0x40000.
* SRAM:		start point 0x20000000	length 0x8000.
* STACK:	start point 0x20007FFF	lenght 0x0.
*/
MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x00040000
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 0x00008000
    STACK (rwx) : ORIGIN = 0x20007FFF , LENGTH = 0x00000000
}

/*
* Sections definitions:
*
* .text 		- machine instructions.
* .data 		- initialized data defined in the program.
* .bss  		- un-initialized global and static variables (to be initialized to 0 before starting main).
* .stack		- just contains the pointer to the stack end at the right place.
*/
SECTIONS
{	
	/* This section it's the code, containing the NVIC Vector table that must start at 0x0
	*   Look at the LM4F120H5QR datasheet for details. (Table 2-8. Exception Types)
	*/
    .text : 
    {
    	/*_start_text = .;	/* This is an index to the beginning of .text segment. */
    	KEEP(*(.nvic_table)) /* I should keep the NVIC ISR Table because it's needed by the processor to start. */
        *(.text.*)			/* This contains the code after the ISR table. */
        *(.rodata.*)    	/* Read only data.  */
        _end_text = .;		/* This is an index to the end of .text segment. */
    }>FLASH
    
    /* 
    * .data segment must be placed in RAM but it's originally stored in FLASH
    * So I set the data segment in ram, but I specify the load address with the AT
    * keyword to set that right after the .text section.
    * (Look at the LD documentation. (Optional Section Attributes)) 
    * Thanks https://github.com/utzig for the hints!
    */
    .data : 
    {
    	_start_data = .;	/* An index to the beginning of .data segment. */
        *(vtable)			/* vtable should be the volatile data I guess*/
        *(.data.*)			/* I should put there all my initialized data of my program. */
        _end_data = .;		/* And another index to the end of .data segment. */
    }>RAM AT >FLASH
    

	/* 
	* .bss contains the unitialized variables and must be set as 0x0 during runtime.
	* It should be loaded in RAM and particula care should be taken initializing them in the startup file. 
	*/
    .bss : 
    {
    	_start_bss = .;		/* This is an index to the beginning of .bss segment. */
        *(.bss.*)       	/* The un-initialized data should go there.  */
        *(COMMON)			/* All the other stuff should be put there */
        _end_bss = .;		/* End index for .bss segment */
    }>RAM
    
	/*
	* .stack contains nothing, but I use it to set the first vector item (SP R13).
	*/
	.stack : 
	{
		_stack_top = .; /* An index to the end of the stack */
	}>STACK
	
}
