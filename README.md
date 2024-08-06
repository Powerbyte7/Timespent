# Timespent

This is a simple commandline tool I made myself to determine the number of hours spent on projects using file metadata. You specify a path, and an inactivity threshold:

```sh
./timespent.exe C:\Users\John\Documents\CoolProject 2
```

```
Using threshold of 2 hours
Looking through files in C:\Users\John\Documents\CoolProject
Hours spent according to files: 13.67
```

## How it works

1. Collect all of the timestamps in a directory
2. Sort the timestamps chronologically
3. Compare chronologically adjectent timestamps. If timestamps are further apart than the inactivity threshold, assume no work was done during that time. If they're closer, this timespan is a valid duration.
4. Add all valid durations together to get the total time spent.