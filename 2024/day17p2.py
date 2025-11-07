import math


class Opcode:
    def __init__(self, opcode, machine):
        self.opcode = opcode
        self.machine = machine

    def execute(self, operand):
        pass

class Operand:
    def __init__(self, value, machine):
        self.value = value
        self.machine = machine
    
    def get(self):
        if self.value >= 0 and self.value < 4:
            return self.value
        if self.value == 4:
            return self.machine.reg_a
        if self.value == 5:
            return self.machine.reg_b
        if self.value == 6:
            return self.machine.reg_c
    
    def get_literal(self):
        return self.value

class Adv(Opcode):
    # opcode 0
    def execute(self, operand):
        self.machine.reg_a = int(self.machine.reg_a / pow(2, operand.get()))
        self.machine.pc += 2

class Bxl(Opcode):
    # opcode 1
    def execute(self, operand):
        self.machine.reg_b ^= operand.get_literal()
        self.machine.pc += 2

class Bst(Opcode):
    # opcode 2
    def execute(self, operand):
        self.machine.reg_b = operand.get() % 8
        self.machine.pc += 2

class Jnz(Opcode):
    # opcode 3
    def execute(self, operand):
        if self.machine.reg_a != 0:
            self.machine.pc = operand.get_literal()
        else:
            self.machine.pc += 2

class Bxc(Opcode):
    # opcode 4
    def execute(self, operand):
        self.machine.reg_b ^= self.machine.reg_c
        operand.get()
        self.machine.pc += 2

class Out(Opcode):
    # opcode 5
    def execute(self, operand):
        self.machine.output.append(operand.get() % 8)
        self.machine.pc += 2

class Bdv(Opcode):
    # opcode 6
    def execute(self, operand):
        self.machine.reg_b = int(self.machine.reg_a / pow(2, operand.get()))
        self.machine.pc += 2

class Cdv(Opcode):
    # opcode 7
    def execute(self, operand):
        self.machine.reg_c = int(self.machine.reg_a / pow(2, operand.get()))
        self.machine.pc += 2

def get_instruction(opcode, machine):
    opcodes = [Adv, Bxl, Bst, Jnz, Bxc, Out, Bdv, Cdv]
    return opcodes[opcode](opcode, machine)

class Machine:
    def __init__(self, reg_a, reg_b, reg_c, program):
        self.reg_a = reg_a
        self.reg_b = reg_b
        self.reg_c = reg_c
        self.program = program
        self.output = []
        self.pc = 0
    
    def run(self):
        while self.pc < len(self.program):
            get_instruction(self.program[self.pc], self).execute(Operand(self.program[self.pc + 1], self))

    def get_output(self):
        return self.output

class Machine_loader:
    def __init__(self, reg_b, reg_c, program):
        self.reg_b = reg_b
        self.reg_c = reg_c
        self.program = program
    
    def run(self, reg_a):
        m = Machine(reg_a, self.reg_b, self.reg_c, self.program)
        m.run()
        return m.get_output()

def calculate_cost(list1, list2):
    len1 = len(list1)
    len2 = len(list2)
    if len1-len2 != 0:
        return len1-len2
    list_res = [abs(x - y) for x, y in zip(list1, list2)]
    return sum(list_res)/len(list_res)
        



def gradient_descent(ml):
    found = False
    parameter = 35184432937081
    increment = 1
    cost = 0
    while not found:
        predicted = ml.run(parameter)
        if predicted == ml.program:
            found = True
            return parameter
        cost = calculate_cost(predicted, ml.program)
        parameter += increment
        print("Parameter: ", parameter, " Cost: ", cost, " Increment: ", increment)
        print("Predicted: ", predicted, " Program: ", ml.program)
        



with open('input.txt') as f:
    content = [x.strip().split(' ') for x in f.readlines()]
    reg_a = int(content[0][2])
    reg_b = int(content[1][2])
    reg_c = int(content[2][2])
    program = [int(x) for x in content[4][1].split(',')]
    ml = Machine_loader(reg_b, reg_c, program)
    res = gradient_descent(ml)
    print("Found: ", res)