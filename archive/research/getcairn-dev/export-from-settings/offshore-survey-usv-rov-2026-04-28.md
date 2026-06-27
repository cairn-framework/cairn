# Offshore Survey USV-ROV

**Version:** 0.1.0
**Schema:** 1.0.0

## System Overview

The system is a hybrid diesel-electric Unmanned Surface Vessel (USV) and tethered light work-class Remotely Operated Vehicle (ROV), operated in concert from a shore-based Remote Operations Centre (ROC) for offshore benthic habitat and environmental survey missions at ranges of 50–200 km from shore. The USV serves as the primary sea-surface platform, providing propulsion, power generation, a stern-ramp Launch and Recovery System (LARS) for the ROV, and a communications relay node between the ROV and the ROC. The ROC provides all mission planning, supervisory control, and payload data reception. The USV operates under supervised autonomy — executing operator-defined waypoints while autonomously managing station-keeping, collision avoidance, and fault responses — with the primary long-range link to the ROC provided by a LEO satellite constellation (e.g. Starlink or Iridium) at approximately 40–80 ms latency. The system is subject to full classification society oversight [assumed: DNV or Lloyd's Register unmanned vessel notation], which governs the rigour of the collision avoidance system, safety case, and design documentation programme.

The ROV is a light work-class vehicle (50–200 kg) deployed via the stern ramp on a fibre-optic management tether [assumed: tether carries high-voltage power and bidirectional data], capable of operating to depths of 300–1000 m. Its primary mission payload is seabed habitat and benthic survey, comprising high-definition video cameras, multibeam or imaging sonar, and sediment sampling equipment [specific payload configuration to be confirmed]. Onboard edge processing on the USV compresses and prioritises video and sonar data before uplink to the ROC, managing the bandwidth constraints of the satellite link. The USV is designed to transit in sea states up to Sea State 6 (Hs ~6 m), with ROV launch and recovery via the stern ramp restricted to Sea State 4 (Hs ~2.5 m) or below. In the event of a communications loss, both vehicles enter a hold state — the USV maintains station and the ROV holds depth — awaiting link restoration up to a defined timeout, after which both vehicles execute an autonomous return-to-surface and home sequence.

The USV power architecture is hybrid diesel-electric with significant battery capacity, sized to supply both vessel hotel loads and ROV tether power simultaneously [assumed: 5–20 kW delivered to ROV at operating depth via high-voltage DC tether]. Three architectural parameters remain open and will shape detailed design: (1) mission endurance per deployment, which drives fuel tankage, battery sizing, and autonomous energy management requirements; (2) the classification society scope — whether it covers the USV only or extends to the LARS, ROV, and ROC cybersecurity architecture; and (3) the operational logistics model — whether the USV operates from a fixed shore base, a support vessel, or a hybrid of both — which determines transit endurance requirements and the degree of autonomous self-sufficiency required between deployments.

## Architecture

### USV Platform

Provides the sea-surface vessel hull, propulsion, and station-keeping capability for the system. The USV is a hybrid diesel-electric vessel designed to transit in sea states up to SS6 and maintain station during ROV operations. It interfaces with the Power Generation, ROV, LARS, and Communications subsystems as the primary physical host platform.

> **Type:** subsystem · **ID:** SUB-USV

### Power Generation & Distribution

Generates, stores, and distributes electrical power for all USV and ROV loads via a hybrid diesel-electric architecture. The subsystem comprises diesel generators, a significant battery bank, and a high-voltage DC distribution bus that supplies both vessel hotel loads and the ROV tether power feed (5–20 kW). It interfaces with the USV Platform for mechanical mounting, the ROV via the tether power conductor, and the Autonomy & Control subsystem for energy management commands.

> **Type:** subsystem · **ID:** SUB-PWR

**Interfaces:**

| Interface | From → To | Protocol | Signals |
| --- | --- | --- | --- |
| ROV Tether Power & Data | Power Generation & Distribution → ROV Vehicle | HVDC power + fibre-optic data | tether_hvdc_power (out), rov_video_sonar_data (in), rov_control_commands (out), rov_telemetry (in) |

### ROV Vehicle

Provides the subsea vehicle capable of operating to 300–1000 m depth for benthic habitat and environmental survey missions. The ROV is a light work-class vehicle (50–200 kg) equipped with HD cameras, multibeam/imaging sonar, and sediment sampling equipment, receiving power and exchanging data via a fibre-optic management tether. It interfaces with the LARS subsystem for launch and recovery, the Power Generation subsystem via the tether HVDC feed, and the Mission Payload & Data subsystem for sensor data upward and control commands downward.

> **Type:** subsystem · **ID:** SUB-ROV

**Interfaces:**

| Interface | From → To | Protocol | Signals |
| --- | --- | --- | --- |
| ROV Tether Power & Data | Power Generation & Distribution → ROV Vehicle | HVDC power + fibre-optic data | tether_hvdc_power (out), rov_video_sonar_data (in), rov_control_commands (out), rov_telemetry (in) |

### Launch & Recovery System

Provides the stern-ramp mechanism for safe deployment and recovery of the ROV in sea states up to SS4 (Hs ~2.5 m). The LARS includes the stern ramp structure, winch, tether management system (TMS), and motion compensation or A-frame as required to protect the ROV during water entry and exit. It interfaces with the USV Platform structurally, the ROV Vehicle physically during handling, and the Autonomy & Control subsystem for launch/recovery sequencing commands.

> **Type:** subsystem · **ID:** SUB-LARS

### Communications & Data Link

Provides all long-range and local communications links, acting as the relay node between the ROV, USV, and the shore-based ROC. The primary long-range link uses a LEO satellite constellation (Starlink/Iridium) at 40–80 ms latency; secondary/backup links (e.g. 4G/LTE or VSAT) provide redundancy. It interfaces with the Mission Payload & Data subsystem to receive compressed video and sonar data for uplink, and with the Autonomy & Control subsystem to relay operator commands and telemetry.

> **Type:** subsystem · **ID:** SUB-COMMS

**Interfaces:**

| Interface | From → To | Protocol | Signals |
| --- | --- | --- | --- |
| Processed Payload & Command Relay | Autonomy, Control & Mission Payload → Communications & Data Link | Ethernet / IP | compressed_video_stream (out), compressed_sonar_data (out), usv_rov_telemetry (out), roc_operator_commands (in) |

### Autonomy, Control & Mission Payload

Provides supervised autonomy for USV navigation, station-keeping, collision avoidance, fault management, and onboard edge processing of survey payload data. This subsystem executes operator-defined waypoints, manages autonomous hold and return-to-home sequences on comms loss, and compresses/prioritises video and sonar data to fit within satellite link bandwidth constraints. It interfaces with all other subsystems: commanding propulsion and LARS, receiving sensor and payload data from the ROV, and exchanging telemetry and mission data with the ROC via the Communications subsystem.

> **Type:** subsystem · **ID:** SUB-AUTO

**Interfaces:**

| Interface | From → To | Protocol | Signals |
| --- | --- | --- | --- |
| Processed Payload & Command Relay | Autonomy, Control & Mission Payload → Communications & Data Link | Ethernet / IP | compressed_video_stream (out), compressed_sonar_data (out), usv_rov_telemetry (out), roc_operator_commands (in) |

## Requirements

### Offshore Survey USV-ROV

| ID | Title | Type | Priority |
| --- | --- | --- | --- |
| REQ-001 | USV Transit Speed | performance | must |
| REQ-002 | ROV Operating Depth | performance | must |
| REQ-003 | LARS Sea State Operational Limit | environmental | must |
| REQ-004 | Communications Loss Hold and Return-to-Home Response | functional | must |
| REQ-005 | COLREGS-Compliant Collision Avoidance Detection Range | safety | must |
| REQ-006 | Onboard Edge Processing Compression for Satellite Uplink | performance | must |

### Launch & Recovery System

| ID | Title | Type | Priority |
| --- | --- | --- | --- |
| REQ-007 | LARS Winch Rated Safe Working Load | performance | must |
| REQ-008 | LARS Motion Compensation Residual Velocity Limit | performance | must |
| REQ-009 | Tether Management System Minimum Usable Tether Length | functional | must |

## Verification

| Requirement | Method | Status | Description |
| --- | --- | --- | --- |
| REQ-001 (USV Transit Speed) | test | draft | Hahaha |
