& .\build.ps1
..\..\target\release\splitter.exe -p ..\..\data\tb_event_small.csv -l 10 -t \tmp --record-regex "(?<!\\)\r?\n"