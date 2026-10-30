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
6. [ ] Committed and pushed. On GitHub: enable Actions, disable upstream
   workflows (especially "Full Flutter CI"), then run "AlloDesk build".
7. [ ] Remaining branding:
   - icons in `res/` and `flutter/assets/`, from the logo face;
   - colours in `flutter/lib/common.dart` (`MyTheme.accent`, currently #0071FF);
   - the "RustDesk" strings in about 18 Flutter files;
   - bundle and binary names (`RustDesk.app`, `rustdesk.exe`, MSI product name);
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
