* 
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
* File:			README.TXT.
* Author:		Mauro Scomparin <http://scompoprojects.worpress.com>.
* Version:		1.0.0.
* Description:	Readme file for the project.
*

This is a sample project to be used with stellaris LM4Fxxx TI microcontrollers.
The main reason why I started doing this project is that Stellarisware 
(C lybraries for LM4F microcontrollers by Texas Instruments 
<http://www.ti.com/tool/sw-ek-lm4f120xl>) has a free and royalty-free license,
while the code examples are available with restrictive license that you can't
use for your own projects.
The other reason it's I wanted to know what's going on behind writing a simple
"$ make".
I hope this can be useful for someone.

Thanks a lot to the following people:

https://github.com/Wollw/stellaris-launchpad-template-gcc
Thanks to this guy who did a great cleaning job in the Makefile 

https://github.com/utzig/lm4tools
I use this project to load the program to the target!

http://fun-tech.se/stm32/index.php
Without this man this would not have existed!

http://www.bravegnu.org/gnu-eprog/c-startup.html
I found this quite explanatory too!