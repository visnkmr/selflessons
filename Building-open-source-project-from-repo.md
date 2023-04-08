# Building open source project eg vscode  

- Go to the latest release or a version you can confirm works.
- then follow instructions as listed in the docs to build the project

In the case of vscode to build from repo
- Go to releases and download the Source zip.
- Open it in webstorm or vscode or any dev environment
- run ```yarn``` from terminal. This will install all required deps.
- If you get an issue relating to postinstall.js then comment out last 2 lines in the file. Courtsey: https://github.com/microsoft/vscode/issues/133544
- Now run .\scripts\code.bat from terminal
-  yarn gulp "vscode-win32-x64-min-ci" to build exe file, linux options also available checkout [vscodium -build.sh file](https://github.com/VSCodium/vscodium/blob/master/build.sh)
-  Builds exe file in a sub-folder thats parent to the project directory.
