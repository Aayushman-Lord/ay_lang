
def make_name(n):
    name = ""
    while n:
        n, r = divmod(n - 1, 26)
        name = chr(97 + r) + name
    return "var" + name


lines = []

# 10,000 variable declarations
for i in range(1, 100001):
    name = make_name(i)

    if i == 1:
        expression = "1"
    else:
        previous = make_name(i - 1)
        expression = f"({previous} + 17) * 3 / 3 - 17"

    lines.append(f"let {name} = {expression};")

# 2,000 additional arithmetic operations
last = make_name(10000)
final_expression = " + ".join([last] * 2001)

lines.append(final_expression + ";")

with open("file.ayl", "w") as f:
    f.write("\n".join(lines))

print("Generated 10,000 variables!")
print("Generated 40,000+ arithmetic operations!")
print("Ready to benchmark AY_LANG!")
