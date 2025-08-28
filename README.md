# About

Date/time CLI utility ([dtg]) and library ([dtg-lib])

[dtg]: https://crates.io/crates/dtg
[dtg-lib]: https://crates.io/crates/dtg-lib

# CLI

## Usage

~~~text
$ dtg -h
Date/time CLI utility

<https://crates.io/crates/dtg> / <https://github.com/qtfkwk/dtg>

---

Usage: dtg [OPTIONS] [ARG]...

Arguments:
  [ARG]...  Argument [-X: "x" timestamp (2), -Z: timezone, "%s.%f" timestamp,
            default: now]

Options:
  -l                  Local timezone (6)
  -a                  "a" format (1)
  -x                  "x" format (2)
  -X                  Give timestamp argument(s) in "x" format (2)
  -Z                  Search/list timezones
  -f <FORMAT>         Format(s) [-z/-l: "%a %d %b %Y %H:%M:%S %Z",
                      "%Y-%m-%dT%H:%M:%SZ"]
  -z <ZONE>           Timezone(s) [default: UTC] (3) (6)
  -s <SEPARATOR>      Separator [default: "\n"]
  -n <NAME>           Named format(s) [all, bcd, compact-date (%Y%m%d),
                      compact-date-time (%Y%m%d-%H%M%S), compact-time (%H%M%S),
                      default, rfc-3339, x, or any custom format] (4) (5)
  -i <N>              Run every N seconds
  -c <N>              Clear and run every N seconds
  -r, --readme        Print the readme
  -h, --help          Print help
  -V, --version       Print version

---

Notes:

1. "a" format:

   ```text
   %s.%f
   %Y-%m-%dT%H:%M:%SZ
   %a %d %b %Y %H:%M:%S %Z # UTC
   %a %d %b %Y %H:%M:%S %Z # Specified or local timezone
   ```

2. "x" format (novel UTC / base 60 encoding):

   ```text
   0* 0 1 2 3 4 5 6 7 8 9
   1* A B C D E F G H I J
   2* K L M N O P Q R S T
   3* U V W X Y Z a b c d
   4* e f g h i j k l m n
   5* o p q r s t u v w x
   ```

   Field  | Values           | Result
   -------|------------------|----------
   Year   | 2020 => 33*60+40 | Xe
   Month  | Jan-Dec => 0-11  | 0-B
   Day    | 0-27/28/29/30    | 0-R/S/T/U
   Hour   | 0-23             | 0-N
   Minute | 0-59             | 0-x
   Second | 0-59             | 0-x

3. Prints the timestamp in each format with one or more timezones using a
   comma-separated string (`-z UTC,EST`).

4. The `-f`, `-a`, and `-x` options are processed *in that order* and do not
   enable any reordering, however the `-n` option processes its arguments in the
   order given and handles custom, "a", "x", and named formats.

5. "bcd" format: year, month, day, hour, minute, and second displayed like a
   binary clock with the Braille Patterns Unicode Block and `|` separators.

6. `-l` / `-z` are ignored when processing UTC-only formats like `-n rfc-3339`.
~~~

## CLI examples

Get current date/time in UTC and RFC 3339 format:

```text
$ dtg
2025-08-28T11:56:20Z
```

Get current date/time in the local timezone and default format:

```text
$ dtg -l
Thu 28 Aug 2025 07:56:20 EDT
```

Get current date/time in explicit timezone and default format:

```text
$ dtg -z MST
Thu 28 Aug 2025 04:56:20 MST
```

```text
$ dtg -z America/Los_Angeles
Thu 28 Aug 2025 04:56:20 PDT
```

Get current date/time in UTC and specific format:

```text
$ dtg -f %A
Thursday
```

```text
$ dtg -f %s.%f
1756382180.95791612
```

Get current date/time in local timezone and specific format:

```text
$ dtg -l -f %A
Thursday
```

Get current date/time in explicit timezone and specific format:

```text
$ dtg -z MST -f %A
Thursday
```

Get current date/time in "a" format:

```text
$ dtg -a
1756382180.115094611
2025-08-28T11:56:20Z
Thu 28 Aug 2025 11:56:20 UTC
Thu 28 Aug 2025 07:56:20 EDT
```

Get current date/time in explicit timezone and "a" format:

