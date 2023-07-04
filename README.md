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
    "filename": "go1.20.4.darwin-amd64.tar.gz",
    "os": "darwin",
    "arch": "amd64",
    "version": "go1.20.4",
    "sha256": "242b099b5b9bd9c5d4d25c041216bc75abcdf8e0541aec975eeabcbce61ad47f",
    "size": 100142830,
    "kind": "archive"
   },
   {
    "filename": "go1.20.4.darwin-amd64.pkg",
    "os": "darwin",
    "arch": "amd64",
    "version": "go1.20.4",
    "sha256": "9f5de2cd6e0785029bfb8f4a505842e4d797e09fe394e4eb83415a0b46ba3fe5",
    "size": 100653335,
    "kind": "installer"
   },
   {
    "filename": "go1.20.4.darwin-arm64.tar.gz",
    "os": "darwin",
    "arch": "arm64",
    "version": "go1.20.4",
    "sha256": "61bd4f7f2d209e2a6a7ce17787fc5fea52fb11cc9efb3d8471187a8b39ce0dc9",
    "size": 96875556,
    "kind": "archive"
   },
   {
    "filename": "go1.20.4.darwin-arm64.pkg",
    "os": "darwin",
    "arch": "arm64",
    "version": "go1.20.4",
    "sha256": "829ca0f614b722805b8ce9e92e5e76d3bd45c247220ee43fdc8f5d118e344f44",
    "size": 97125856,
    "kind": "installer"
   },
   {
    "filename": "go1.20.4.freebsd-386.tar.gz",
    "os": "freebsd",
    "arch": "386",
    "version": "go1.20.4",
    "sha256": "66b1646786304553c5f3208d4b5ed4b3f227293728bfa5c7b5d9a3c5fa1312cb",
    "size": 100195402,
    "kind": "archive"
   },
   {
    "filename": "go1.20.4.freebsd-amd64.tar.gz",
    "os": "freebsd",
    "arch": "amd64",
    "version": "go1.20.4",
    "sha256": "24ee729660372fb408c34a11284daa6e17e43e1db4a5bee2a96b4b6d291edfc5",
    "size": 99893313,
    "kind": "archive"
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
    "filename": "go1.20.4.linux-arm64.tar.gz",
    "os": "linux",
    "arch": "arm64",
    "version": "go1.20.4",
    "sha256": "105889992ee4b1d40c7c108555222ca70ae43fccb42e20fbf1eebb822f5e72c6",
    "size": 95470016,
    "kind": "archive"
   },
   {
    "filename": "go1.20.4.linux-armv6l.tar.gz",
    "os": "linux",
    "arch": "armv6l",
    "version": "go1.20.4",
    "sha256": "0b75ca23061a9996840111f5f19092a1bdbc42ec1ae25237ed2eec1c838bd819",
    "size": 97868577,
    "kind": "archive"
   },
   {
    "filename": "go1.20.4.linux-ppc64le.tar.gz",
    "os": "linux",
    "arch": "ppc64le",
    "version": "go1.20.4",
    "sha256": "8c6f44b96c2719c90eebabe2dd866f9c39538648f7897a212cac448587e9a408",
    "size": 95978513,
    "kind": "archive"
   },
   {
    "filename": "go1.20.4.linux-s390x.tar.gz",
    "os": "linux",
    "arch": "s390x",
    "version": "go1.20.4",
    "sha256": "57f999a4e605b1dfa4e7e58c7dbae47d370ea240879edba8001ab33c9a963ebf",
    "size": 100216921,
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
   },
   {
    "filename": "go1.20.4.windows-arm64.zip",
    "os": "windows",
    "arch": "arm64",
    "version": "go1.20.4",
    "sha256": "691b292c8284f31864b998f5bef8bc6d639799dec2bc319bfbe67dc6986ae02f",
    "size": 108504719,
    "kind": "archive"
   },
   {
    "filename": "go1.20.4.windows-arm64.msi",
    "os": "windows",
    "arch": "arm64",
    "version": "go1.20.4",
    "sha256": "b9f53616fc81ade97999ae9117ace8a0a7d2785f60436df4f3283d6153a38153",
    "size": 95834112,
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
   },
   {
    "filename": "go1.19.9.darwin-amd64.pkg",
    "os": "darwin",
    "arch": "amd64",
    "version": "go1.19.9",
    "sha256": "c282d55f01a130de102e7f082ed55f802fb2eac95e81c55da9171227618c96a8",
    "size": 151916489,
    "kind": "installer"
   },
   {
    "filename": "go1.19.9.darwin-arm64.tar.gz",
    "os": "darwin",
    "arch": "arm64",
    "version": "go1.19.9",
    "sha256": "f06e07f313bb914c6364b4d2cafb7d16d4782176fd34fbe0a5937d7ea40cc58b",
    "size": 146553431,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.darwin-arm64.pkg",
    "os": "darwin",
    "arch": "arm64",
    "version": "go1.19.9",
    "sha256": "bad596858ce410a965321c3032927fd8be2892634e3c686fedd4c270f462fcf3",
    "size": 145722394,
    "kind": "installer"
   },
   {
    "filename": "go1.19.9.freebsd-386.tar.gz",
    "os": "freebsd",
    "arch": "386",
    "version": "go1.19.9",
    "sha256": "07fb72dc6fb1137ca3c2fd6b90e30426e9584c07c1d20d6dc1aaaab4d909e3de",
    "size": 119939735,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.freebsd-amd64.tar.gz",
    "os": "freebsd",
    "arch": "amd64",
    "version": "go1.19.9",
    "sha256": "a6a3e8d9cc6cfb2765972d1a44adcfcc5ac91e214559a87ae50ba8d1308dbf4a",
    "size": 148987416,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.linux-386.tar.gz",
    "os": "linux",
    "arch": "386",
    "version": "go1.19.9",
    "sha256": "c06ed6ac131507e637af43ab47e5fc2f191142b258b59c2769ea261bf1337943",
    "size": 120183960,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.linux-amd64.tar.gz",
    "os": "linux",
    "arch": "amd64",
    "version": "go1.19.9",
    "sha256": "e858173b489ec1ddbe2374894f52f53e748feed09dde61be5b4b4ba2d73ef34b",
    "size": 149067328,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.linux-arm64.tar.gz",
    "os": "linux",
    "arch": "arm64",
    "version": "go1.19.9",
    "sha256": "b947e457be9d7b52a082c68e42b6939f9cc151f1ad5b3d8fd646ca3352f6f2f1",
    "size": 115272407,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.linux-armv6l.tar.gz",
    "os": "linux",
    "arch": "armv6l",
    "version": "go1.19.9",
    "sha256": "4a613b3c9d74975470096c5323e54db0f5f0f56dd0471fc1a7ee04e559032dc4",
    "size": 116872373,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.linux-ppc64le.tar.gz",
    "os": "linux",
    "arch": "ppc64le",
    "version": "go1.19.9",
    "sha256": "6855899fdab7aefed4dff869e8399d4df68d47506a0b31fbb3ac818255dbc4ae",
    "size": 115673275,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.linux-s390x.tar.gz",
    "os": "linux",
    "arch": "s390x",
    "version": "go1.19.9",
    "sha256": "ffdde2fb657761f67938e00a094fa7df2f41c129287e34326189d448b5cc7f91",
    "size": 119158553,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.windows-386.zip",
    "os": "windows",
    "arch": "386",
    "version": "go1.19.9",
    "sha256": "1ba29ea3659ca4fce635e7d028773ff133c956f4a0fc995adb3197f88a30c508",
    "size": 134401106,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.windows-386.msi",
    "os": "windows",
    "arch": "386",
    "version": "go1.19.9",
    "sha256": "c6c29daedbc841764e36446cbb6e476da1c7fcd201a1765a65208116b8a55cac",
    "size": 116989952,
    "kind": "installer"
   },
   {
    "filename": "go1.19.9.windows-amd64.zip",
    "os": "windows",
    "arch": "amd64",
    "version": "go1.19.9",
    "sha256": "3b0ca22aedf5fd85e84c944dd96ab3044213bf224cc3e9850ad86f1f71e1be93",
    "size": 163508111,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.windows-amd64.msi",
    "os": "windows",
    "arch": "amd64",
    "version": "go1.19.9",
    "sha256": "8a568fc4c77084f1308bc4153bc561094a7d642cc76cc71aead4d9f7017c6dda",
    "size": 141803520,
    "kind": "installer"
   },
   {
    "filename": "go1.19.9.windows-arm64.zip",
    "os": "windows",
    "arch": "arm64",
    "version": "go1.19.9",
    "sha256": "7e74c5f2ef1ba76c8afc56ab53eeda5eac676743fe6929296f14676c8f4d424a",
    "size": 128253904,
    "kind": "archive"
   },
   {
    "filename": "go1.19.9.windows-arm64.msi",
    "os": "windows",
    "arch": "arm64",
    "version": "go1.19.9",
    "sha256": "f1df2d92447ccd3644ff37f5c752d455586e335c1e025971f16a6c316ad2e649",
    "size": 111710208,
    "kind": "installer"
   }
  ]
 }
]
```
