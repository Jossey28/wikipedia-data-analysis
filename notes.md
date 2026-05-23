# Notes

### Things to keep in mind


***Let MySQL handle filtering, sorting, and limiting***

*Only pull into rust what is necessary to complete the objective*

Since the database size is large (~200GB), it's important to optimize the code whenever allowed for quicker enumeration. Because I'm running a seperate MySQL server caching allows for quick data fetching, however, the overhead of transfering the data into the program will still result in significant overhead.  