```text
$ dtg -a -z MST
1756382180.121655544
2025-08-28T11:56:20Z
Thu 28 Aug 2025 11:56:20 UTC
Thu 28 Aug 2025 04:56:20 MST
```

Get current date/time in "x" format:

```text
$ dtg -x
Xj7RBuK
```

Get a specific date / time in UTC and RFC 3339 format:

```text
$ dtg 1606447276.941324100
2020-11-27T03:21:16Z
```

Get a specific date/time in the local timezone and default format:

```text
$ dtg -l 1606447276.941324100
2020-11-27T03:21:16Z
```

Get a specific date/time in explicit timezone and default format:

```text
$ dtg -z MST 1606447276.941324100
Thu 26 Nov 2020 20:21:16 MST
```

```text
$ dtg -z America/Los_Angeles 1606447276.941324100
Thu 26 Nov 2020 19:21:16 PST
```

Get specific date/time in UTC and specific format:

```text
$ dtg -f %A 1606447276.941324100
Friday
```

Get specific date/time in local timezone and specific format:

```text
$ dtg -l -f %A -- 1606447276.941324100
Thursday
```

Get specific date/time in explicit timezone and specific format:

```text
$ dtg -z MST -f %A -- 1606447276.941324100
Thursday
```

Get a specific date/time in "a" format:

```text
$ dtg -a 1606447276.941324100
1606447276.941324100
2020-11-27T03:21:16Z
Fri 27 Nov 2020 03:21:16 UTC
Thu 26 Nov 2020 22:21:16 EST
```

Get a specific date/time in explicit timezone and "a" format:

```text
$ dtg -a -z MST 1606447276.941324100
1606447276.941324100
2020-11-27T03:21:16Z
Fri 27 Nov 2020 03:21:16 UTC
Thu 26 Nov 2020 20:21:16 MST
```

Get a specific date/time in "x" format:

```text
$ dtg -x 1606447276.941324100
XeAQ3LG
```

Get a specific date / time from "x" format in RFC 3339 format:

```text
$ dtg -X XeAQ3LG
2020-11-27T03:21:16Z
```

Get a specific date / time from "x" format in the local timezone and default format:

```text
$ dtg -X -l XeAQ3LG
Thu 26 Nov 2020 22:21:16 EST
```

Get a specific date / time from "x" format in explicit timezone and default format:

```text
$ dtg -X -z MST XeAQ3LG
Thu 26 Nov 2020 20:21:16 MST
```

Get a specific date / time from "x" format in specific format:

```text
$ dtg -f '%a %d %b %Y %H:%M:%S %Z' -X XeAQ3LG
Fri 27 Nov 2020 03:21:16 UTC
```

Get a specific date / time from "x" format in local timezone and specific
format:

```text
$ dtg -l -f '%a %d %b %Y %H:%M:%S %Z' -X XeAQ3LG
Thu 26 Nov 2020 22:21:16 EST
```

Get a specific date / time from "x" format in explicit timezone and specific
format:

```text
$ dtg -z MST -f '%a %d %b %Y %H:%M:%S %Z' -X XeAQ3LG
Thu 26 Nov 2020 20:21:16 MST
```

List available time zones:

