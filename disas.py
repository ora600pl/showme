import gdb, sys

def show_last_instruction(disas):
    cnt = 0
    pos = 0
    for instr in disas:
        instr_words = instr.split()
        if len(instr_words) > 4 and instr_words[0] == "=>":
            print(disas[cnt-1])
            print(instr)
            pos = cnt - 1 
        cnt += 1
    return pos

def parse_ldr(disas):
    instruction = disas.split(":")[1]
    source_reg = instruction.split("[")[1][:-1]
    if source_reg.count("sp") < 1:
        source_reg = source_reg.split(",")[0]

    return source_reg

def parse_ldp(disas):
    instruction = disas.split(":")[1]
    source_reg = instruction.split("[")[1][:-1]
    return source_reg

def parse_stp(disas):
    instruction = disas.split(":")[1]
    source_reg = instruction.split(",")[0][4:].strip()
    return source_reg

def parse_mov(disas):
    instruction = disas.split(":")[1]
    source_reg = instruction.split(",")[1].strip()
    return source_reg


def track_register(reg, pos_start):
    print("Tracking register ", reg)
    disas = gdb.execute("disas", to_string=True).split("\n")
    cp = pos_start
    source_reg = reg
    while cp > 1:
        mnemo = disas[cp].split(":")[1]
        if mnemo.count("ldr") > 0 and mnemo.split(",")[0].count(reg) > 0:
            source_reg = parse_ldr(disas[cp])
            print(disas[cp])
            break
        elif mnemo.count("ldp") > 0 and (mnemo.split(",")[0].count(reg) > 0 or mnemo.split(",")[1].count(reg) > 0):
            source_reg = parse_ldp(disas[cp])
            print(disas[cp])
            break
        elif mnemo.count("stp") > 0 and (mnemo.split("[")[1].count(reg) > 0):
            source_reg = parse_stp(disas[cp])
            print(disas[cp])
            break
        elif mnemo.count("mov") > 0 and mnemo.split(",")[0].count(reg) > 0:
            source_reg = parse_mov(disas[cp])
            print(disas[cp])
            if source_reg == "x0" and disas[cp-1].count("bl") > 0:
                print(disas[cp-1])

            break
            
        cp -= 1
    if cp > 0:
        track_register(source_reg, cp-1)
    else:
        exit
    

if __name__ == "__main__":
    disas = gdb.execute("disas", to_string=True).split("\n")
    pos = show_last_instruction(disas)
    print("\n --------------------------------- \n")
    track_register("x0", pos)

