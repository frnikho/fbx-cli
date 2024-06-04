### Flow
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