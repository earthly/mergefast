## Idea

- use rust and py03 for mergefast
- then add galopping to make even faster 

## Problems

- can't use specilized compares for python types like int and string
- even without those, and timsort without those, we are still half the speed
- adding galooping seems to not make a difference at all 

## Investigate

- what is the slow part?
 - Can't tell, because can't profile. 
  - Dtrace just giving memory locations, need to try profiling on linux.
