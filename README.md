# FBX


<!-- ROADMAP -->
## Roadmap

### V1.0.0

- [ ] Save and load the fbx app inside the user cache directory
- [ ] Handle different freebox API version
- [ ] Handle commands 
  - [x] Auth
    - [x] Login
    - [x] Logout
    - [x] Status
  - [x] Device
    - [x] List
    - [x] Get
  - [ ] Wifi
  - [ ] DHCP
  - [ ] Switch
  - [ ] System
  - [ ] LCD
  - [ ] Call, Voicemail and contact
  - [ ] Port forwarding
  - [ ] Parental filter
  - [ ] Connection
- [ ] Support flags 
  - [ ] Verbose
  - [ ] Debug
  - [ ] Version
  - [ ] Help
  - [ ] Display
    - [ ] JSON
    - [ ] YAML
- [ ] Distribution on different OS
  - [ ] Linux
    - [ ] Debian
    - [ ] Fedora
    - [ ] Arch
  - [ ] MacOS
  - [ ] Windows
### V1.x.x

- [ ] Interactive TUI support
- [ ] Discover freebox devices with mDNS
- [ ] Commands
  - [ ] VM
  - [ ] Download
  - [ ] File
  - [ ] Home
  - [ ] AirMedia

See the [open issues](https://github.com/othneildrew/Best-README-Template/issues) for a full list

## Flow

```shell
# authorize the app to access the freebox
$ fbx auth login https://mafreebox.freebox.fr/
# You can discover all freebox device using this command to see url
$ fbx auth discover 
# Show the current status of authentication with the freebox
$ fbx auth status
# logout from the freebox
$ fbx auth logout

$ fbx device list

$ fbx system info

$ fbx vm list
$ fbx vm start <vm_id>
```