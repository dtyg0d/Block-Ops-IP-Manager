# BlockOps Static IP Manager

BlockOps is a Windows rack dashboard and static-IP assignment tool for VNISH and
Bitmain miners. The default site layout maps 36 racks with 168 slots each,
starting at `10.3.1.1` and ending at `10.3.36.168`.

## What it does

- Continuously monitors every expected rack address or scans one selected rack.
- Identifies VNISH, Bitmain, generic web, HTTPS-only, and SSH-only devices.
- Keeps known-online miners visible through one transient failed cycle.
- Lets an operator click a rack slot, press the miner's physical IP Report
  button, and apply the slot's expected static address.
- Provides a separate sequential IP-assignment workspace with duplicate checks,
  wrong-subnet protection, safe dependency ordering, parking addresses, pre-checks,
  and CSV exports.
- Skips rows that are already on the correct target IP.

## v3 workflow

### Rack dashboard

1. Set the shared **Start point** in **IP assignment**. This is both the first sequential target and
   Rack 1 / Slot 1; there is no second rack-base field to keep synchronized.
2. Select **Scan all**, type a rack number and select **Scan rack**, or start the
   live monitor.
3. Click a physical slot to arm its expected address.
4. Press IP Report on that miner. The listener starts automatically and the
   captured miner is assigned through the existing safe apply engine.

### IP assignment

Use the same start point to capture miners in physical order. Build or pre-check
the safe plan, review every row, then apply. A later report received while an
apply is active is queued instead of starting a competing write batch.

## Faster monitoring

The monitor uses a bounded worker pool, short TCP reachability checks, and at
most two HTTP identification requests for a reachable web device. Apply workers
are configured separately, so making scans faster does not increase concurrent
configuration writes. Live monitoring is cancellable and reports progress in the
status bar.

## VPN behavior

Normal rack scanning is direct IP traffic and works over any VPN that routes the
miner subnets. No relay is required for scanning or for applying settings.

The physical IP Report button is different: miners usually send a rack-local UDP
broadcast, and a normal routed VPN does not forward broadcasts. If the VPN is
layer-2/bridged and forwards that traffic, the manager can receive it directly.
Otherwise, the optional `BlockOps_IP_Report_Relay.exe` can forward only those
packets from a rack-side Windows machine. See `README_IP_REPORT_RELAY.md`.

## Standard-user operation

The manager embeds a Windows `asInvoker` manifest and does not request elevation.
Settings, logs, and autosaved results are stored below:

```text
%LOCALAPPDATA%\BlockOps\Static IP Manager
```

Miner passwords are session-only and are not written to the settings file. An IT
administrator may still need to approve an inbound Windows Firewall rule for UDP
`14235` once on each workstation that receives physical IP Report packets.

## Build

Install the stable Rust toolchain, then run:

```text
build_release.bat
```

The script runs all tests, builds both binaries, and creates:

```text
dist\BlockOps_Static_IP_Manager_v3.0.0.exe
dist\BlockOps_Static_IP_Manager_v3.0.0_Windows.zip
```

The animated BlockOps intro remains embedded in the manager.

## Safety notes

- Verify one miner before applying a large batch on unfamiliar firmware.
- The app treats monitoring support and configuration-write support separately.
- A reachable device may be shown as `Other online` when its vendor endpoint is
  unavailable; that does not claim the device supports a write operation.
- Do not distribute real miner credentials in logs, screenshots, or repository
  files.
