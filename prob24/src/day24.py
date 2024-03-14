import numpy as np
from dataclasses import dataclass
import sympy as sp

@dataclass
class Hail:
    px: int
    py: int
    pz: int
    vx: int
    vy: int
    vz: int


    def __init__(self, line: str):
        # 18, 19, 22 @ -1, -1, -2
        parts = line.split(" @ ")
        pp1 = parts[0].split(", ")
        self.px = int(pp1[0])
        self.py = int(pp1[1])
        self.pz = int(pp1[2])
        pp2 = parts[1].split(", ")
        self.vx = int(pp2[0])
        self.vy = int(pp2[1])
        self.vz = int(pp2[2])

    def intersect(self, other: 'Hail'):
        a = np.array([[self.vx, -other.vx], [self.vy, -other.vy]])
        b = np.array([other.px - self.px, other.py - self.py])
        try:
            t = np.linalg.solve(a, b)
        except Exception: 
            return None
        else:
            if t[0] > 0 and t[1] > 0:
                return (self.px + t[0] * self.vx, self.py + t[0] * self.vy)
            else:
                return None
        

def main():
    # low = 7
    # high = 27
    # file = 'inputs/test24.txt'
    low = 200000000000000
    high = 400000000000000
    file = 'inputs/input24.txt'
    hails = []
    for line in open(file):
        hails.append(Hail(line))
    part1(hails, low, high)
    part2(hails)

def part1(hails, low, high):
    ans = 0
    for i in range(len(hails) - 1):
        for j in range(i, len(hails)):
            inter = hails[i].intersect(hails[j])
            if inter and low <= inter[0] <= high and low <= inter[1] <= high:
                ans += 1
    print(ans)

def part2(hails):
    PX, PY, PZ = sp.symbols('PX,PY,PZ', positive=True, integer=True)
    VX, VY, VZ = sp.symbols('VX,VY,VZ')

    system = []

    for hail in hails:
        system.append(sp.Eq((hail.px - PX) * (VY - hail.vy), (hail.py - PY) * (VX - hail.vx)))
        system.append(sp.Eq((hail.px - PX) * (VZ - hail.vz), (hail.pz - PZ) * (VX - hail.vx)))
    solution = sp.solve(system, [PX, PY, PZ, VX, VY, VZ])
    print(solution)
    px, py, pz, *_ = solution[0]
    print(px + py + pz)

if __name__ == '__main__':
    main()