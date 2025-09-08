
### how will it work

in the first version i work towards, all its going to do is print a value to the terminal at an interval, giving a realistic value and a unit. 
for example, if the sensor simulator is started giving a temperature parameter, then the a temp value in C will be printed  to the terminal at a sensible interval until the user terminates the program.

```
> sensim temperature
12:00:00.000    28.23C
12:01:00.000    26.91C
12:02:00.000    26.30C
12:03:00.000    25.85C
```

then, options could be provided for what interval data is generated at (definitly useful) and how long data should be generated for until the program terminates itself (probably useful)

```
> sensim temperature --interval 300 --duration 1hr
12:00:00.000    28.23C
12:05:00.000    26.91C
12:10:00.000    26.30C
12:15:00.000    25.85C
12:20:00.000    25.11C
...
12:55:00.000    22.20C
13:00:00.000    22.19C
```

in the early version, its probably going to be easiest to do this in real time - meaning using the system time for the timestamp and printing at that time. so if the interval is set at 3 minutes and the duration is 30 minutes, then the program will be printing to the terminal for 30 minutes and waiting in between. but in a future version, it would probably be useful to simulate this in a shorter time span i.e. set a rate so that one hour is simulated as one minute (so speeding up testing in event driven settings by and order of magnitude). Not sure how to implement this - maybe the subcommand would be --rate or --velocity or something. 


im really not sure about `sensim` as the command name. maybe just sn? or sm?


### cli structure

what arguements do I think need to be able to be parsed from the command line?

- what type of sensor
- what interval data will be generated at
- how long data will be generated for
- total duration data should be generated for
- how fast data should be produced
- write data to terminal or to file, and in what format (json, csv, datbase)
- historical/future periods i.e. set a time period start and end
- option to add a number of readings instead of a duration and sample interval (e.g. -n or --number 1000). This should be set up so that only two of duration, interval and number can be set, with the third one inferred.
- units
- maybe some actual sensors (brands, models) that can be simulated
- for file outputs, destination path

### thoughts
- for a "realistic" temperature, we need to either have configurations for start/average value, variation, trend, ect. So for example, by default it could be room temperature that is centred around 25 degrees and has a minor flucuations with no overall upwards or downwards trends. Then, there can be ways to add day/night cycles (The center shifts from 25 to 15 and back over 24hrs). Or, we can have preset trends like exponential cooling rates (a 1000 degree object in a room fixed at 25 degrees will cool slower as object temp approaches environment temp). there has to be a good balance between useful defaults and flexibility to create scenarios. for more realistic scenarios, a json (or yaml?) config file will probably be preferable over just cli subcommands, so probably not worth worrying about for now.
- i should research some actual sensors and write out how they record and present data. For example, things in the house such as the environment control in dads cellar, the oven thermostat, the house thermostat, fridge thermostat, the motion light in the kitchen. Also, some more intricate systems with mulitple sensors such as the egg incubator, a combined pressure/temp gauge. Maybe an espresso machine
- 

### trends, signal, noise, seasons

- the base value should not be totally random - it should probably be autocorrelated. That is, the next value is more likely to be close to the last value than far from it. We can do this by using the last value and modifying it with a value taken from a normal distribution. 
- one step beyond this would be a value that can vary randomly, but trends to some average over time. So you can set a mean temperature over the duration, and the value will vary from one value to the next, but when the previous value is above the mean then it is more likely to decrease than increase. And also the further above the "base value" the current value is, the more likely it is to increase. Not sure how to do this - maybe combining (convolving, adding, multiplying?) curves centered arount the current value and the base value. Or is there a way of applying a skewing affect to a normal curve using the distance from the mean to the base value?
- standard deviation can be set as struct field for the sensor - this means it can be configured when its build, allowing configuration of how noisy the readings of a sensor will be by the user at some point.
- the standard deviation (and therefore the amount of change from one reading to the next) should probably be proportional to the interval in som e way. That way, there is going to be a similar pattern drawn through data sampled every 2 seconds for five minutes and every 20 seconds for five minutes, rather than the shorter interval sensor looking noisier because the same modifyer is applied more often and ten times more overall. 
- maybe there should be different standard deviation defaults for different sensor types too - probably atmospheric pressure is much more stable than air temperature? 
- We want to be able to replicate a few different trends - maybe do this by applying a trend just to the base value or mean value, allowing the noise to be seperate. 
    - linear decrease/increase
    - exponential decay (useful for cooling curves)
    - step changes?
    - day/night cycles (combining different heating and cooling rates in a cycle)
    - weather fronts?
    - mixes like a steady state, then at a defined point and exponential decay, then oscillation until a new steady state is reached at a lower temperature/pressure/whatever. like a curve from a PID controlled climate.


