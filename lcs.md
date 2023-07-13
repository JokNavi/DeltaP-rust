The Longest Common Subsequence (LCS) is a classic computer science problem that finds the longest subsequence common to all sequences in a set of sequences (often just two sequences). It differs from problems of finding common substrings: unlike substrings, subsequences are not required to occupy consecutive positions within the original sequences.

The LCS problem is a classic computer science problem, the study of which has led to the development of important techniques for the broader field of algorithm design. The problem is solved by dynamic programming approach. Here is a step-by-step guide on how to implement the LCS algorithm in Python.

1. **Initialize the 2D Matrix**

We start by initializing a 2D matrix `L` of size `(m+1) * (n+1)`, where `m` and `n` are the lengths of the two input strings. This matrix will be used to store the lengths of the LCS of the substrings of the input strings. The size of the matrix is `m+1` and `n+1` because the first row and the first column are used as base cases for the dynamic programming solution and are set to 0.

```python
def lcs(X, Y): 
    m = len(X) 
    n = len(Y) 
  
    L = [[0 for j in range(n+1)] for i in range(m+1)] 
```

2. **Fill the Matrix**

Then, we fill the matrix in a bottom-up manner. For each cell `L[i][j]`, if the characters in the input strings at index `i-1` and `j-1` are the same, we set `L[i][j]` to `L[i-1][j-1] + 1`. Otherwise, we set it to the maximum of `L[i-1][j]` and `L[i][j-1]`.

```python
for i in range(m+1): 
    for j in range(n+1): 
        if i == 0 or j == 0 : 
            L[i][j] = 0
        elif X[i-1] == Y[j-1]: 
            L[i][j] = L[i-1][j-1]+1
        else: 
            L[i][j] = max(L[i-1][j], L[i][j-1]) 
```

3. **Backtrack to Construct the LCS**

Finally, we construct the LCS by backtracking from the bottom right cell of the matrix to the top left. If the characters at the current cell in the input strings are the same, we include this character in the LCS. If they are different, we move to the cell with the larger value.

```python
index = L[m][n] 
lcs = [""] * (index+1) 
lcs[index] = "" 

i = m 
j = n 
while i > 0 and j > 0: 
    if X[i-1] == Y[j-1]: 
        lcs[index-1] = X[i-1] 
        i-=1
        j-=1
        index-=1
    elif L[i-1][j] > L[i][j-1]: 
        i-=1
    else: 
        j-=1
```

The LCS of the input strings is then printed.

```python
print("LCS of " + X + " and " + Y + " is " + "".join(lcs)) 
```

All these steps are implemented in the function `lcs`.

Here is the full code:

```python
def lcs(X, Y): 
    m = len(X) 
    n = len(Y) 
  
    L = [[0 for j in range(n+1)] for i in range(m+1)] 
  
    for i in range(m+1): 
        for j in range(n+1): 
            if i == 0 or j == 0 : 
                L[i][j] = 0
            elif X[i-1] == Y[j-1]: 
                L[i][j] = L[i-1][j-1]+1
            else: 
                L[i][j] = max(L[i-1][j], L[i][j-1]) 
  
    index = L[m][n] 
    lcs = [""] * (index+1) 
    lcs[index] = "" 
  
    i = m 
    j = n 
    while i > 0 and j > 0: 
        if X[i-1] == Y[j-1]: 
            lcs[index-1] = X[i-1] 
            i-=1
            j-=1
            index-=1
        elif L[i-1][j] > L[i][j-1]: 
            i-=1
        else: 
            j-=1
    print("LCS of " + X + " and " + Y + " is " + "".join(lcs)) 
```
    