# Connecting to the wifi from the commandline

List networks:

```bash
$ nmcli d wifi list
IN-USE  BSSID              SSID                           MODE   CHAN  RATE        SIGNA>
*       FA:E2:62:4E:3D:C4  Melbourne Free Wifi            Infra  5     130 Mbit/s  100  >
        98:42:65:7D:AC:5E  Optus_7DAC5C                   Infra  11    195 Mbit/s  87   >
        FA:8F:CA:36:8C:C3  Family Room speaker.o,         Infra  6     65 Mbit/s   75   >
        94:83:C4:0A:31:A2  Use Protection                 Infra  5     270 Mbit/s  74   >
        98:42:65:7D:AC:5F  Optus_7DAC5C_5GHz              Infra  149   540 Mbit/s  72   >
        9C:53:22:13:B1:C6  WiFi-13B1C6                    Infra  7     270 Mbit/s  60   >
        BC:30:D9:EE:71:C0  TelstraEE71BE                  Infra  2     405 Mbit/s  52   >
        78:98:E8:9C:6D:F0  dlink-9C6DF0                   Infra  8     405 Mbit/s  45   >
        D4:35:1D:1E:AB:81  chilli                         Infra  1     130 Mbit/s  44   >
        B0:A7:B9:8F:9F:95  Whyfi                          Infra  3     270 Mbit/s  40   >
        9C:53:22:13:B1:C8  WiFi-13B1C6                    Infra  161   270 Mbit/s  40   >
        10:06:45:A3:D7:EF  Optus_A3D7ED                   Infra  11    195 Mbit/s  29   >
        44:A5:6E:87:87:89  Telstra3E94                    Infra  11    195 Mbit/s  27   >
        B8:8D:12:62:DE:2F  Oscar's Wi-Fi Network          Infra  11    195 Mbit/s  25   >
        72:A5:6E:87:87:8A  AL_Guest                       Infra  11    195 Mbit/s  25   >
        86:69:93:27:A3:DC  DIRECT-DC-HP ENVY 6000 series  Infra  11    65 Mbit/s   25   >
        78:98:E8:9C:6D:F1  dlink-9C6DF0                   Infra  153   540 Mbit/s  24   >
        BC:30:D9:EE:71:BF  TelstraEE71BE                  Infra  100   540 Mbit/s  17   >
        72:30:D9:EE:71:B9  --                             Infra  100   540 Mbit/s  17   >
        D6:92:5E:20:DC:25  Telstra20DC25                  Infra  132   540 Mbit/s  14   >
        72:92:5E:20:DC:26  --                             Infra  132   540 Mbit/s  14   >
        B8:8D:12:62:DE:30  Oscar's Wi-Fi Network          Infra  36    405 Mbit/s  7    >
        78:A0:51:15:8E:9B  IINET-158E9B                   Infra  60    405 Mbit/s  7    
```

Then

```bash
$ nmcli c up "Melbourne Free Wifi"
Connection successfully activated (D-Bus active path: /org/freedesktop/NetworkManager/ActiveConnection/20)
```
