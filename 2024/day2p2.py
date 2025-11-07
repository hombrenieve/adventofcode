

def is_report_safe(line):
    report_sign = 0
    for i in range(0, len(line)-1):
        if line[i] > line[i+1]:
            if report_sign == 1:
                return False
            report_sign = -1
        elif line[i] < line[i+1]:
            if report_sign == -1:
                return False
            report_sign = 1
        if(abs(line[i]-line[i+1]) > 3):
            return False
        if line[i] == line[i+1]:
            return False
    return True

def check(report):
    if is_report_safe(report):
        return True
    for i in range(0, len(report)):
        backup = report.copy()
        del backup[i]
        if is_report_safe(backup):
            return True
    return False


    


safe_reports = 0
for line in open('input.txt'):
    line = [int(i) for i in line.strip().split(' ')]
    if check(line):
        safe_reports += 1
print(safe_reports)