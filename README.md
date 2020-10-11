## :warning: This code has been merged to https://github.com/Schniz/fnm :warning:

# fnm_rust

An experimental Rust implementation of [fnm](https://github.com/Schniz/fnm)

## Testing

The tests should be written in a DSL that will generate tests for every shell we support.

## Windows Support

I'm working on it. I want everything to be well-tested.

The shell initialization script in Windows looks like:

```
FOR /f "tokens=*" %i IN ('fnm env --use-on-cd') DO CALL %i
```

PowerShell is:

```
fnm env --use-on-cd | Out-String | Invoke-Expression
```
### Process listing

We can get the pid and parent pid by running the following command:

```batch
wmic process get processid,parentprocessid,executablepath /format:csv
```

We will get back a CSV that looks like:
```csv
Node,ExecutablePath,ParentProcessId,ProcessId
WINDEV2007EVAL,,0,0
WINDEV2007EVAL,,0,4
WINDEV2007EVAL,,4,72
WINDEV2007EVAL,,4,332
WINDEV2007EVAL,,416,424
WINDEV2007EVAL,,416,492
WINDEV2007EVAL,,484,500
WINDEV2007EVAL,C:\Windows\system32\winlogon.exe,484,548
WINDEV2007EVAL,,492,612
WINDEV2007EVAL,C:\Windows\system32\lsass.exe,492,620
WINDEV2007EVAL,C:\Windows\system32\fontdrvhost.exe,492,712
WINDEV2007EVAL,C:\Windows\system32\fontdrvhost.exe,548,720
...
```
Then we can parse it with Serde and traverse everything in-memory to look for the binary!
