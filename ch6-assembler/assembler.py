# from os import listdir, path
# import regex as re

# RE = re.compile('^[\d*]')

# CODE_GRAMMAR = {
#     A: {

#     },
#     C: {

#     }
# }

# dir = './input_code/'
# filename = listdir(dir)[0]

# with open(dir + filename) as f:
#     content = f.readlines()

# content_clean = []
# for line in content:
#     line = line.strip()
#     if (line[0:1] == '') or (line[0:2] == '//'):
#         continue
#     content_clean.append(line)
#     print(line)

# vars = {}
# for (index, line) in enumerate(content_clean):
#     if line[0] == '@' and not RE.match(line[1:]):
#         var = line[1:]
#         vars.setdefault(var, index+1)

# instructions = []
# for (index, line) in enumerate(content_clean):
#     if line[0] == '@':
#         addrOrVar = line[1:]
#         if not RE.match(addrOrVar):
#             addr = vars[addrOrVar]
#         else:
#             addr = addrOrVar

#         addr = int(line[1:])
#         instructions.append('0'+format(addr, '015b'))

    
    

# with open('out.hack', 'w') as f:
#     for line in instructions:
#         f.write(line+'\n') 

a = {1, 2, 3}
b = a.copy()
a.add(4)
print(b)