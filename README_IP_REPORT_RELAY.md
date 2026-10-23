# BlockOps IP Report Relay 3.0.0

The miner IP Report button sends a local UDP broadcast. Routed VPNs normally do not
carry that broadcast, so the manager cannot receive it remotely without an on-site
listener.

This relay is optional. Direct rack scans and static-IP writes use routed unicast
traffic and do not need it. A layer-2 VPN that carries rack broadcasts also does
not need it.

## Setup

1. Leave one Windows PC connected to both the rack LAN and your VPN.
2. Copy the `dist` folder to that PC.
3. Run `start_ip_report_relay.bat`.
4. Enter the VPN IP of the laptop running BlockOps Static IP Manager.
5. Keep the relay window open.
6. On the manager laptop, select a rack slot and press the miner's IP Report button.

Both programs use UDP port `14235` by default. The relay preserves the miner's
original rack-side sender IP before forwarding the report over the VPN.

The VPN must also route the miner subnets to the manager laptop. Receiving the
report alone is not enough if the manager cannot reach the miner's current IP to
apply its new static address.

## Command Line

```text
BlockOps_IP_Report_Relay.exe <manager-vpn-ip> [rack-listen-port] [manager-port]
```

Example:

```text
BlockOps_IP_Report_Relay.exe 100.64.0.25
```
