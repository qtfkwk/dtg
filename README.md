# About

Date/time CLI utility

# Usage

~~~text
dtg 3.3.1
Date/time CLI utility; https://github.com/qtfkwk/dtg

USAGE:
    dtg [FLAGS] [OPTIONS] [--] [ARG]...

FLAGS:
    -a               "a" format (1)
    -X               Give timestamp argument(s) in "x" format (2)
    -h, --help       Prints help information
    -Z               Search/list timezones
    -l               Local timezone
        --readme     Print the readme
    -V, --version    Prints version information
    -x               "x" format (2)

OPTIONS:
    -f <formats>...        Format(s) [-z/-l: "%a %d %b %Y %H:%M:%S %Z",
                           "%Y-%m-%dT%H:%M:%SZ"]
    -z <zone>              Timezone [default: UTC]

ARGS:
    <ARG>...    Argument [-X: timestamp in "x" format (2), -Z: timezone
                search term, timestamp in "%s.%f" format, default: now]

NOTES:
    1. "a" format:

       ```
       %s.%f
       %Y-%m-%dT%H:%M:%SZ
       %a %d %b %Y %H:%M:%S %Z
       %a %d %b %Y %H:%M:%S %Z # -l implied or use -z <zone>
       ```

    2. "x" format (novel UTC / base 60 encoding):

       ```
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
~~~

# Examples

Get current date/time in UTC and RFC 3339 format:

```text
$ dtg
2020-11-27T03:21:16Z
```

Get current date/time in the local timezone and default format:

```text
$ dtg -l
Thu 26 Nov 2020 22:21:16 EST
```

Get current date/time in explicit timezone and default format:

```text
$ dtg -z MST
Thu 26 Nov 2020 20:21:16 MST
```

```text
$ dtg -z America/Los_Angeles
Thu 26 Nov 2020 19:21:16 PST
```

Get current date/time in UTC and specific format:

```text
$ dtg -f %A
Friday
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
1606447276.941324100
2020-11-27T03:21:16Z
Fri 27 Nov 2020 03:21:16 UTC
Thu 26 Nov 2020 22:21:16 EST
```

Get current date/time in explicit timezone and "a" format:

```text
$ dtg -a -z MST
1606447276.941324100
2020-11-27T03:21:16Z
Fri 27 Nov 2020 03:21:16 UTC
Thu 26 Nov 2020 20:21:16 MST
```

Get current date/time in "x" format:

```text
$ dtg -x
XeAQ3LG
```

Get a specific date / time in UTC and RFC 3339 format:

```text
$ dtg 1606447276.941324100
2020-11-27T03:21:16Z
```

Get a specific date/time in the local timezone and default format:

```text
$ dtg -l 1606447276.941324100
Thu 26 Nov 2020 22:21:16 EST
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
America/Coral_Harbour
America/Cordoba
America/Costa_Rica
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
Australia/NSW
Australia/North
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
CET
CST6CDT
Canada/Atlantic
Canada/Central
Canada/Eastern
Canada/Mountain
Canada/Newfoundland
Canada/Pacific
Canada/Saskatchewan
Canada/Yukon
Chile/Continental
Chile/EasterIsland
Cuba
EET
EST
EST5EDT
Egypt
Eire
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
Etc/UTC
Etc/Universal
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
HST
Hongkong
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
MET
MST
MST7MDT
Mexico/BajaNorte
Mexico/BajaSur
Mexico/General
NZ
NZ-CHAT
Navajo
PRC
PST8PDT
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
ROC
ROK
Singapore
Turkey
UCT
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
US/Pacific-New
US/Samoa
UTC
Universal
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

# Formats

The following information originates from the
[chrono documentation](https://docs.rs/chrono/0.4/chrono/format/strftime/index.html#specifiers),
which `dtg` uses internally.

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

# Changelog

* 1.0.2: Initial public release
* 2.0.0
    * General improvements to readme and code
    * Replaced `-u` with `-l` and ability to get local timezone via the
      [iana-time-zone crate](https://crates.io/crates/iana-time-zone)
* 2.0.1: Improve readme (add `-f` examples...); `s/-r/-l/` in usage
* 2.1.0: Enable bundling options (`-a -z TZ`: `-az TZ`, `-a -l`: `-al`, `-l -a`: `-la`,
  `-l -f FORMAT`: `-lf FORMAT`)
* 2.2.0: Add `-x` option
* 2.2.1: Fix `-x` option year from hex to base 60
* 2.2.2: Update doc; allow `-ax`, `-axz`, `-xa`, `-xaz`
* 2.2.3: Enable multiple `-f` options
* 2.2.4: Add `-X` option
* 3.0.0: Rewrite with structopt; add `-Z`
* 3.1.0: Add `--readme`
* 3.2.0: Improve doc
* 3.2.1: Fix `-a` option (yanked)
* 3.2.2: Fix readme
* 3.3.0: Improve doc; upgrade dependencies
* 3.3.1: Fix tables in readme