### csv output

there is a commented-out function for EnvironmentalSensor, append_to_file. I actually started writing this first before some reading where i decided to use the csv and serde crates. This meant that I thought the approach would be to format each reading and append it to a plain text file, then simply name it with a .csv extension. While i discovered that serde would allow me to serialize the entire outputs field in one go for a faster implementation of writing to csv, I can still think of some situations where appending to the file while the sensor is still running would be useful: 
    1. in a sensor that is running for a long time (e.g. days), appending to a file would allow the outputs vector to be capped at a maximum size without historical data loss - the csv would act as an archive of sorts. 
    2. the csv could be source data that changes in real time. An application could be that its a seed for a dbt model that refreshes on a schedule. While not a production ready solution, this could be nice for prototyping 
    3. allows sensors to be run "indefinitly", and still have access to files. This means that partitioning the files once they reach a maximum number of lines is probably also necessary. 
Im kinda imagining this as a precursor to putting the program in a docker image and building an api to recieve readings. So you could have it running and just read the csv intermitently to get near live data for a pipeline/reporting application. 

However, now i thought about it, using a logging system with the in memory vector buffering recent readings but partitioned files used to store readings long term on disk solves a issue in the current design. If set to run for a long time, then the program would crash when it runs out of memory by filling up the outputs vector. In real life lots of sensors are expected to run for months or years reliably - although i would never do that with this project, it would be nice to be able to replicate a production-ready approach. 

so next steps:  

    1. how many rows per file?
    2. is number of rows the best way to partition logs? maybe a timestamp would be better. With an interval of 1 second, there would be 86400 rows per day, which could be a sensible size for a single file. however, its a very silly partition if the interval is one hour or one day. total rows or file size in mb therefore seems a better way to partition files. otherwise i might have to come up with a complex way to dynamically calculate a partition timeframe when the program starts based on the timing args.
    3. should we append one row every time a one is generated, or in batches? should the batches be the whole file, or chunks?
    4. should the entire in memery vector be cleared once appended to the file?
    5. how can we handle errors while writing to file? we can keep the readings in the vector until the file is correctly saved, then clear them. How do I check if a file has been corrupted?
        - we have to create an atomic process for opening, appending to, and closing/saving a file 
        - this has to happen in a way it can fail and be repeated without loosing any in memory data or the previous state of the log file
        - ideally, this should happen asynchronously so that it doesn't block the thread and interfere with the interval of the sensor. Realistically, theres no way im doing this in this project. 

an atomic transaction:
    1. copy the existing log file.
    2. serialize readings in the vector and append them to the csv
    3. check the csv is safely saved
    4. clear data from the vector
    5. clean up the copy of the file

If steps 2 or 3 fail, the copy of the file can be used to recover the existing state before the transaction began. Errors in step 1 or 5 should be handled gracefully, but an error in step four should probably crash the program, either immediately or after three retries maybe.

To start with, im not going to worry about the most appropriate way to partition logs. In fact, to make sure its working while i develop i will probably go with a really low row count. 


some problems in the first approach:

- currently, the headers are being appended every time instead of only the first time
- the same file is appended to if the process is run twice. This could be exactly the desired behaviour in some cases (stop and restart the sensor, pick up where you left off with log files). But this was actually not what i was expecting this time. I think there are two option:
    1. The first time, if there already exists a file then either delete it first or open in write mode not append mode. 
    2. each run should have a unique id prefix for the output file. this could be the sensor id - we then need a sensible unique code for the sensor to be generated at runtime. This would also allow the same sensor to be stopped and started and reuse the existing logs bc the id could be set with a config value


im using a counter variable to track when to append a batch of readings to the file. This seems really clunky, but i thought that computing the length of the vector directly each loop would be expensive. Im also not sure what the implications of adding an if statement to every loop are. Since we are already doing a timestamp comparison on each loop, would a time based partition actually make more sense from an efficiency point of view? 