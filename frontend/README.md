# Rune Frontend

This is a React Native app with [Expo](https://docs.expo.dev/).

## Run

1. Install dependencies

```bash
pnpm install
```

2. Start the development server

```bash
pnpm expo start
```

### For web

Once you start the development server, you can access the app with your browser. Press `w` on the console window.

### For iOS Devices/Simulators

To build and run on iOS simulator, you need a macOS computer with Xcode.
Follow [this documentation](https://docs.expo.dev/get-started/set-up-your-environment/?platform=ios&device=simulated&mode=development-build&buildEnv=local) to setup.

After the setup, run the following command to install the development build on your device/simulator.

```bash
pnpm expo run:ios
```

To configure the build process, open the app with xcode:

```bash
xed ios
```

### For Android Devices/Emulators

You need Android Studio to build for Android.
Follow [this guide](https://docs.expo.dev/get-started/set-up-your-environment/?platform=android&device=physical&mode=development-build&buildEnv=local) to setup your Android device.

After the setup, run the following command:

```bash
pnpm expo run:android
```

## Project Structure

- `src`: where `.ts` and `.tsx` files located.
  - `app`: route files. Each files become pages.
  - `components`: non-route files.
  - `constants`: files including contansts. (e.g. color palettes)
- `assets`: `.css`, images, etc.

## VS Code Extension Recommendations

We encourage you to use Visual Studio Code to contribute to rune.
Also using the following extensions would help you follow our code convention:

- [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)
- [PostCSS Language Support](https://marketplace.visualstudio.com/items?itemName=csstools.postcss)
- [Prettier - Code Formatter](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)
- [Tailwind CSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)
