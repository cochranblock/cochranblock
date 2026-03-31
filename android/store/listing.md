# Google Play Store Listing — CochranBlock

## App Name
CochranBlock

## Short Description (80 chars max)
Your entire portfolio runs offline in a 6 MB app. Zero cloud. Pure Rust.

## Full Description (4000 chars max)
CochranBlock is a zero-cloud portfolio and business site running as a native Android app. The entire site — 22 pages, native search, source code viewer, analytics dashboard, DCAA audit trail, government documents — runs offline from a single Rust binary.

No internet required. No data collection. No tracking. No ads. No permissions.

FEATURES:
• 22 pages of business content — services, products, government documents, cost analysis
• Native full-text search — find anything across all pages instantly
• Source code viewer — read the Rust code that powers the app
• Binary size leaderboard — 16 projects measured from 48 KB to 51 MB
• Speed comparison — 240x lighter than Wix, zero JavaScript
• Open Books IR&D audit — live hours calculated from GitHub commits
• Privacy policy built in — zero data collection by design

TECHNICAL DETAILS:
• Built in Rust — memory-safe, compiled, no runtime interpreter
• Axum web framework serving localhost via WebView
• All assets embedded at compile time (zstd compressed)
• SQLite for local data persistence
• 9.5 MB native binary, 6.2 MB APK
• API 35, minimum SDK 28

This app is the live demo of CochranBlock's zero-cloud architecture. The same code that serves cochranblock.org on a $10/month laptop now runs on your phone.

Built by Michael Cochran — Army veteran (17C Cyber Operations), 13 years defense and enterprise. All source code public at github.com/cochranblock.

## Category
Business

## Content Rating
Everyone

## Contact Email
mcochran@cochranblock.org

## Privacy Policy URL
https://cochranblock.org/privacy

## Website
https://cochranblock.org

---

## Files to Upload

### App Icon (512x512)
android/store/icons/icon-512.png

### Feature Graphic (1024x500)
android/store/icons/feature-graphic.png
NOTE: Replace with proper dark bg + centered logo (current is stretched fallback)

### Screenshots (minimum 2, Pixel 9 Pro XL resolution)
android/store/screenshots/01-home.png — Home page with hero
android/store/screenshots/02-current.png — Navigation/content view
android/store/screenshots/03-app.png — App overview

### APK / AAB
android/app/build/outputs/bundle/release/app-release.aab (6.1 MB)
android/app/build/outputs/apk/release/app-release.apk (6.2 MB)

### Signing
Keystore: android/cochranblock-release.jks (NOT in git)
