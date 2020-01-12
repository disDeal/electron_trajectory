unset key
set xyplane relative 0

unset box

set view map

set size ratio -1

unset border
unset xtics
unset ytics

set terminal pngcairo size 2460,3696 backgr rgb "black"
set output "tmp.png"

yshift=-15.0
maxiC=29
maxiQ=29
splot \
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 30.0 lc rgb "#030d19" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 18.0 lc rgb "#071b33" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 17.0 lc rgb "#0a294c" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 16.0 lc rgb "#0e3766" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 15.0 lc rgb "#11457f" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 14.0 lc rgb "#155399" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 13.0 lc rgb "#1861b2" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 12.0 lc rgb "#1c6fcc" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 11.0 lc rgb "#1f7de5" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 10.0 lc rgb "#238bff" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 9.0 lc rgb "#3896ff" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 8. lc rgb "#4ea2ff" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 7. lc rgb "#65adff" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 6. lc rgb "#7bb9ff" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 5. lc rgb "#91c5ff" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 4. lc rgb "#a7d0ff" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 3. lc rgb "#bddcff" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 2. lc rgb "#d3e7ff" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 1. lc rgb "#e9f3ff" not,\
for [i=0:maxiC] sprintf("cmd_%05i.trj", i) w l lw 0.5 lc rgb "#ffffff" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 30.0 lc rgb "#190613" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 18.0 lc rgb "#330c27" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 17.0 lc rgb "#4c123b" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 16.0 lc rgb "#66184f" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 15.0 lc rgb "#7f1e63" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 14.0 lc rgb "#992476" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 13.0 lc rgb "#b22a8a" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 12.0 lc rgb "#cc309e" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 11.0 lc rgb "#e536b2" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 10.0 lc rgb "#ff3dc6" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 9.0 lc rgb "#ff50cb" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 8. lc rgb "#ff63d1" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 7. lc rgb "#ff77d7" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 6. lc rgb "#ff8adc" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 5. lc rgb "#ff9ee2" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 4. lc rgb "#ffb1e8" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 3. lc rgb "#ffc4ed" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 2. lc rgb "#ffd8f3" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 1. lc rgb "#ffebf9" not,\
for [i=0:maxiQ] sprintf("bmd_%05i.trj", i) u 1:($2+yshift):3 w l lw 0.5 lc rgb "#ffffff" not
