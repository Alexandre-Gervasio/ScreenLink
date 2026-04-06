# Performance Benchmarking & Optimization

Complete guide to benchmarking and optimizing ScreenLink performance.

## Table of Contents

- [Running Benchmarks](#running-benchmarks)
- [Understanding Metrics](#understanding-metrics)
- [Performance Targets](#performance-targets)
- [Optimization Guide](#optimization-guide)
- [Network Tuning](#network-tuning)
- [Hardware Recommendations](#hardware-recommendations)

---

## Running Benchmarks

### Quick Start

```bash
# Run default benchmark (60 seconds, 1920x1080)
node benchmarks/run-benchmark.js

# Custom configuration
node benchmarks/run-benchmark.js --duration=120 --resolution=3840x2160
```

### Benchmark Options

| Option | Default | Description |
|--------|---------|-------------|
| `--duration` | 60 | Benchmark duration in seconds |
| `--resolution` | 1920x1080 | Display resolution (WxH) |
| `--fps` | 30 | Target frame rate |

### Interpreting Results

```
📊 Results
─────────────────────────────────
Frames Processed:     1800
Frame Rate:           30.0 FPS          ← Actual achieved FPS
Avg Latency:          45.23 ms          ← Average frame latency
P99 Latency:          62.15 ms          ← 99th percentile
Max Latency:          156.42 ms         ← Worst case
Throughput:           12.5 Mbps         ← Network bandwidth used
Avg Memory:           145 MB            ← RAM consumption
Avg CPU Usage:        18.5%             ← CPU load
```

---

## Understanding Metrics

### Frame Rate (FPS)
**What it is**: Frames per second being processed and transmitted.

**Target**: ≥24 FPS (smooth motion), ideally 30-60 FPS

**Factors affecting it**:
- CPU speed and cores
- GPU acceleration
- Resolution
- Available bandwidth
- System load

**How to improve**:
```bash
# Lower resolution
Resolution: 1920x1080 → 1280x720

# Reduce frame rate
--fps 30 → --fps 24

# Close background apps (check Task Manager)
# Enable GPU acceleration (Settings)
```

### Latency (ms)
**What it is**: Time from capture to display (round-trip).

**Target**: <100ms typical, <50ms ideal on LAN

**Factors affecting it**:
- Network delay (ping)
- Encoding/decoding time
- Screen capture efficiency
- Driver overhead

**How to improve**:
```bash
# Use Ethernet instead of WiFi
# Reduce resolution (faster encoding)
# Update GPU drivers
# Close network-heavy applications
```

### Throughput (Mbps)
**What it is**: Network bandwidth consumed.

**Target**: 5-20 Mbps, ideally <15 Mbps

**Factors affecting it**:
- Resolution
- Frame rate
- Video codec efficiency (H.264)
- Compression ratio

**How to improve**:
```bash
# Lower resolution
# Reduce FPS
# Enable more aggressive compression
# Use wired connection for consistency
```

### Memory Usage (MB)
**What it is**: RAM consumed by ScreenLink process.

**Target**: <200MB per instance

**Factors affecting it**:
- Resolution (frame buffers)
- Number of connections
- Buffering strategy

**How to improve**:
```bash
# Lower resolution
# Reduce number of concurrent connections
# Enable memory optimization mode
```

### CPU Usage (%)
**What it is**: Processor load for encoding/decoding.

**Target**: <30%, ideally <20%

**Factors affecting it**:
- Resolution
- Frame rate
- Codec complexity (H.264)
- Hardware acceleration availability

**How to improve**:
```bash
# Enable hardware acceleration (GPU)
# Reduce resolution or FPS
# Close background applications
# Update system drivers
```

---

## Performance Targets

### Minimum Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **CPU** | Intel i3 / AMD Ryzen 3 | Intel i5+ / Ryzen 5+ |
| **RAM** | 2GB | 4GB+ |
| **GPU** | Integrated | Dedicated (optional) |
| **Network** | 10 Mbps LAN | 100 Mbps+ LAN |
| **Storage** | - | 100MB for binary |

### Performance Tiers

#### Tier 1: Light Use (Web browsing)
```
Resolution:     1280x720
FPS:           24
Latency:       <100ms
Bandwidth:     5-8 Mbps
CPU Usage:     <15%
Memory:        <100MB
Devices:       Laptop + Budget PC
```

#### Tier 2: Balanced (Development)
```
Resolution:     1920x1080
FPS:           30
Latency:       <50ms
Bandwidth:     10-15 Mbps
CPU Usage:     <25%
Memory:        <150MB
Devices:       Modern laptop + Desktop
```

#### Tier 3: High Performance (Gaming)
```
Resolution:     2560x1440 or 4K
FPS:           60
Latency:       <30ms
Bandwidth:     15-30 Mbps
CPU Usage:     <40%
Memory:        <200MB
Devices:       Gaming PC + High-end GPU
```

---

## Optimization Guide

### Software Optimization

#### 1. Resolution Strategy

```bash
# For most users: 1920x1080 @ 30 FPS
# For laptops: 1280x720 @ 24 FPS
# For fast networks: 2560x1440 @ 30 FPS

# Calculate bandwidth: (W × H × 4 × 8 × FPS) × 0.15
# 1920×1080 × 4 × 8 × 30 × 0.15 = 12.4 Mbps
```

#### 2. Frame Rate Tuning

| FPS | Use Case | Features |
|-----|----------|----------|
| 24 | Web browsing, office work | Smooth scrolling, lower bandwidth |
| 30 | Standard development | 30 fps video standard |
| 60 | Gaming, video editing | Smooth motion, high bandwidth |

#### 3. Codec Optimization

- **H.264**: Default, balanced quality/speed
- **VP9**: Better compression, more CPU
- **AV1**: Best compression, very demanding

#### 4. Buffer Management

```javascript
// Optimal buffer sizes
videoBufferSize: 500 * 1024,  // 500KB
audioBufferSize: 50 * 1024,   // 50KB
maxBufferDelay: 200           // 200ms
```

### Hardware Acceleration

#### Enable in Windows
```
Settings > System > Display > Advanced display settings
  > GPU preferences > ScreenLink > Options
  > Select GPU-accelerated
```

#### Enable on macOS
```
System Preferences > Energy Saver
  Disable "Enable Power Nap"
  Enable graphics acceleration
```

#### Enable on Linux
```bash
# Check GPU driver
lspci | grep VGA
# Install appropriate drivers

# Or enable VAAPI:
export LIBVA_DRIVER_NAME=i965
./screenlink
```

### Network Optimization

#### Bandwidth Monitoring
```bash
# Monitor during capture
# LAN: Should be <20 Mbps
# WAN: Should be <5 Mbps (not implemented yet)

# Check actual usage
iftop -n
# or
nethogs ScreenLink
```

#### Quality Adaptation

```javascript
// Pseudo-code for adaptive bitrate
if (packetLoss > 5%) {
  quality = 'low';     // More aggressive compression
  fps = 24;
} else if (bandwidth < 10) {
  quality = 'medium';  // Balanced
  fps = 30;
} else {
  quality = 'high';    // Less compression
  fps = 60;
}
```

---

## Network Tuning

### System Level (macOS)

```bash
# Increase socket buffer sizes
sudo sysctl -w net.inet.tcp.sndbuf=2517792
sudo sysctl -w net.inet.tcp.rcvbuf=2517792

# Persist changes
echo "net.inet.tcp.sndbuf=2517792" | sudo tee -a /etc/sysctl.conf
```

### System Level (Linux)

```bash
# Optimize TCP window size
sudo sysctl -w net.core.rmem_max=134217728
sudo sysctl -w net.core.wmem_max=134217728
sudo sysctl -w net.ipv4.tcp_rmem="4096 87380 134217728"
sudo sysctl -w net.ipv4.tcp_wmem="4096 65536 134217728"
```

### System Level (Windows)

```powershell
# Run as Administrator
netsh int tcp set global autotuninglevel=normal
netsh int tcp set global ecncapability=enabled
```

### Application Level

```javascript
// WebSocket buffer optimization
const ws = new WebSocket(url);
ws.binaryType = 'arraybuffer';  // Not 'blob'
ws.bufferedAmount;               // Monitor buffering
```

---

## Hardware Recommendations

### Minimum Setup

```
Primary PC:       Laptop (MacBook Air, ThinkPad X1)
Secondary PC:     Budget desktop or mini-PC
Network:          100 Mbps Ethernet
```

**Performance**: 1280x720 @ 24 FPS, ~5 Mbps

### Recommended Setup

```
Primary PC:       Modern laptop (MacBook Pro, Dell XPS)
Secondary PC:     Desktop with dedicated GPU
Network:          Gigabit Ethernet (1000 Mbps)
GPU:              NVIDIA GTX 1050+ / RTX 3050+
```

**Performance**: 1920x1080 @ 30 FPS, ~12 Mbps

### High-End Setup

```
Primary PC:       High-end workstation
Secondary PC:     Gaming PC with RTX GPU
Network:          Dedicated 10 Gbps connection
GPU:              NVIDIA RTX 3070+ or A6000
```

**Performance**: 4K @ 60 FPS, ~25 Mbps

---

## Common Issues & Solutions

### High Latency (>200ms)

**Check:**
1. Network connection: `ping 192.168.1.x`
2. WiFi signal strength: Should be >-67 dBm
3. Other network traffic: Close bandwidth hogs
4. Physical distance: Move closer to router

**Solution:**
```bash
# Switch to Ethernet
# Or improve WiFi: move router, reduce interference
# Reduce resolution: 1920x1080 → 1280x720
```

### High CPU Usage (>60%)

**Check:**
1. Background processes: Open Task Manager/Activity Monitor
2. GPU acceleration: Is it enabled?
3. Driver version: Update graphics drivers

**Solution:**
```bash
# Enable GPU acceleration
# Close background apps
# Lower resolution
# Reduce FPS: 30 → 24
```

### High Memory Usage (>500MB)

**Check:**
1. Memory leaks in encoding
2. Buffer management
3. Number of connections

**Solution:**
```bash
# Restart application
# Monitor with: free -h (Linux) / Activity Monitor (macOS)
# Report memory leak to GitHub issues
```

### Frame Drops / Stuttering

**Check:**
1. CPU/GPU load
2. Network packet loss
3. Thermal throttling (laptop)

**Solution:**
```bash
# Monitor: top -p $(pgrep -f screenlink)
# Check thermals: watch -n1 sensors
# Use charger and ventilation pads
# Reduce quality settings
```

---

## Profiling & Debugging

### Enable Performance Logging

```bash
# Backend
DEBUG=performance npm run dev:backend

# Frontend (in browser console)
localStorage.debug = 'screenlink:*'
```

### Real-time Monitoring

```bash
# CPU and Memory
watch -n 1 'ps aux | grep screenlink'

# Network
sudo nethogs -d 1 screenlink

# Disk I/O
iotop -p $(pgrep -f screenlink)
```

### Performance Profiling

```javascript
// In Node.js backend
import { performance } from 'perf_hooks';

const start = performance.now();
// ... operation ...
const duration = performance.now() - start;
console.log(`Operation took ${duration.toFixed(2)}ms`);
```

---

## Advanced Tuning

### Video Encoder Settings

```bash
# H.264 preset
# ultrafast < superfast < veryfast < faster < fast < medium < slow (default)
--h264-preset faster

# Bitrate control
# 0-51 scale (lower = better quality, more bandwidth)
--h264-crf 28  # Default, balanced
--h264-crf 23  # Higher quality, more bandwidth
--h264-crf 35  # Lower quality, less bandwidth
```

### Network Socket Tuning

```javascript
// Set TCP_NODELAY for low-latency
socket.setNoDelay(true);
socket.setKeepAlive(true, 30000);

// Adjust buffer sizes
socket.setMaxListeners(100);
```

---

**Last Updated**: April 6, 2026
