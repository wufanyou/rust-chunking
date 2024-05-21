from rusk_chunking import MyTextSplitter

m = MyTextSplitter(max_tokens=512, overlap = 128, file="/Users/fanyou/Documents/KDD2024/rusk-chunking/resource/tokenizer.json")

import time

with open('/Users/fanyou/Documents/KDD2024/eda/finance/161_2024-02-28.html') as f:
    file = f.read()

s = time.time()
output = m.chunks_batch([file])

for x in output:
    for y in x:
        print(y)
        #print(len(y))

print(time.time()-s)

# print(output)
