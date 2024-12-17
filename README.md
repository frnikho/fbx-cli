<h1 align="center">
  <br>
  <a href="http://www.amitmerchant.com/electron-markdownify"><img src="https://raw.githubusercontent.com/amitmerchant1990/electron-markdownify/master/app/img/markdownify.png" alt="Markdownify" width="200"></a>
  <br>
  fbx-cli
  <br>
</h1>


<h4 align="center">fbx-cli est un utilitaire en ligne de commande pour controller et gérer votre freebox depuis votre terminal</h4>

<p align="center">
  <a href="#key-features">Key Features</a> •
  <a href="#how-to-use">How To Use</a> •
  <a href="#download">Download</a> •
  <a href="#credits">Credits</a> •
  <a href="#related">Related</a> •
  <a href="#license">License</a>
</p>

![screenshot](https://raw.githubusercontent.com/amitmerchant1990/electron-markdownify/master/app/img/markdownify.gif)

## Fonctionnalités

* Authentification avec la freebox
* Afficher les informations de la freebox
* Afficher les périphériques connectés
* Afficher les informations du système
* Afficher les informations des VM
  * Démarrer une VM
  * Arrêter une VM
  * Redémarrer une VM
  * Mettre en pause une VM

* Cross platform
  - Windows, macOS et Linux.

## Installation


### From sources

Rustup version 1.58 ou + doit être installé

```shell
# 
git clone ***

cargo build --release


```

> **Note**
> Si vous êtes sur windows, vous devrez ajouter , [see this guide](https://www.howtogeek.com/261575/how-to-run-graphical-linux-desktop-applications-from-windows-10s-bash-shell/) or use `node` from the command prompt.


## Utilisation

---

## Related

[markdownify-web](https://github.com/amitmerchant1990/markdownify-web) - Web version of Markdownify

## Support

<a href="https://www.buymeacoffee.com/5Zn8Xh3l9" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/purple_img.png" alt="Buy Me A Coffee" style="height: 41px !important;width: 174px !important;box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;-webkit-box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;" ></a>

<p>Or</p> 

<a href="https://www.patreon.com/amitmerchant">
	<img src="https://c5.patreon.com/external/logo/become_a_patron_button@2x.png" width="160">
</a>

## You may also like...

- [Pomolectron](https://github.com/amitmerchant1990/pomolectron) - A pomodoro app
- [Correo](https://github.com/amitmerchant1990/correo) - A menubar/taskbar Gmail App for Windows and macOS

## License

MIT




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
  - [x] LCD  100% 
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