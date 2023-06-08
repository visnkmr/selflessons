first try to identify line number of issue.
read the code and try to figure any anomalies.
if the issue is within a rayon like parallel iterator try to remove the parallel iterator and use normal iterator to check if issue is with an individual case within the iterator. 
now write a test case to check for dummy values.
now alternate dummy and real value.
use breakpoints to understand where the ambiguity occurs if none of the above works or if you are working with an old/foreign code base.