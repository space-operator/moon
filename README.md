# Space Operator - Project Moon


## Getting Started

Use the below steps to build

### Device specific

Windows

Install LLVM
winget install -e --id LLVM.LLVM

Linux
Install libclangdev - sudo apt-get install libclang-dev

Android
use NDK version 22.1.7171670
- export ANDROID_NDK_HOME="/home/amir/Android/Sdk/ndk/22.1.7171670" 
- export PATH="$PATH:/home/amir/Android/Sdk/ndk/22.1.7171670/toolchains/llvm/prebuilt/linux-x86_64/bin/"

OpenSSL - might not be necessary after vendored feature in cargo.toml
- https://www.howtoforge.com/tutorial/how-to-install-openssl-from-source-on-linux/
- https://agryaznov.com/guides/2019/05/20/substrate-install.html#openssl
- fix shared library
    - export LD_LIBRARY_PATH=/usr/local/lib:/usr/local/lib64
- export OPENSSL_DIR="/usr/local/ssl"

### 0. Setup Flutter


flutter config
```
flutter config --enable-windows-desktop
flutter config --enable-macos-desktop
flutter config --enable-linux-desktop
```

generate device specific build files (yes, just a dot after create)
```
flutter create .
```


### 1. Generate Glue Code

Windows and Linux
```sh
./sh/bindgen
```

macOS M1
```sh
./sh/bindgen-m1
```

### 2. Build For Desired Target/Device

Run any of the below three to build the binary for the specific device and have it placed into
the devices specific plugin folder.

macOS
```sh
./sh/macos
```

Windows & Linux
Not required

### 3. Run with Flutter

Run on the device.


```sh
flutter devices (find the available device names)

flutter run -d [device]

e.g.
flutter run -d windows
```


### 4. Develop

Run step `1` whenever a function exposed to Flutter changes.

Run step `2` whenever any of your Rust code changes.

**Note** that to apply changes from Rust you need to restart the app to reload the compiled binary.
A hot restart/reload does not achieve this.

## Folder Structure

```
├── android
├── ios
├── macos
├── lib
├── plugin
│   ├── android
│   ├── ios
│   ├── macos
│   └── lib
└── src
```

### `./plugin`

Provides connection from Flutter to Rust.

Rust binaries are placed into the respective plugin folders `./ios, ./macos, ./android` when
they are built.

Generated Dart glue code is placed inside `./plugin/lib/generated` while
`./plugin/lib/plugin.dart` just exposes the API to the app.

### `./src`

Contains the starter Rust code inside `./src/lib.rs`. Keep developing the Rust part of your app
here.

### `./lib`

Contains the starter Flutter app inside `./lib/main.dart`.

### `./sh`

Provides scripts to run build and code generation tasks. In the future a tool will provide the
functionality currently provided by these scripts.

- `bindgen` generates the `binding.h` header file for the extern Rust functions found inside
  `./src`. These are then placed inside the `./plugin` device folders were needed as well as
  `./plugin/lib/generated/binding.h` where they are used to generate Dart glue code
  - as part of this script `ffigen` generates Dart glue code inside
    `./plugin/lib/generated/ffigen_binding.dart` using `./plugin/lib/generated/binding.h` as input
- `./android` builds the Rust binary to run on Android devices/emulators and places it inside
  `./plugin/lib/android`
- `./ios` builds the Rust binary to run on IOS devices/emulators and places it inside
  `./plugin/lib/ios`
- `./macos` builds the Rust binary to run on MacOs directly and places it inside
  `./plugin/lib/macos`, this is the same format as running `cargo build` on your Mac
- `clean` cleans both the Flutter plugin and application, run this to reset Flutter when things
  aren't working
  
  


### Release Mode
Make sure to run cargo build in release mode

Windows
- run dumpbin to get dependent DLLs
- C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.31.31103\bin\Hostx64\x86>dumpbin /dependents C:\Users\amirb\rust\moon\build\windows\runner\Release\moon.dll