```text
$ dtg -Z
Africa/Abidjan
Africa/Accra
Africa/Addis_Ababa
Africa/Algiers
Africa/Asmara
Africa/Asmera
Africa/Bamako
Africa/Bangui
Africa/Banjul
Africa/Bissau
Africa/Blantyre
Africa/Brazzaville
Africa/Bujumbura
Africa/Cairo
Africa/Casablanca
Africa/Ceuta
Africa/Conakry
Africa/Dakar
Africa/Dar_es_Salaam
Africa/Djibouti
Africa/Douala
Africa/El_Aaiun
Africa/Freetown
Africa/Gaborone
Africa/Harare
Africa/Johannesburg
Africa/Juba
Africa/Kampala
Africa/Khartoum
Africa/Kigali
Africa/Kinshasa
Africa/Lagos
Africa/Libreville
Africa/Lome
Africa/Luanda
Africa/Lubumbashi
Africa/Lusaka
Africa/Malabo
Africa/Maputo
Africa/Maseru
Africa/Mbabane
Africa/Mogadishu
Africa/Monrovia
Africa/Nairobi
Africa/Ndjamena
Africa/Niamey
Africa/Nouakchott
Africa/Ouagadougou
Africa/Porto-Novo
Africa/Sao_Tome
Africa/Timbuktu
Africa/Tripoli
Africa/Tunis
Africa/Windhoek
America/Adak
America/Anchorage
America/Anguilla
America/Antigua
America/Araguaina
America/Argentina/Buenos_Aires
America/Argentina/Catamarca
America/Argentina/ComodRivadavia
America/Argentina/Cordoba
America/Argentina/Jujuy
America/Argentina/La_Rioja
America/Argentina/Mendoza
America/Argentina/Rio_Gallegos
America/Argentina/Salta
America/Argentina/San_Juan
America/Argentina/San_Luis
America/Argentina/Tucuman
America/Argentina/Ushuaia
America/Aruba
America/Asuncion
America/Atikokan
America/Atka
America/Bahia
America/Bahia_Banderas
America/Barbados
America/Belem
America/Belize
America/Blanc-Sablon
America/Boa_Vista
America/Bogota
America/Boise
America/Buenos_Aires
America/Cambridge_Bay
America/Campo_Grande
America/Cancun
America/Caracas
America/Catamarca
America/Cayenne
America/Cayman
America/Chicago
America/Chihuahua
America/Ciudad_Juarez
America/Coral_Harbour
America/Cordoba
America/Costa_Rica
America/Coyhaique
America/Creston
America/Cuiaba
America/Curacao
America/Danmarkshavn
America/Dawson
America/Dawson_Creek
America/Denver
America/Detroit
America/Dominica
America/Edmonton
America/Eirunepe
America/El_Salvador
America/Ensenada
America/Fort_Nelson
America/Fort_Wayne
America/Fortaleza
America/Glace_Bay
America/Godthab
America/Goose_Bay
America/Grand_Turk
America/Grenada
America/Guadeloupe
America/Guatemala
America/Guayaquil
America/Guyana
America/Halifax
America/Havana
America/Hermosillo
America/Indiana/Indianapolis
America/Indiana/Knox
America/Indiana/Marengo
America/Indiana/Petersburg
America/Indiana/Tell_City
America/Indiana/Vevay
America/Indiana/Vincennes
America/Indiana/Winamac
America/Indianapolis
America/Inuvik
America/Iqaluit
America/Jamaica
America/Jujuy
America/Juneau
America/Kentucky/Louisville
America/Kentucky/Monticello
America/Knox_IN
America/Kralendijk
America/La_Paz
America/Lima
America/Los_Angeles
America/Louisville
America/Lower_Princes
America/Maceio
America/Managua
America/Manaus
America/Marigot
America/Martinique
America/Matamoros
America/Mazatlan
America/Mendoza
America/Menominee
America/Merida
America/Metlakatla
America/Mexico_City
America/Miquelon
America/Moncton
America/Monterrey
America/Montevideo
America/Montreal
America/Montserrat
America/Nassau
America/New_York
America/Nipigon
America/Nome
America/Noronha
America/North_Dakota/Beulah
America/North_Dakota/Center
America/North_Dakota/New_Salem
America/Nuuk
America/Ojinaga
America/Panama
America/Pangnirtung
America/Paramaribo
America/Phoenix
America/Port-au-Prince
America/Port_of_Spain
America/Porto_Acre
America/Porto_Velho
America/Puerto_Rico
America/Punta_Arenas
America/Rainy_River
America/Rankin_Inlet
America/Recife
America/Regina
America/Resolute
America/Rio_Branco
America/Rosario
America/Santa_Isabel
America/Santarem
America/Santiago
America/Santo_Domingo
America/Sao_Paulo
America/Scoresbysund
America/Shiprock
America/Sitka
America/St_Barthelemy
America/St_Johns
America/St_Kitts
America/St_Lucia
America/St_Thomas
America/St_Vincent
America/Swift_Current
America/Tegucigalpa
America/Thule
America/Thunder_Bay
America/Tijuana
America/Toronto
America/Tortola
America/Vancouver
America/Virgin
America/Whitehorse
America/Winnipeg
America/Yakutat
America/Yellowknife
Antarctica/Casey
Antarctica/Davis
Antarctica/DumontDUrville
Antarctica/Macquarie
Antarctica/Mawson
Antarctica/McMurdo
Antarctica/Palmer
Antarctica/Rothera
Antarctica/South_Pole
Antarctica/Syowa
Antarctica/Troll
Antarctica/Vostok
Arctic/Longyearbyen
Asia/Aden
Asia/Almaty
Asia/Amman
Asia/Anadyr
Asia/Aqtau
Asia/Aqtobe
Asia/Ashgabat
Asia/Ashkhabad
Asia/Atyrau
Asia/Baghdad
Asia/Bahrain
Asia/Baku
Asia/Bangkok
Asia/Barnaul
Asia/Beirut
Asia/Bishkek
Asia/Brunei
Asia/Calcutta
Asia/Chita
Asia/Choibalsan
Asia/Chongqing
Asia/Chungking
Asia/Colombo
Asia/Dacca
Asia/Damascus
Asia/Dhaka
Asia/Dili
Asia/Dubai
Asia/Dushanbe
Asia/Famagusta
Asia/Gaza
Asia/Harbin
Asia/Hebron
Asia/Ho_Chi_Minh
Asia/Hong_Kong
Asia/Hovd
Asia/Irkutsk
Asia/Istanbul
Asia/Jakarta
Asia/Jayapura
Asia/Jerusalem
Asia/Kabul
Asia/Kamchatka
Asia/Karachi
Asia/Kashgar
Asia/Kathmandu
Asia/Katmandu
Asia/Khandyga
Asia/Kolkata
Asia/Krasnoyarsk
Asia/Kuala_Lumpur
Asia/Kuching
Asia/Kuwait
Asia/Macao
Asia/Macau
Asia/Magadan
Asia/Makassar
Asia/Manila
Asia/Muscat
Asia/Nicosia
Asia/Novokuznetsk
Asia/Novosibirsk
Asia/Omsk
Asia/Oral
Asia/Phnom_Penh
Asia/Pontianak
Asia/Pyongyang
Asia/Qatar
Asia/Qostanay
Asia/Qyzylorda
Asia/Rangoon
Asia/Riyadh
Asia/Saigon
Asia/Sakhalin
Asia/Samarkand
Asia/Seoul
Asia/Shanghai
Asia/Singapore
Asia/Srednekolymsk
Asia/Taipei
Asia/Tashkent
Asia/Tbilisi
Asia/Tehran
Asia/Tel_Aviv
Asia/Thimbu
Asia/Thimphu
Asia/Tokyo
Asia/Tomsk
Asia/Ujung_Pandang
Asia/Ulaanbaatar
Asia/Ulan_Bator
Asia/Urumqi
Asia/Ust-Nera
Asia/Vientiane
Asia/Vladivostok
Asia/Yakutsk
Asia/Yangon
Asia/Yekaterinburg
Asia/Yerevan
Atlantic/Azores
Atlantic/Bermuda
Atlantic/Canary
Atlantic/Cape_Verde
Atlantic/Faeroe
Atlantic/Faroe
Atlantic/Jan_Mayen
Atlantic/Madeira
Atlantic/Reykjavik
Atlantic/South_Georgia
Atlantic/St_Helena
Atlantic/Stanley
Australia/ACT
Australia/Adelaide
Australia/Brisbane
Australia/Broken_Hill
Australia/Canberra
Australia/Currie
Australia/Darwin
Australia/Eucla
Australia/Hobart
Australia/LHI
Australia/Lindeman
Australia/Lord_Howe
Australia/Melbourne
Australia/North
Australia/NSW
Australia/Perth
Australia/Queensland
Australia/South
Australia/Sydney
Australia/Tasmania
Australia/Victoria
Australia/West
Australia/Yancowinna
Brazil/Acre
Brazil/DeNoronha
Brazil/East
Brazil/West
Canada/Atlantic
Canada/Central
Canada/Eastern
Canada/Mountain
Canada/Newfoundland
Canada/Pacific
Canada/Saskatchewan
Canada/Yukon
CET
Chile/Continental
Chile/EasterIsland
CST6CDT
Cuba
EET
Egypt
Eire
EST
EST5EDT
Etc/GMT
Etc/GMT+0
Etc/GMT+1
Etc/GMT+10
Etc/GMT+11
Etc/GMT+12
Etc/GMT+2
Etc/GMT+3
Etc/GMT+4
Etc/GMT+5
Etc/GMT+6
Etc/GMT+7
Etc/GMT+8
Etc/GMT+9
Etc/GMT-0
Etc/GMT-1
Etc/GMT-10
Etc/GMT-11
Etc/GMT-12
Etc/GMT-13
Etc/GMT-14
Etc/GMT-2
Etc/GMT-3
Etc/GMT-4
Etc/GMT-5
Etc/GMT-6
Etc/GMT-7
Etc/GMT-8
Etc/GMT-9
Etc/GMT0
Etc/Greenwich
Etc/UCT
Etc/Universal
Etc/UTC
Etc/Zulu
Europe/Amsterdam
Europe/Andorra
Europe/Astrakhan
Europe/Athens
Europe/Belfast
Europe/Belgrade
Europe/Berlin
Europe/Bratislava
Europe/Brussels
Europe/Bucharest
Europe/Budapest
Europe/Busingen
Europe/Chisinau
Europe/Copenhagen
Europe/Dublin
Europe/Gibraltar
Europe/Guernsey
Europe/Helsinki
Europe/Isle_of_Man
Europe/Istanbul
Europe/Jersey
Europe/Kaliningrad
Europe/Kiev
Europe/Kirov
Europe/Kyiv
Europe/Lisbon
Europe/Ljubljana
Europe/London
Europe/Luxembourg
Europe/Madrid
Europe/Malta
Europe/Mariehamn
Europe/Minsk
Europe/Monaco
Europe/Moscow
Europe/Nicosia
Europe/Oslo
Europe/Paris
Europe/Podgorica
Europe/Prague
Europe/Riga
Europe/Rome
Europe/Samara
Europe/San_Marino
Europe/Sarajevo
Europe/Saratov
Europe/Simferopol
Europe/Skopje
Europe/Sofia
Europe/Stockholm
Europe/Tallinn
Europe/Tirane
Europe/Tiraspol
Europe/Ulyanovsk
Europe/Uzhgorod
Europe/Vaduz
Europe/Vatican
Europe/Vienna
Europe/Vilnius
Europe/Volgograd
Europe/Warsaw
Europe/Zagreb
Europe/Zaporozhye
Europe/Zurich
GB
GB-Eire
GMT
GMT+0
GMT-0
GMT0
Greenwich
Hongkong
HST
Iceland
Indian/Antananarivo
Indian/Chagos
Indian/Christmas
Indian/Cocos
Indian/Comoro
Indian/Kerguelen
Indian/Mahe
Indian/Maldives
Indian/Mauritius
Indian/Mayotte
Indian/Reunion
Iran
Israel
Jamaica
Japan
Kwajalein
Libya
localtime
MET
Mexico/BajaNorte
Mexico/BajaSur
Mexico/General
MST
MST7MDT
Navajo
NZ
NZ-CHAT
Pacific/Apia
Pacific/Auckland
Pacific/Bougainville
Pacific/Chatham
Pacific/Chuuk
Pacific/Easter
Pacific/Efate
Pacific/Enderbury
Pacific/Fakaofo
Pacific/Fiji
Pacific/Funafuti
Pacific/Galapagos
Pacific/Gambier
Pacific/Guadalcanal
Pacific/Guam
Pacific/Honolulu
Pacific/Johnston
Pacific/Kanton
Pacific/Kiritimati
Pacific/Kosrae
Pacific/Kwajalein
Pacific/Majuro
Pacific/Marquesas
Pacific/Midway
Pacific/Nauru
Pacific/Niue
Pacific/Norfolk
Pacific/Noumea
Pacific/Pago_Pago
Pacific/Palau
Pacific/Pitcairn
Pacific/Pohnpei
Pacific/Ponape
Pacific/Port_Moresby
Pacific/Rarotonga
Pacific/Saipan
Pacific/Samoa
Pacific/Tahiti
Pacific/Tarawa
Pacific/Tongatapu
Pacific/Truk
Pacific/Wake
Pacific/Wallis
Pacific/Yap
Poland
Portugal
PRC
PST8PDT
ROC
ROK
Singapore
Turkey
UCT
Universal
US/Alaska
US/Aleutian
US/Arizona
US/Central
US/East-Indiana
US/Eastern
US/Hawaii
US/Indiana-Starke
US/Michigan
US/Mountain
US/Pacific
US/Samoa
UTC
W-SU
WET
Zulu
```

