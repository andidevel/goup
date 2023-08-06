# GoUp - Golang updater

Check for new **Golang** versions on *https://go.dev/dl/?mode=json*.

Compare with the current version installed.

If there is a new one, download it and install it.

Linux/Unix only.

This is a learning project. Needs improvement.

## Golang website endpoint

`GET https://go.dev/dl/?mode=json`

**Response**
```json
[
 {
  "version": "go1.20.4",
  "stable": true,
  "files": [
   {
    "filename": "go1.20.4.src.tar.gz",
    "os": "",
    "arch": "",
    "version": "go1.20.4",
    "sha256": "9f34ace128764b7a3a4b238b805856cc1b2184304df9e5690825b0710f4202d6",
    "size": 26185429,
    "kind": "source"
   },
   {
    "filename": "go1.20.4.linux-386.tar.gz",
    "os": "linux",
    "arch": "386",
    "version": "go1.20.4",
    "sha256": "5dfa3db9433ef6a2d3803169fb4bd2f4505414881516eb9972d76ab2e22335a7",
    "size": 100573399,
    "kind": "archive"
   },
   {
    "filename": "go1.20.4.linux-amd64.tar.gz",
    "os": "linux",
    "arch": "amd64",
    "version": "go1.20.4",
    "sha256": "698ef3243972a51ddb4028e4a1ac63dc6d60821bf18e59a807e051fee0a385bd",
    "size": 100148454,
    "kind": "archive"
   },
   {
    "filename": "go1.20.4.windows-386.zip",
    "os": "windows",
    "arch": "386",
    "version": "go1.20.4",
    "sha256": "8f2c5574bb822cc02d3bad4d449e4d2a2de341663df63ad0e7cb0b650a321dab",
    "size": 114776553,
    "kind": "archive"
   },
   {
    "filename": "go1.20.4.windows-386.msi",
    "os": "windows",
    "arch": "386",
    "version": "go1.20.4",
    "sha256": "03bdb95782028af5140233de1649d5e1b76b046955502f9c51fc364dc1ac4c5c",
    "size": 100290560,
    "kind": "installer"
   },
   {
    "filename": "go1.20.4.windows-amd64.zip",
    "os": "windows",
    "arch": "amd64",
    "version": "go1.20.4",
    "sha256": "e7528da720f470b711fbd826814167a5fe1bc02a479ab1958dcf839a8294e6d2",
    "size": 114002098,
    "kind": "archive"
   },
   {
    "filename": "go1.20.4.windows-amd64.msi",
    "os": "windows",
    "arch": "amd64",
    "version": "go1.20.4",
    "sha256": "d9f24142072ec50b06325b317b9f40e7c50b7a22949f55b39b467718193c6e46",
    "size": 100040704,
    "kind": "installer"
   }
  ]
 },
 {
  "version": "go1.19.9",
  "stable": true,
  "files": [
   {
    "filename": "go1.19.9.src.tar.gz",
    "os": "",
    "arch": "",
    "version": "go1.19.9",
    "sha256": "131190a4697a70c5b1d232df5d3f55a3f9ec0e78e40516196ffb3f09ae6a5744",
    "size": 26556330,
    "kind": "source"
   },
   {
    "filename": "go1.19.9.darwin-amd64.tar.gz",
    "os": "darwin",
    "arch": "amd64",
    "version": "go1.19.9",
    "sha256": "22e2fc77a8f11709a2c9ffc7d5699ba226753b2ed3e30574049c2dc28870dc7a",
    "size": 152321066,
    "kind": "archive"
   }
  ]
 }
]
```
