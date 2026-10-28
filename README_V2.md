# BlockOps Static IP Manager 3.0.1

Version 3 is a reliability and workflow rebuild of the rack monitor and static-IP
manager.

## Highlights

- The default layout is 36 racks with 168 slots, labeled Rack 1 through Rack 36.
- Previous 19-rack settings upgrade to 36 racks; other custom counts are retained.
- The rack map always starts at `10.3.1.1`; sequential assignment has its own start point.
- The duplicate `Rack 1, Slot 1 IP` setting was removed.
- Clear assignments starts a fresh queue without restarting the app or losing
  previous autosaved results. It is disabled during an active apply batch.
- Direct scans over routed VPN connections do not require a relay.
- Monitoring uses a fixed worker pool instead of creating one operating-system
  thread per target.
- A reachable web miner receives no more than two short vendor probes per cycle.
- Scan concurrency is independent from static-IP apply concurrency.
- Scan progress and elapsed time are reported in the interface.
- Known vendor identity survives a later generic web response.
- A known-online miner must fail two monitor cycles before it is marked offline.
- Start/stop listener state and background jobs use typed events instead of
  parsing status strings.
- Pre-check results now reach the correct assignment rows.
- Successful writes update the visible current IP immediately.
- Settings and session output use the signed-in user's local app-data directory.
- Non-sensitive site settings persist; miner credentials remain session-only.
- The Windows executable explicitly runs `asInvoker` without requesting admin.
- The animated inverted BlockOps intro opens in a dark frameless splash without
  a loading footer.

## Optional IP Report relay

The manager does not need a relay for scans or configuration writes. The relay is
only for physical IP Report broadcasts when a routed VPN does not carry rack-local
broadcast traffic. Layer-2 VPN users can receive those packets directly.

## Build output

Run `build_release.bat` to test, build, and package the manager and optional relay.
