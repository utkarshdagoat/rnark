def get_bingint(arr):
    sum = 0
    for i in range(len(arr)):
        sum += arr[i]*(2**(64*i))
    return sum


# # test_base_case_mult
# a =0X7FA1B2C3D4E5F60789ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE0
# b =0xABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF
# print(a+b)
# b_calc = get_bingint( [7542649922918342911, 7542649922918342692, 12379813817064611620, 12379813812177893520])
# print("b result",b_calc - b)


# # print(a>b)
# sub_res = [13527631012418014177, 2882002266266188730, 15987179287544887124, 15263758729759128950]
# result = get_bingint(sub_res)
# print(b-a)
# print(result)

# arr = [17106816733836489504, 8437436160694434591, 17102275007484288737, 17291968141207752297, 6871473844393097586, 16721514386755580715, 8596134145241391739, 6172093223836742082]
# res = get_bingint(arr)
# print(res)
# print("mult",a*b);
# print(a*b - res) # True


# # karatsuba
# a0 = a % (2**(64*2))
# a0_calc = get_bingint([2623536861626805472, 10424652189184531423])
# print(a0 - a0_calc) #0 

# b0 = b % (2**(64*2))
# b0_calc = get_bingint([7542649922918342911, 7542649922918342692] )
# print(b0 - b0_calc) #0

# c0 = a0*b0
# c0_calc =  get_bingint([17106816733836489504, 8437436160694434591, 2696075392888458932, 4262513846184196733])
# print(c0 - c0_calc)

# a1 = a // (2**(64*2))
# a1_calc = get_bingint([9920249030899947128, 9196828468227470855] )
# print(a1 - a1_calc) #0
# b1 = b // (2**(64*2))
# b1_calc = get_bingint([12379813817064611620, 12379813812177893520])
# print(b1 - b1_calc) #0

# c1 = a1*b1
# c1_calc = get_bingint( [10389936785480659168, 7192767197262303206, 8596134145241391739, 6172093223836742082] )
# print(c1 - c1_calc) #0

# diff = abs(a0 - a1)
# diff_1 = get_bingint( [11150031904436409960, 1227823720957060567] )
# print(diff - diff_1) #0

# diff2 = abs(b0 - b1)
# diff2_calc = get_bingint([4837163894146266981, 4837163889259550828] )
# print(diff2) #0
# print(diff2 -diff2_calc) #0



# ## ODD EVEN KARASTUBA
# print("ODD EVEN KARASTUBA")
# a = 249166486039954038678241653028365438597414117400323598999
# b = 4062882539673212545835653973091204775
# result_arr=  [1635125010864364161, 11949138821930240557, 16131851141360144773, 11615463313061491774, 8742688488232360] 
# result = get_bingint(result_arr)
# print(result)
# print(a*b)


# print("\n\n")
print("DIVISION")
# a = 4486116365047057285615619245805733533662098016812356305731015669708311065831995465137397594684684578328578381687188591572342160574912716102443372011598624
# b = 0xABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF
a=4486116365047057285615619245805733533662098016812356305731015669708311065831995465137397594684684578328578381687188591572342160574912716102443372011598624
b=0xABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF
q = get_bingint([2623536861626805472, 10424652189184531423, 9920249030899947128, 9196828468227470855])
# print(q)
print((a/b) - q)

r = get_bingint([0])
print(a%b - r)
# def normalize(u, v):
#     """Normalize u and v so v's leading digit is >= floor(b/2)"""
#     d = 1
#     msb = v[-1]
#     while msb < (1 << 31):  # Assuming 32-bit words
#         msb <<= 1
#         d <<= 1
#     if d == 1:
#         return u[:], v[:], d
    
#     un = [(u[i] * d) & ((1 << 32) - 1) for i in range(len(u))]
#     for i in range(len(u) - 1):
#         un[i + 1] += un[i] >> 32
#         un[i] &= (1 << 32) - 1
    
#     vn = [(v[i] * d) & ((1 << 32) - 1) for i in range(len(v))]
#     for i in range(len(v) - 1):
#         vn[i + 1] += vn[i] >> 32
#         vn[i] &= (1 << 32) - 1
    
#     return un, vn, d

# def algorithm_d(u, v):
#     """
#     Knuth's Algorithm D for multi-precision division
#     Input: Arrays u and v representing multi-precision integers
#     Output: Quotient and remainder
#     """
#     n = len(v)
#     m = len(u) - n
    
#     if n <= 0 or v[-1] == 0:
#         raise ValueError("Invalid divisor")
#     if m < 0:
#         return [0], u[:]
    
#     # D1: Normalize
#     u, v, d = normalize(u, v)
    
