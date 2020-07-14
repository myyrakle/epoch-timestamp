# epoch-timestamp

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-1.0.0-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/epoch-timestamp/blob/master/LICENSE) 

Boilerplate collection for epoch(unix) timestamping.

##
You can get the current time as an epoch timestamp as shown below.
```
let now = Epoch::now(); //return u64 
```

##
And if you want to make tomorrow's time a timestamp, you can do this.
```
let tomorrow = now + Epoch::day(1);
```

`Epoch::day` function converts 64 numbers passed as arguments to day units.

###
Other conversion functions are as follows.
```
Epoch::second(2); //2 seconds
Epoch::minute(3); //3 minutes
Epoch::hour(4); //4 hours
Epoch::day(5); //5 days
Epoch::week(6); //6 weeks
Epoch::year(7); //7 years
```