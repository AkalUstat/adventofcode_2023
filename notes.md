## Day 5 Part 2

Some reddit comments have helpfully explained the concept which I applied here:

[Comment
One](https://www.reddit.com/r/adventofcode/comments/18bimer/comment/kc52zpl/?utm_source=share&utm_medium=web2x&context=3)
> Basically, instead of working with every single digit imaginable, you use buckets.    
> Seeds has buckets : "5-12" and "17-24".    
 > Seed-to-Soil has buckets : "10 to 15, moves to 24 to 29", "24 to 29, moves to 10 to 15".    
> This leads to the new bucketing :    
> 5-9 -> 5-9    
> 10-12 -> 24-26    
> 17-23 -> 17-23    
> 24-24 -> 10-10    
> Alternatively, you could say that you initially had the seeds 5, 12, 17, and 24; and that the first conversion went (* -> denotes a new entry) :    
> 5 -> 5    
> \* -> 9    
> \* -> 24    
> 12 -> 26    
> 17 -> 17    
> \* -> 23    
> 24 -> 10    
> So TL;DR :    
> Only care about the boundaries of each equivalence class, not the content; and    
> You can safely ignore every conversion that doesn't apply to your seeds.    

[Comment Two](https://www.reddit.com/r/adventofcode/comments/18bimer/comment/kc5ae35/?utm_source=share&utm_medium=web2x&context=3)
> I'm seeing lots of complicated solutions being given to you, and none of them are as simple as my solution which runs in 2ms  
> Most brute solutions look like this:  
> for(i = seed; i < seed + range; i++) result = min(result, f(seed+i))  
> where f() calculates the final value for the input  
> Ok so imagine your seed is the number x. You feed it in and it spits out a value, n  
> Now, imagine you put in x+1. What does it spit out? Most likely, it will spit out n+1  
> x+2 will likely output n+2  
> x+3 will likely output n+3  
> x+4 will likely output n+4  
> As long as this pattern continues, the output will be larger than the output for x. So, we can discard any number that comes after x for as long as the pattern continues.  
> How long does this pattern continue for? Well, if your INPUT number falls within a range (DESTINATION, SOURCE, RANGE), then the pattern will continue for RANGE-(INPUT-SOURCE). We can call this number STOP  
> If your INPUT doesn't fall within a range STOP is (SOURCE OF NEXT HIGHEST RANGE) - INPUT  
> And if your INPUT is bigger than any range, the STOP is INFINITY  
> So, what do you do with this information?  
> When calculating the output for a given seed, also calculate, for each map, the STOP value. The final STOP output is the min() of all STOP values you calculated for each level of the map. Now, with your brute force for loop above, you can say  
> for(i = seed; i < seed + range; i+=STOP)  
> When calculating STOP, you can verify it's the correct value because the output for STOP-1 will be significantly different from STOP, while the output for STOP+1 will usually be the same as (OUTPUT for STOP) + 1, same goes fro STOP-2 and STOP-1  
> If you feel like unpacking my golfed solution for part 2, I use this method.  
> So what's the runtime of this? If the absolute worst case would be something like sizeof(seeds)*sizeof(map 1) * sizeof(map 2) * sizeof(map 3) ... * sizeof(map n), which is still only a couple of seconds. Actual runtime is more like n * seeds * 2  
> The worst case is very unlikely though  

These is from [this](https://www.reddit.com/r/adventofcode/comments/18bimer/comment/kc4y6io/?utm_source=share&utm_medium=web2x&context=3) comment thread, which is worth a read.

Here is [another thread](https://www.reddit.com/r/adventofcode/comments/18buwiz/2023_day_5_part_2_can_someone_explain_a_more/?utm_source=share&utm_medium=web2x&context=3) worth a read