# iOS App Setup

The iOS target requires **Xcode 16+** on **macOS** to build and run.

## One-time setup

1. Open the KMP project in Android Studio or Fleet.
2. Run the Gradle task to generate the iOS framework:
   ```bash
   cd mobile
   ./gradlew :composeApp:linkDebugFrameworkIosSimulatorArm64
   ```
3. Open `iosApp/` in Xcode:
   - Create a new Xcode project (iOS → App) named `iosApp` inside `mobile/iosApp/`
   - Add the existing `VuxeApp.swift` and `Info.plist` to the target
   - Add the ComposeApp framework from `../composeApp/build/XCFrameworks/debug/` to the project's
     "Frameworks, Libraries, and Embedded Content"
   - Set the framework search path to `$(SRCROOT)/../composeApp/build/XCFrameworks/debug`

## Alternatively

The KMP Gradle plugin can generate an Xcode project via:
```bash
./gradlew :composeApp:iosSimulatorArm64MainBinaryFramework
```

Then open the generated `.xcodeproj` at `composeApp/build/`.

## Notes

- A physical iOS device requires an Apple Developer account and provisioning profile.
- The `iosApp/Info.plist` is a template — update `PRODUCT_BUNDLE_IDENTIFIER` in Xcode.
