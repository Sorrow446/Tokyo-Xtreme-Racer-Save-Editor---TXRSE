# Tokyo Xtreme Racer Save Editor - TXRSE
CLI save editor for Tokyo Xtreme Racer 2025.    
![](https://i.imgur.com/MhkteCr.png)    
[Pre-compiled binaries](https://github.com/Sorrow446/Tokyo-Xtreme-Racer-Save-Editor---TXRSE/releases)
## Usage

### CLI
Set the CP to 500K and the BP to 100:    
`txrse.exe --cp 500000 --bp 100 -i G:\rust\txr\UserData_00.sav`

Wrap the input save file path in double quotes if it contains any spaces.

```
Usage: txrse.exe --in-path <IN_PATH> <--bp <BP>|--cp <CP>>

Options:
  -i, --in-path <IN_PATH>  Input path of save file.
      --bp <BP>            Set BP. Max: 999
      --cp <CP>            Set CP. Max: 999,999,999,999
  -h, --help               Print help
```

### Interactive batch file
If you can't be bothered with CLI, simply drag and drop your save file onto the included batch file. The TXRSE binary's still required.

## Thank you
- Uses [trumank's great uesave-rs library](https://github.com/trumank/uesave-rs) which does the heavy lifting.

## Disclaimer   
- I won't be responsible for the rare chance of your saves getting corrupted. Back them up if you want.
- TXRSE has no partnership, sponsorship or endorsement with Genki Co., Ltd or Tokyo Xtreme Racer.
