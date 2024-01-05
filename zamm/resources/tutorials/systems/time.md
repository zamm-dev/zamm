# Editing timezone from the commandline

Check your current timezone:

```bash
$ timedatectl
               Local time: Thu 2024-01-04 02:36:31 UTC
           Universal time: Thu 2024-01-04 02:36:31 UTC
                 RTC time: Thu 2024-01-04 02:36:31
                Time zone: Etc/UTC (UTC, +0000)
System clock synchronized: yes
              NTP service: active
          RTC in local TZ: no
```

Follow the instructions [here](https://linuxize.com/post/how-to-set-or-change-timezone-in-linux/) and run

```bash
$ timedatectl list-timezones
```

You can grep for the city if you'd like:

```bash
$ timedatectl list-timezones | grep Melbourne
Australia/Melbourne
```

Then pick from one of them, and set it as the new timezone as such:

```bash
$ sudo timedatectl set-timezone Australia/Melbourne
```

Confirm that it's different now:

```bash
$ timedatectl                                
               Local time: Thu 2024-01-04 13:37:06 AEDT
           Universal time: Thu 2024-01-04 02:37:06 UTC
                 RTC time: Thu 2024-01-04 02:37:06
                Time zone: Australia/Melbourne (AEDT, +1100)
System clock synchronized: yes
              NTP service: active
          RTC in local TZ: no
```
