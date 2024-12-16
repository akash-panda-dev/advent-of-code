import re
from sympy import Eq, symbols, solve


if __name__=="__main__":

    with open("2023/day24/input.txt") as f:
        hailstones = [list(map(int, re.findall(r'-?\d+', line))) for line in f.readlines()]

    rx, ry, rz, rvx, rvz, rvy = symbols('rx ry rz rvx rvz rvy')
    times = [symbols(f't{i}') for i in range(3)]
    
    equations = []
    for i, (x,y,z,vx,vy,vz) in enumerate(hailstones[:3]):
        equations.append(Eq(x + times[i]*vx, rx + times[i]*rvx))
        equations.append(Eq(y + times[i]*vy, ry + times[i]*rvy))
        equations.append(Eq(z + times[i]*vz, rz + times[i]*rvz))

    sol = solve(equations, (rx, ry, rz, rvx, rvy, rvz) + tuple(times))
    print(sol)
    print("Sum of coordinates: ", abs(sol[0][0]) + abs(sol[0][1]) + abs(sol[0][2]))
        

    