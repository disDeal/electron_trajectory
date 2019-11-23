from math import *
import numpy as np

cutoff = 5.e-4
Quantum = True


def dist(r1, r2):
    return np.dot((r1-r2), (r1-r2))


def Vc(r, r0):
    if dist(r, r0) >= cutoff:
        return -1./dist(r, r0)
    else:
        return -1./cutoff


rH = []
# h1
# rH.append(np.array([0.,0.,0.]))

# h2
rH.append(np.array([-1., 0., 0.]))
rH.append(np.array([+1., 0., 0.]))


def Vat(r):
    V = 0.
    for rh in rH:
        V += Vc(r, rh)
    return V


def PsiA(r):
    psi = 0.0
    for rh in rH:
        if dict(r, rh) <= 1.e3:
            psi += np.exp(-dist(r, rh))
    return psi


def Vq(r):
    vq = 0.
    for rh in rH:
        if dist(r, rh) >= cutoff:
            dist = dist(r, rh)
            vq -= 2.*np.exp(-dist)/dist
        else:
            vq -= 2.*np.exp(-cutoff)/cutoff
    vq *= (-0.5)  # -0.5*hbar/me
    return vq


def GradF(F, r):
    grad = np.zeros(3)
    dx = 0.1
    for i in range(0, 3):
        dr = np.zeros(3)
        dr[i] = dx
        grad[i] += (F(r+dr) - F(r-dr))/(2.*dx)
    return grad


dt = 0.001
tmax = 2.e1
t = 0.0

DR = 1.0
dx = 0.001

MaxR = 10.0

cent = np.zeros(3)

Ntrj = 30
m = 1.0


def GenRvBox(DX):
    return np.random.uniform(-DX, DX, 3)


def GenRvSph(DX):
    r = np.random.uniform(0.0, DX)
    phi = np.random.uniform(0.0, 2.0*np.pi)
    theta = np.random.uniform(0.0, np.pi)

    x = r*np.sin(theta)*np.cos(phi)
    y = r*np.sin(theta)*np.sin(phi)
    z = r*np.cos(theta)
    return np.array([x, y, z])


for ntrj in range(0, Ntrj):
    if Quantum:
        outf = open("bmd_%05i" % (ntrj) + ".trj", "w")
    else:
        outf = open("cmd_%05i" % (ntrj) + ".trj", "w")

    nat = np.random.randint(0, len(rH))
    r = rH[nat] + GenRvSph(DR)
    rprev = r + GenRvBox(dx)

    outf.write("%15.10f %15.10f %15.10f\n" % tuple(r))
    t = 0.0
    while t <= tmax and dist(r, cent) <= MaxR:
        F = -GradF(Vat, r)
        if Quantum:
            f -= GradF(Vq, r)
        rnew = 2.0*r - rprev + (F/m)*dt**2
        rprev = r
        r = rnew
        outf.write("%15.10f %15.10f %15.10f\n" % tuple(r))

        t += dt
    outf.close()

exit()
