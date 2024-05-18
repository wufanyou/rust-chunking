from rusk_chunking import MyTextSplitter

m = MyTextSplitter(max_tokens=256, file="/Users/fanyou/Documents/KDD2024/rusk-chunking/resource/tokenizer.json")

import time

s = time.time()
output = m.chunks_batch_v2(['a'*5000]*32)
print(time.time()-s)

# print(output)
