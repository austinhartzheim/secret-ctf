c={cipher_flag}
f=open('cat.jpg','rb');f.seek({offset});k=f.read(len(c))
p=[k[i]^c[i] for i in range(len(c))]
print(bytes(p))