Search for a timezone:

```text
$ dtg -Z ca/w
Africa/Windhoek
America/Whitehorse
America/Winnipeg
```

Multiple timezones:

```text
$ dtg -z UTC,EST5EDT,CST6CDT,MST7MDT,PST8PDT -f '%Z%n%H:%M:%S%n' -f '%Z%n%Y-%m-%d%n'
UTC
11:56:20

EDT
07:56:20

CDT
06:56:20

MDT
05:56:20

PDT
04:56:20

UTC
2025-08-28

EDT
2025-08-28

CDT
2025-08-28

MDT
2025-08-28

PDT
2025-08-28

```

*Note the above prints each format for each timezone... to print each timezone for each format,
use a single format and `%n`:*

```text
$ dtg -z UTC,EST5EDT,CST6CDT,MST7MDT,PST8PDT -f '%Z%n%H:%M:%S%n%n%Z%n%Y-%m-%d%n'
UTC
11:56:20

UTC
2025-08-28

EDT
07:56:20

EDT
2025-08-28

CDT
06:56:20

CDT
2025-08-28

MDT
05:56:20

MDT
2025-08-28

PDT
04:56:20

PDT
2025-08-28

```

Use a custom separator between formats/timezones:

```text
$ dtg -z PST8PDT,MST7MDT,CST6CDT,EST5EDT,UTC -f '[%Z %H:%M:%S]' -s ' '
[PDT 04:56:20] [MDT 05:56:20] [CDT 06:56:20] [EDT 07:56:20] [UTC 11:56:20]
```

