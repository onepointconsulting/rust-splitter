& .\build.ps1
..\..\target\release\splitter.exe -p ..\..\data\tb_event.csv -l 10000 -t \tmp --record-regex "(?<!\\)\r\n"