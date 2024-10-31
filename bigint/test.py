def get_bingint(arr):
    sum = 0
    for i in range(len(arr)):
        sum += arr[i]*(2**(64*i))
    return sum


# test_base_case_mult
a =0X7FA1B2C3D4E5F60789ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE0
b =0xABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF
print(a+b)
b_calc = get_bingint( [7542649922918342911, 7542649922918342692, 12379813817064611620, 12379813812177893520])
print("b result",b_calc - b)


# print(a>b)
sub_res = [13527631012418014177, 2882002266266188730, 15987179287544887124, 15263758729759128950]
result = get_bingint(sub_res)
print(b-a)
print(result)

arr = [17106816733836489504, 8437436160694434591, 17102275007484288737, 17291968141207752297, 6871473844393097586, 16721514386755580715, 8596134145241391739, 6172093223836742082]
res = get_bingint(arr)
print(res)
print("mult",a*b);
print(a*b - res) # True


# karatsuba
a0 = a % (2**(64*2))
a0_calc = get_bingint([2623536861626805472, 10424652189184531423])
print(a0 - a0_calc) #0 

b0 = b % (2**(64*2))
b0_calc = get_bingint([7542649922918342911, 7542649922918342692] )
print(b0 - b0_calc) #0

c0 = a0*b0
c0_calc =  get_bingint([17106816733836489504, 8437436160694434591, 2696075392888458932, 4262513846184196733])
print(c0 - c0_calc)

a1 = a // (2**(64*2))
a1_calc = get_bingint([9920249030899947128, 9196828468227470855] )
print(a1 - a1_calc) #0
b1 = b // (2**(64*2))
b1_calc = get_bingint([12379813817064611620, 12379813812177893520])
print(b1 - b1_calc) #0

c1 = a1*b1
c1_calc = get_bingint( [10389936785480659168, 7192767197262303206, 8596134145241391739, 6172093223836742082] )
print(c1 - c1_calc) #0

diff = abs(a0 - a1)
diff_1 = get_bingint( [11150031904436409960, 1227823720957060567] )
print(diff - diff_1) #0

diff2 = abs(b0 - b1)
diff2_calc = get_bingint([4837163894146266981, 4837163889259550828] )
print(diff2) #0
print(diff2 -diff2_calc) #0



## ODD EVEN KARASTUBA
print("ODD EVEN KARASTUBA")
a = 249166486039954038678241653028365438597414117400323598999
b = 4062882539673212545835653973091204775
result_arr=  [1635125010864364161, 11949138821930240557, 16131851141360144773, 11615463313061491774, 8742688488232360] 
result = get_bingint(result_arr)
print(result)
print(a*b)