Named formats:

* a, all
* bcd
* cd, compact-date
* cdt, compact-date-time
* ct, compact-time
* d, default
* i, r, rfc, rfc-3339
* x

```text
$ dtg -l -n all
1756382180.292252461
2025-08-28T11:56:20Z
Thu 28 Aug 2025 11:56:20 UTC
Thu 28 Aug 2025 07:56:20 EDT
```

```text
$ dtg -n bcd
⠄⢔|⠈|⠌|⣀|⡲|⠄
```

```text
$ dtg -l -n bcd
⠄⢔|⠈|⠌|⢰|⡲|⠄
```

```text
$ dtg -z MST7MDT -n bcd
⠄⢔|⠈|⠌|⢐|⡲|⠄
```

```text
$ dtg -n compact-date
20250828
```

```text
$ dtg -n compact-date-time
20250828-115620
```

```text
$ dtg -n compact-time
115620
```

```text
$ dtg -n default
Thu 28 Aug 2025 11:56:20 UTC
```

```text
$ dtg -n rfc-3339
2025-08-28T11:56:20Z
```

```text
$ dtg -n x
Xj7RBuK
```

Ordered named formats:

```text
$ dtg -x -a
1756382180.357787925
2025-08-28T11:56:20Z
Thu 28 Aug 2025 11:56:20 UTC
Thu 28 Aug 2025 07:56:20 EDT
Xj7RBuK
$ dtg -a -x
1756382180.364601334
2025-08-28T11:56:20Z
Thu 28 Aug 2025 11:56:20 UTC
Thu 28 Aug 2025 07:56:20 EDT
Xj7RBuK
$ dtg -n x -n all
Xj7RBuK
1756382180.370636986
2025-08-28T11:56:20Z
Thu 28 Aug 2025 11:56:20 UTC
Thu 28 Aug 2025 11:56:20 UTC
$ dtg -n all -n x
1756382180.377433216
2025-08-28T11:56:20Z
Thu 28 Aug 2025 11:56:20 UTC
Thu 28 Aug 2025 11:56:20 UTC
Xj7RBuK
```