#     # Initialize quotient array
#     q = [0] * (m + 1)
    
#     # D2, D7: Main loop
#     for j in range(m, -1, -1):
#         # D3: Calculate approximate quotient digit
#         qhat = min(
#             ((u[j + n] << 32) + u[j + n - 1]) // v[n - 1],
#             (1 << 32) - 1
#         )
        
#         # D4: Multiply and subtract
#         while True:
#             # Test if qhat is too large
#             if qhat * v[n - 2] > (
#                 ((u[j + n] << 32) + u[j + n - 1] - qhat * v[n - 1]) << 32
#                 ) + u[j + n - 2]:
#                 qhat -= 1
#                 continue
            
#             # Multiply and subtract
#             k = 0
#             borrow = 0
#             for i in range(n):
#                 p = qhat * v[i]
#                 t = u[i + j] - k - borrow
#                 u[i + j] = t & ((1 << 32) - 1)
#                 k = p >> 32
#                 borrow = (p & ((1 << 32) - 1)) > t
            
#             t = u[j + n] - k - borrow
#             u[j + n] = t & ((1 << 32) - 1)
            
#             # D5: Test remainder
#             if t < 0:
#                 # D6: Add back
#                 qhat -= 1
#                 k = 0
#                 carry = 0
#                 for i in range(n):
#                     t = u[i + j] + v[i] + carry
#                     u[i + j] = t & ((1 << 32) - 1)
#                     carry = t >> 32
#                 u[j + n] += carry
#             else:
#                 break
        
#         q[j] = qhat
    
#     # D8: Unnormalize
#     remainder = []
#     for i in range(n):
#         remainder.append(u[i] // d)
    
#     return q, remainder

# def uint_to_array(n, word_size=32):
#     """Convert a Python integer to an array of word_size-bit words"""
#     result = []
#     mask = (1 << word_size) - 1
#     while n:
#         result.append(n & mask)
#         n >>= word_size
#     return result if result else [0]

# def array_to_uint(arr, word_size=32):
#     """Convert an array of word_size-bit words back to a Python integer"""
#     result = 0
#     for digit in reversed(arr):
#         result = (result << word_size) | digit
#     return result

# def test_algorithm_d():
#     # Test case 1: Simple division
#     print("Test 1: Simple division (12345 ÷ 123)")
#     u = uint_to_array(12345)
#     v = uint_to_array(123)
#     q, r = algorithm_d(u + [0], v)  # Add extra word for u as per algorithm requirement
#     q_val = array_to_uint(q)
#     r_val = array_to_uint(r)
#     print(f"Expected: q = 100, r = 45")
#     print(f"Got:      q = {q_val}, r = {r_val}")
#     assert q_val == 100 and r_val == 45

#     # Test case 2: Large numbers
#     print("\nTest 2: Large number division")
#     dividend = 123456789012345678901234567890
#     divisor = 987654321098765432
#     u = uint_to_array(dividend)
#     v = uint_to_array(divisor)
#     q, r = algorithm_d(u + [0], v)
#     q_val = array_to_uint(q)
#     r_val = array_to_uint(r)
#     expected_q = dividend // divisor
#     expected_r = dividend % divisor
#     print(f"Expected: q = {expected_q}")
#     print(f"          r = {expected_r}")
#     print(f"Got:      q = {q_val}")
#     print(f"          r = {r_val}")
#     assert q_val == expected_q and r_val == expected_r

#     # Test case 3: Division by power of 2
#     print("\nTest 3: Division by power of 2 (2^35 ÷ 2^5)")
#     u = uint_to_array(1 << 35)
#     v = uint_to_array(1 << 5)
#     q, r = algorithm_d(u + [0], v)
#     q_val = array_to_uint(q)
#     r_val = array_to_uint(r)
#     print(f"Expected: q = {1 << 30}, r = 0")
#     print(f"Got:      q = {q_val}, r = {r_val}")
#     assert q_val == (1 << 30) and r_val == 0

#     # Test case 4: Division with remainder
#     print("\nTest 4: Division with remainder (1000000007 ÷ 1000)")
#     u = uint_to_array(1000000007)
#     v = uint_to_array(1000)
#     q, r = algorithm_d(u + [0], v)
#     q_val = array_to_uint(q)
#     r_val = array_to_uint(r)
#     print(f"Expected: q = 1000000, r = 7")
#     print(f"Got:      q = {q_val}, r = {r_val}")
#     assert q_val == 1000000 and r_val == 7

#     # Test case 5: Error cases
#     print("\nTest 5: Error cases")
#     try:
#         algorithm_d([1], [0])
#         assert False, "Should have raised ValueError for division by zero"
#     except ValueError as e:
#         print("Successfully caught division by zero")

