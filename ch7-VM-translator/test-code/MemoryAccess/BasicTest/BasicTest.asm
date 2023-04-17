// push constant 10
@10
D=A
@SP
M=M+1
A=M
M=D
// pop local 0
@SP
M=M-1
A=M+1
D=M
@LCL
A=A+0
M=D
// push constant 21
@21
D=A
@SP
M=M+1
A=M
M=D
// push constant 22
@22
D=A
@SP
M=M+1
A=M
M=D
// pop argument 2
@SP
M=M-1
A=M+1
D=M
@ARG
A=A+2
M=D
// pop argument 1
@SP
M=M-1
A=M+1
D=M
@ARG
A=A+1
M=D
// push constant 36
@36
D=A
@SP
M=M+1
A=M
M=D
// pop this 6
@SP
M=M-1
A=M+1
D=M
@THIS
A=A+6
M=D
// push constant 42
@42
D=A
@SP
M=M+1
A=M
M=D
// push constant 45
@45
D=A
@SP
M=M+1
A=M
M=D
// pop that 5
@SP
M=M-1
A=M+1
D=M
@THAT
A=A+5
M=D
// pop that 2
@SP
M=M-1
A=M+1
D=M
@THAT
A=A+2
M=D
// push constant 510
@510
D=A
@SP
M=M+1
A=M
M=D
// pop temp 6
@SP
M=M-1
A=M+1
D=M
@TEMP
A=A+6
M=D
// push local 0
@LCL
A=A+0
D=M
@SP
M=M+1
A=M
M=D
// push that 5
@THAT
A=A+5
D=M
@SP
M=M+1
A=M
M=D
// add
@SP
M=M-1
A=M+1
D=M
A=A-1
M=M+D
// push argument 1
@ARG
A=A+1
D=M
@SP
M=M+1
A=M
M=D
// sub
@SP
M=M-1
A=M+1
D=M
A=A-1
M=M-D
// push this 6
@THIS
A=A+6
D=M
@SP
M=M+1
A=M
M=D
// push this 6
@THIS
A=A+6
D=M
@SP
M=M+1
A=M
M=D
// add
@SP
M=M-1
A=M+1
D=M
A=A-1
M=M+D
// sub
@SP
M=M-1
A=M+1
D=M
A=A-1
M=M-D
// push temp 6
@TEMP
A=A+6
D=M
@SP
M=M+1
A=M
M=D
// add
@SP
M=M-1
A=M+1
D=M
A=A-1
M=M+D
// EXIT
@EXIT_LOOP
0;JMP
(EXIT_LOOP)
@EXIT_LOOP
0;JMP