# Library

```Rust
use chrono::{TimeZone, Utc};
use dtg_lib::{tz, Dtg, Format};

let epoch = 1658448142;
let nanoseconds = 936196858;
let rfc_3339 = "2022-07-22T00:02:22Z";
let default_utc = "Fri 22 Jul 2022 00:02:22 UTC";
let default_mt = "Thu 21 Jul 2022 18:02:22 MDT";
let x = "Xg6L02M";
let a_utc = format!("{epoch}.000000000\n{rfc_3339}\n{default_utc}\n{default_utc}");
let a_mt = format!("{epoch}.000000000\n{rfc_3339}\n{default_utc}\n{default_mt}");
let day_of_week_utc = "Friday";
let day_of_week_mt = "Thursday";
let tz_utc = tz("UTC").ok();
let tz_mt = tz("MST7MDT").ok();
let default_fmt = Some(Format::default());
let day_of_week_fmt = Some(Format::custom("%A"));

// Create Dtg

let dtg_1_str = format!("{}", epoch);

let dtg_1_ts = Dtg::from(&dtg_1_str).unwrap();
let dtg_1_dt = Dtg::from_dt(&Utc.timestamp(epoch, 0));
let dtg_1_x = Dtg::from_x(x).unwrap();

assert_eq!(dtg_1_ts, dtg_1_dt);
assert_eq!(dtg_1_dt, dtg_1_x);
assert_eq!(dtg_1_x, dtg_1_ts);

// Create Dtg with nanoseconds

let dtg_2_str = format!("{}.{}", epoch, nanoseconds);

let dtg_2_ts = Dtg::from(&dtg_2_str).unwrap();
let dtg_2_dt = Dtg::from_dt(&Utc.timestamp(epoch, nanoseconds));

assert_eq!(dtg_2_ts, dtg_2_dt);

// Default format

assert_eq!(dtg_1_ts.default(&None), default_utc);
assert_eq!(dtg_1_ts.default(&tz_utc), default_utc);
assert_eq!(dtg_1_ts.default(&tz_mt), default_mt);

assert_eq!(dtg_1_ts.format(&default_fmt, &None), default_utc);
assert_eq!(dtg_1_ts.format(&default_fmt, &tz_utc), default_utc);
assert_eq!(dtg_1_ts.format(&default_fmt, &tz_mt), default_mt);

// RFC 3339 format

assert_eq!(dtg_1_ts.rfc_3339(), rfc_3339);
assert_eq!(dtg_1_ts.format(&None, &None), rfc_3339);

// "x" format

assert_eq!(dtg_1_ts.x_format(), x);
assert_eq!(dtg_1_ts.format(&Some(Format::X), &None), x);

// "a" format

assert_eq!(dtg_1_ts.a_format(&None), a_utc);
assert_eq!(dtg_1_ts.a_format(&tz_utc), a_utc);
assert_eq!(dtg_1_ts.a_format(&tz_mt), a_mt);

assert_eq!(dtg_1_ts.format(&Some(Format::A), &None), a_utc);
assert_eq!(dtg_1_ts.format(&Some(Format::A), &tz_utc), a_utc);
assert_eq!(dtg_1_ts.format(&Some(Format::A), &tz_mt), a_mt);

// Custom format

assert_eq!(dtg_1_ts.format(&day_of_week_fmt, &None), day_of_week_utc);
assert_eq!(dtg_1_ts.format(&day_of_week_fmt, &tz_mt), day_of_week_mt);
```

