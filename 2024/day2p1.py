
def sign(report):
    # 1 if increasing, -1 if decreasing
    # if not return 0
    report_sign = 0
    for i in range(0, len(report)-1):
        if report[i] > report[i+1]:
            if report_sign == 1:
                return 0
            report_sign = -1
        elif report[i] < report[i+1]:
            if report_sign == -1:
                return 0
            report_sign = 1
    return report_sign


def is_report_safe(line):
    if sign(line) == 0:
        return False
    for i in range(0, len(line)-1):
        if(abs(line[i]-line[i+1]) > 3):
            return False
        if line[i] == line[i+1]:
            return False
    return True

safe_reports = 0
for line in open('input.txt'):
    line = [int(i) for i in line.strip().split(' ')]
    if is_report_safe(line):
        safe_reports += 1
print(safe_reports)