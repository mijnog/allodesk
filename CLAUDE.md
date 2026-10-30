@AGENTS.md

# AlloDesk project (fork of RustDesk)

Rebranding RustDesk for Allo Geek (https://allo-geek.com), a French home and
remote computer-support business on the Côte Basque. The owner is new to
RustDesk. Name is undecided: "AlloDesk" or "AlloViewer" (placeholder: AlloDesk).

Fork: https://github.com/mijnog/allodesk. Only this one fork. `libs/hbb_common`
is NOT forked; all branding is applied at runtime from `src/branding.rs`.

## Decisions made

- Customers: Windows and macOS. Builds run on GitHub Actions (a Mac app can't be
  built on this Linux PC).
- Unsigned builds are fine for now (customers get SmartScreen / Gatekeeper
  warnings).
- Self-hosted server on the owner's home Linux PC (this machine), not RustDesk's
  public servers.
- The server and key are set as defaults (`DEFAULT_SETTINGS`), so they can be
  changed in Settings > Network. Consider switching to `OVERWRITE_SETTINGS`
  (locked) before handing out to customers.
- Update check: nothing to do. `is_custom_client()` (`src/common.rs`) is true
  whenever the app name != "RustDesk", which disables RustDesk's update check.

## Server setup

- Free (Freebox) fibre. Full-stack IPv4 requested on 2026-10-30: fixed public IP
  **82.67.254.214**. The old IP 88.187.196.222 was shared, and Freebox OS refused
  forwarding ports below 49152.
- DNS: `remote.allo-geek.com` A record → 82.67.254.214 in Cloudflare (DNS only,
  grey cloud). Verified working. Registrar is Namecheap; DNS is on Cloudflare.
- This PC's LAN IP: 192.168.1.84 (wired, enp7s0). Also runs Tailscale and the
  Eddie VPN; make sure the VPN doesn't capture all traffic.
- Docker compose file: `~/allodesk-server/compose.yml` (hbbs + hbbr, host
  network, data in `~/allodesk-server/data`).

## Done (not committed yet)

- `src/branding.rs`: sets APP_NAME, `custom-rendezvous-server`, and `key`.
  Hooked at the top of `load_custom_client()` in `src/common.rs`; module
  registered in `src/lib.rs`.
- `.github/workflows/allodesk-build.yml`: manual (`workflow_dispatch`) unsigned
  build for Windows x64 (.exe + .msi) and macOS x86_64 + aarch64 (.dmg).
  Publishes a prerelease. Trimmed from `flutter-build.yml`.
- `libs/hbb_common` submodule initialised.

## Next steps

1. [x] Freebox restarted; `curl -4 ifconfig.me` shows 82.67.254.214 (2026-10-30).
2. [x] Freebox port forwarding to 192.168.1.84: TCP 21115-21117, UDP 21116
   (Paramètres > Mode avancé > Gestion des ports; IP source = Toutes).
3. [x] Server running (hbbs + hbbr, started with sudo on 2026-10-30). The owner
   still has to log out and back in for docker group membership. TCP
   21115-21117 answer on remote.allo-geek.com from the LAN (hairpin).
4. [x] Server key `DfLDM9nlq0XvVF1swLjQxl34q5XXrcAdvZE5yx5MaBY=` is in
   `src/branding.rs`. The private key is chmod 600 and backed up in the owner's
   KeePass, plus `~/allodesk-key-backup.tar.gpg` (gpg-encrypted). Losing it
   breaks every distributed client.
5. [x] Tested with the stock RustDesk app: phone on 4G connected to this PC
   (2026-10-30). This PC runs ufw, which only allows the LAN; opened
   21115:21117/tcp and 21116/udp. This PC (Hyprland) can't be controlled: its
   portal has no RemoteDesktop interface. Controlling FROM it works (tested PC to
   phone over 4G).
6. [x] Pushed; first "AlloDesk build" (commit 727cd33, 2026-10-30) produced
   Windows `.exe` + `.msi` and Intel Mac `.dmg` in the `nightly` prerelease:
   https://github.com/mijnog/allodesk/releases/tag/nightly . The Apple Silicon job
   was cancelled in ~1s because GitHub is retiring the `macos-14` runner image (brownouts; retired 2026-11-02). Fixed locally by switching that job to `macos-15` (uncommitted). Those files predate icons,
   colours, dark default and name changes. TODO: re-run "AlloDesk build" on the latest
   master, install the .exe/.msi in the Windows VM, check: name, server pre-set
   ("Ready" without touching settings), dark default, icons, Connect button colour,
   MSI shortcuts/folder say AlloDesk. Disable "Full Flutter CI" on GitHub.
7. [ ] Remaining branding:
   - DONE locally, not committed: icons (cropped face from the website logo;
     `res/*.png|ico`, `flutter/macos/Runner/AppIcon.icns`, Windows `app_icon.ico`,
     `flutter/assets/icon.png`). `flutter/assets/icon.png` is gitignored by `*png`:
     use `git add -f`. Swap in the owner's original logo
     files when available. `res/mac-tray-*.png` (macOS menu bar) NOT replaced yet;
   - DECIDED: no home-screen logo (deleted `logo_light/dark.png`; icon only);
     default theme dark via `DEFAULT_LOCAL_SETTINGS` in `src/branding.rs` (users can
     still switch; "system" is saved explicitly for custom builds). Not yet
     compile-checked or seen in a real build.
   - Preview page (mock-up, link-shared): https://claude.ai/artifact/8rag2LFHqM4mBrwU4JnnWF
   - DONE locally (flutter/lib/common.dart): light accent/button = mid green
     #1D7E47 (`MyTheme.accent`, `button`). Dark theme: the Connect/elevated button
     fill = deep green #0C5E2F (`MyTheme.accentDark`); colour scheme primary and
     secondary stay mid green (deep green is only ~1.9:1 on the dark background, so
     unsuitable for text, switches, outlines). Owner chose mid-light / deep-dark;
     outlined buttons are neutral grey in the real app; `MyTheme.idColor` (blue) left as is;
   - DONE locally (uncommitted): tab-bar title uses `bind.mainGetAppNameSync()`;
     About page Website link -> allo-geek.com and its blue box -> `MyTheme.accent`
     (copyright line kept: AGPL); Windows `Runner.rc` ProductName/FileDescription;
     macOS `Info.plist` CFBundleName/DisplayName = AlloDesk; MSI step in
     `allodesk-build.yml` renames exe to AlloDesk.exe and passes `--app-name AlloDesk`.
     Translated text already swaps "RustDesk" -> app name at runtime (`src/lang.rs`).
   - NOT changed on purpose: file names `rustdesk.exe` / `RustDesk.app`
     (`PRODUCT_NAME` in AppInfo.xcconfig; many scripts depend on them);
     About "Privacy Statement" link removed on desktop (owner has no privacy page;
     the site is static). Still pointing at rustdesk.com/privacy.html: the Windows
     installer page's "End-user license agreement" link
     (`desktop/pages/install_page.dart`) and the mobile settings page. Decide whether
     to write an Allo Geek privacy/terms page and repoint them;
   - `PRIVACY.md` (French draft): identity (micro-entreprise, SIRET, info@allo-geek.com)
     and date filled 2026-10-30. Still [À CONFIRMER]: no session recording; relay only
     forwards encrypted data; no telemetry (check on final build). Promises: server
     logs 30 days max (`~/allodesk-server/rotate-logs.sh`, monthly user timer
     `allodesk-log-rotate.timer`, size caps in compose.yml; owner must enable it) and
     device records 30 days max (owner's decision). TODO: build the purge for device records (peer table
     has `created_at` only, no last-seen; delete rows older than 30 days by
     created_at, ideally inside `rotate-logs.sh` so one monthly job covers logs and records, after testing that an active client re-registers). Possibly add the
     owner's own name to the policy (identity of controller), and publish the page.
   - maybe French by default, and phone number or tagline on the home screen.
8. [ ] Optional: local Linux builds. Needs `sudo pacman -S --needed cmake nasm
   yasm ninja xdotool libappindicator-gtk3`, Flutter 3.24.5 in `~/flutter`,
   vcpkg at commit 9e593bb1 in `~/vcpkg`, and the flutter_rust_bridge 1.80.1
   codegen. The agent was blocked from downloading these; the owner runs them.

## Brand assets (from allo-geek.com)

- Logo: https://allo-geek.com/assets/logo.png (1083x706 RGBA: geek with a green
  beret and headset, "ALLO" in dark green, "GEEK" in red). Ask the owner for
  original/SVG files, or crop the face for the icon.
- Colours:
  - green #2E9E5F;
  - green-dark #0C5E2F;
  - green-deep #0A6B2C;
  - green-mid #1D7E47;
  - green-pale #EDF6EE;
  - red #AC2B22;
  - text-dark #24262A.
- Font: Manrope. Tagline: "On dépanne • On explique • On protège". Phone: +33 7 49 25 89 72.