# Formats

## Date specifiers

Spec. | Example       | Description
------|---------------|----------------------------------------------------------------------------
`%Y`  | `2001`        | The full proleptic Gregorian year, zero-padded to 4 digits.
`%C`  | `20`          | The proleptic Gregorian year divided by 100, zero-padded to 2 digits.
`%y`  | `01`          | The proleptic Gregorian year modulo 100, zero-padded to 2 digits.
`%m`  | `07`          | Month number (01--12), zero-padded to 2 digits.
`%b`  | `Jul`         | Abbreviated month name. Always 3 letters.
`%B`  | `July`        | Full month name. Also accepts corresponding abbreviation in parsing.
`%h`  | `Jul`         | Same as `%b`.
`%d`  | `08`          | Day number (01--31), zero-padded to 2 digits.
`%e`  | ` 8`          | Same as `%d` but space-padded. Same as `%_d`.
`%a`  | `Sun`         | Abbreviated weekday name. Always 3 letters.
`%A`  | `Sunday`      | Full weekday name. Also accepts corresponding abbreviation in parsing.
`%w`  | `0`           | Sunday = 0, Monday = 1, ..., Saturday = 6.
`%u`  | `7`           | Monday = 1, Tuesday = 2, ..., Sunday = 7. (ISO 8601)
`%U`  | `28`          | Week number starting with Sunday (00--53), zero-padded to 2 digits.
`%W`  | `27`          | Same as `%U`, but week 1 starts with the first Monday in that year instead.
`%G`  | `2001`        | Same as `%Y` but uses the year number in ISO 8601 week date.
`%g`  | `01`          | Same as `%y` but uses the year number in ISO 8601 week date.
`%V`  | `27`          | Same as `%U` but uses the week number in ISO 8601 week date (01--53).
`%j`  | `189`         | Day of the year (001--366), zero-padded to 3 digits.
`%D`  | `07/08/01`    | Month-day-year format. Same as `%m/%d/%y`.
`%x`  | `07/08/01`    | Locale's date representation (e.g., 12/31/99).
`%F`  | `2001-07-08`  | Year-month-day format (ISO 8601). Same as `%Y-%m-%d`.
`%v`  | ` 8-Jul-2001` | Day-month-year format. Same as `%e-%b-%Y`.

