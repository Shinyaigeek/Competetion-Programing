n = input();
ans = 0;
for a in range(0,n) :
    for b in range(a,n) :
        if (str(a)[0] == str(b)[len(str(b))]):
            if(str(b)[0] == str(a)[len(str(a))]):
                ans += 1;
print(ans);