#     print("\nAll tests passed! 🎉")

# test_algorithm_d()

def nlz(x):
    """Number of leading zeros in a 32-bit integer."""
    if x == 0:
        return 32
    n = 0
    if x <= 0x0000FFFF:
        n += 16
        x <<= 16
    if x <= 0x00FFFFFF:
        n += 8
        x <<= 8
    if x <= 0x0FFFFFFF:
        n += 4
        x <<= 4
    if x <= 0x3FFFFFFF:
        n += 2
        x <<= 2
    if x <= 0x7FFFFFFF:
        n += 1
    return n

def dumpit(msg, arr):
    """Prints arrays in hexadecimal format."""
    print(msg, " ".join(f"{x:08x}" for x in reversed(arr)))

def divmnu(q, r, u, v, m, n):
    """Performs m-word by n-word division, following Knuth's Algorithm D."""
    b = 2**32
    if m < n or n <= 0 or v[n - 1] == 0:
        return 1  # Invalid parameters
    
    if n == 1:
        # Handle single-digit divisor case
        k = 0
        for j in range(m - 1, -1, -1):
            q[j] = (k * b + u[j]) // v[0]
            k = (k * b + u[j]) - q[j] * v[0]
        if r is not None:
            r[0] = k
        return 0

    s = nlz(v[n - 1])  # Shift amount for normalization
    vn = [(v[i] << s | v[i - 1] >> (32 - s)) & 0xFFFFFFFF for i in range(1, n)] + [(v[0] << s) & 0xFFFFFFFF]
    un = [(u[i] << s | u[i - 1] >> (32 - s)) & 0xFFFFFFFF for i in range(1, m)] + [(u[0] << s) & 0xFFFFFFFF]
    un.append((u[m - 1] >> (32 - s)) & 0xFFFFFFFF)
    
    for j in range(m - n, -1, -1):
        qhat = (un[j + n] * b + un[j + n - 1]) // vn[n - 1]
        rhat = (un[j + n] * b + un[j + n - 1]) - qhat * vn[n - 1]
        while qhat >= b or qhat * vn[n - 2] > b * rhat + un[j + n - 2]:
            qhat -= 1
            rhat += vn[n - 1]
            if rhat >= b:
                break
        
        k = 0
        for i in range(n):
            p = qhat * vn[i]
            t = (un[i + j] - k - (p & 0xFFFFFFFF)) & 0xFFFFFFFF
            un[i + j] = t
            k = (p >> 32) - (t >> 32)
        un[j + n] = (un[j + n] - k) & 0xFFFFFFFF
        q[j] = qhat

        if un[j + n] < 0:
            q[j] -= 1
            k = 0
            for i in range(n):
                t = un[i + j] + vn[i] + k
                un[i + j] = t & 0xFFFFFFFF
                k = t >> 32
            un[j + n] = (un[j + n] + k) & 0xFFFFFFFF

    if r is not None:
        for i in range(n - 1):
            r[i] = (un[i] >> s | un[i + 1] << (32 - s)) & 0xFFFFFFFF
        r[n - 1] = (un[n - 1] >> s) & 0xFFFFFFFF
    return 0

def check(q, r, u, v, m, n, cq, cr):
    """Checks if the computed quotient and remainder match expected values."""
    szq = max(m - n + 1, 1)
    if q[:szq] != cq[:szq] or (r is not None and r[:n] != cr[:n]):
        dumpit("Error, dividend u =", u)
        dumpit("       divisor  v =", v)
        dumpit("For quotient,  got:", q[:szq])
        dumpit("        Should get:", cq[:szq])
        if r is not None:
            dumpit("For remainder, got:", r[:n])
            dumpit("        Should get:", cr[:n])

def main():
    """Runs a series of test cases for the divmnu function."""
    test_cases = [
        # Each tuple represents (m, n, u, v, cq, cr)
        (1, 1, [3], [1], [3], [0]),
        (1, 1, [3], [2], [1], [1]),
        (1, 1, [3], [4], [0], [3]),
        
        # Additional cases can be added here
    ]
    
    errors = 0
    for case in test_cases:
        m, n, u, v, expected_q, expected_r = case
        q = [0] * max(m - n + 1, 1)
        r = [0] * n
        f = divmnu(q, r, u, v, m, n)
        if f:
            print(f"Error: invalid parameters for u={u}, v={v}")
            errors += 1
        else:
            check(q, r, u, v, m, n, expected_q, expected_r)
    print(f"{errors} errors out of {len(test_cases)} cases")

if __name__ == "__main__":
    main()