## Time specifiers

Spec.  | Example       | Description
-------|---------------|----------------------------------------------------------------------
`%H`   | `00`          | Hour number (00--23), zero-padded to 2 digits.
`%k`   | ` 0`          | Same as `%H` but space-padded. Same as `%_H`.
`%I`   | `12`          | Hour number in 12-hour clocks (01--12), zero-padded to 2 digits.
`%l`   | `12`          | Same as `%I` but space-padded. Same as `%_I`.
`%P`   | `am`          | `am` or `pm` in 12-hour clocks.
`%p`   | `AM`          | `AM` or `PM` in 12-hour clocks.
`%M`   | `34`          | Minute number (00--59), zero-padded to 2 digits.
`%S`   | `60`          | Second number (00--60), zero-padded to 2 digits.
`%f`   | `026490000`   | The fractional seconds (in nanoseconds) since last whole second.
`%.f`  | `.026490`     | Similar to `.%f` but left-aligned. These all consume the leading dot.
`%.3f` | `.026`        | Similar to `.%f` but left-aligned but fixed to a length of 3.
`%.6f` | `.026490`     | Similar to `.%f` but left-aligned but fixed to a length of 6.
`%.9f` | `.026490000`  | Similar to `.%f` but left-aligned but fixed to a length of 9.
`%3f`  | `026`         | Similar to `%.3f` but without the leading dot.
`%6f`  | `026490`      | Similar to `%.6f` but without the leading dot.
`%9f`  | `026490000`   | Similar to `%.9f` but without the leading dot.
`%R`   | `00:34`       | Hour-minute format. Same as `%H:%M`.
`%T`   | `00:34:60`    | Hour-minute-second format. Same as `%H:%M:%S`.
`%X`   | `00:34:60`    | Locale's time representation (e.g., 23:13:48).
`%r`   | `12:34:60 AM` | Hour-minute-second format in 12-hour clocks. Same as `%I:%M:%S %p`.

## Time zone specifiers

Spec. | Example  | Description
------|----------|--------------------------------------------------------------------------
`%Z`  | `ACST`   | Local time zone name. Skips all non-whitespace characters during parsing.
`%z`  | `+0930`  | Offset from the local time to UTC (with UTC being `+0000`).
`%:z` | `+09:30` | Same as `%z` but with a colon.
`%#z` | `+09`    | *Parsing only:* Same as `%z` but allows minutes to be missing or present.

## Date & time specifiers

Spec. | Example                            | Description
------|------------------------------------|------------------------------------------------------------------
`%c`  | `Sun Jul  8 00:34:60 2001`         | Locale's date and time (e.g., Thu Mar  3 23:05:25 2005).
`%+`  | `2001-07-08T00:34:60.026490+09:30` | ISO 8601 / RFC 3339 date & time format.
`%s`  | `994518299`                        | UNIX timestamp, the number of seconds since 1970-01-01 00:00 UTC.

## Special specifiers

Spec. | Description
------|------------------------
`%t`  | Literal tab (`\t`).
`%n`  | Literal newline (`\n`).
`%%`  | Literal percent sign.

