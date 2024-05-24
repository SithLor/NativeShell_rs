def escape_char(c):
    if c in ['\\', '\"']:
        return '\\' + c
    return c

print("match data {")
for i in range(32, 127):
    print(f"    {i} => final_data = \"{escape_char(chr(i))}\",")
print("    _ => final_data[i] = \"Unknown\",")
print("}")