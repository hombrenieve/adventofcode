
with open("input.txt") as f:
    content = f.readlines()
    content = [x.strip() for x in content]
    valid = 0
    for passphrase in content:
        passphrase_sp = passphrase.split(" ")
        if len(passphrase_sp) == 1:
            continue
        passphrase_sp = [sorted(x) for x in passphrase_sp]
        while len(passphrase_sp) > 1:
            if passphrase_sp[0] in passphrase_sp[1:]:
                break
            passphrase_sp.remove(passphrase_sp[0])
        if len(passphrase_sp) == 1:
            valid += 1
    print(valid)