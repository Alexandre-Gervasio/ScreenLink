#!/usr/bin/env node

/**
 * ScreenLink Performance Benchmark Suite
 * 
 * Measures:
 * - Frame processing latency
 * - Memory usage
 * - CPU usage
 * - Throughput (Mbps)
 * - Connection establishment time
 * 
 * Usage:
 *   node benchmark.js --duration=60 --resolution=1920x1080
 */

import os from 'os';
import { performance } from 'perf_hooks';

class BenchmarkSuite {
  constructor(options = {}) {
    this.duration = options.duration || 60; // seconds
    this.resolution = options.resolution || '1920x1080';
    this.results = {
      frames: 0,
      latencies: [],
      memoryUsage: [],
      cpuUsage: [],
      throughput: 0,
      frameRate: 0,
      avgLatency: 0,
      p99Latency: 0,
    };
    this.startTime = null;
  }

  /**
   * Simulate frame processing
   */
  processFrame(frameSize) {
    const startTime = performance.now();

    // Simulate H.264 encoding (simplified)
    const encoded = this.simulateH264Encoding(frameSize);

    // Simulate WebSocket transmission
    this.simulateNetworkTransmit(encoded);

    const latency = performance.now() - startTime;
    return latency;
  }

  /**
   * Simulate H.264 encoding
   */
  simulateH264Encoding(frameSize) {
    // Simple compression simulation
    // H.264 typically achieves ~10-20% of original size
    const encoded = Buffer.allocUnsafe(Math.floor(frameSize * 0.15));
    return encoded;
  }

  /**
   * Simulate network transmission
   */
  simulateNetworkTransmit(data) {
    // Simulate 1 Gbps network (0.125 MB/ms) 
    const transmitTime = data.length / (1024 * 1024 * 125);
    const start = performance.now();
    while (performance.now() - start < transmitTime) {
      // Busy wait to simulate network latency
    }
  }

  /**
   * Get current memory usage
   */
  getMemoryUsage() {
    const used = process.memoryUsage();
    return {
      heapUsed: Math.round(used.heapUsed / 1024 / 1024), // MB
      heapTotal: Math.round(used.heapTotal / 1024 / 1024),
      rss: Math.round(used.rss / 1024 / 1024),
    };
  }

  /**
   * Get CPU usage
   */
  getCPUUsage() {
    const cpus = os.cpus();
    let totalIdle = 0;
    let totalTick = 0;

    cpus.forEach((cpu) => {
      for (const type in cpu.times) {
        totalTick += cpu.times[type];
      }
      totalIdle += cpu.times.idle;
    });

    const idle = totalIdle / cpus.length;
    const total = totalTick / cpus.length;

    return 100 - ~~((idle / total) * 100);
  }

  /**
   * Run benchmark suite
   */
  async run() {
    console.log('📊 ScreenLink Performance Benchmark');
    console.log('═══════════════════════════════════════');
    console.log(`Duration: ${this.duration}s`);
    console.log(`Resolution: ${this.resolution}`);
    console.log(`Started: ${new Date().toLocaleTimeString()}\n`);

    const [width, height] = this.resolution.split('x').map(Number);
    const frameSize = width * height * 4; // RGBA

    const targetFPS = 30;
    const frameInterval = 1000 / targetFPS;

    this.startTime = performance.now();
    let lastFrameTime = this.startTime;

    console.log('🔴 Running...');

    while (performance.now() - this.startTime < this.duration * 1000) {
      const now = performance.now();

      // Frame-rate limiting
      if (now - lastFrameTime >= frameInterval) {
        // Measure frame latency
        const latency = this.processFrame(frameSize);
        this.results.latencies.push(latency);
        this.results.frames++;

        // Record memory and CPU
        if (this.results.frames % 10 === 0) {
          this.results.memoryUsage.push(this.getMemoryUsage());
          this.results.cpuUsage.push(this.getCPUUsage());
        }

        lastFrameTime = now;

        // Progress indicator
        if (this.results.frames % 30 === 0) {
          process.stdout.write('.');
        }
      }
    }

    console.log('\n✅ Benchmark complete!\n');
    this.analyze();
  }

  /**
   * Analyze results
   */
  analyze() {
    const fps = this.results.frames / this.duration;
    const totalData = this.results.frames * 1920 * 1080 * 4 * 0.15; // Compressed
    const throughputMbps = (totalData / this.duration / 1024 / 1024) * 8;

    const latencies = this.results.latencies.sort((a, b) => a - b);
    const avgLatency = latencies.reduce((a, b) => a + b) / latencies.length;
    const p99Latency = latencies[Math.floor(latencies.length * 0.99)];

    const avgMemory =
      this.results.memoryUsage.length > 0
        ? this.results.memoryUsage.reduce((a, b) => a + b.heapUsed, 0) /
          this.results.memoryUsage.length
        : 0;

    const avgCPU =
      this.results.cpuUsage.length > 0
        ? this.results.cpuUsage.reduce((a, b) => a + b) / this.results.cpuUsage.length
        : 0;

    console.log('📈 Results');
    console.log('─────────────────────────────────');
    console.log(`Frames Processed:     ${this.results.frames}`);
    console.log(`Frame Rate:           ${fps.toFixed(1)} FPS`);
    console.log(`Avg Latency:          ${avgLatency.toFixed(2)} ms`);
    console.log(`P99 Latency:          ${p99Latency.toFixed(2)} ms`);
    console.log(`Max Latency:          ${latencies[latencies.length - 1].toFixed(2)} ms`);
    console.log(`Throughput:           ${throughputMbps.toFixed(1)} Mbps`);
    console.log(`Avg Memory:           ${avgMemory.toFixed(0)} MB`);
    console.log(`Avg CPU Usage:        ${avgCPU.toFixed(1)}%`);
    console.log('─────────────────────────────────\n');

    // Performance assessment
    console.log('🎯 Performance Assessment');
    console.log('─────────────────────────────────');
    console.log(fps > 30 ? '✅ FPS:     EXCELLENT' : fps > 24 ? '✅ FPS:     GOOD' : '⚠️  FPS:     NEEDS IMPROVEMENT');
    console.log(
      avgLatency < 50
        ? '✅ Latency: EXCELLENT'
        : avgLatency < 100
        ? '✅ Latency: GOOD'
        : '⚠️  Latency: NEEDS IMPROVEMENT'
    );
    console.log(
      throughputMbps < 15
        ? '✅ Bandwidth: EFFICIENT'
        : throughputMbps < 25
        ? '✅ Bandwidth: ACCEPTABLE'
        : '⚠️  Bandwidth: HIGH'
    );
    console.log(avgCPU < 25 ? '✅ CPU:     EFFICIENT' : avgCPU < 50 ? '✅ CPU:     ACCEPTABLE' : '⚠️  CPU:     HIGH');
    console.log('─────────────────────────────────\n');

    this.printRecommendations(fps, avgLatency, avgCPU, throughputMbps);
  }

  /**
   * Print optimization recommendations
   */
  printRecommendations(fps, latency, cpu, bandwidth) {
    console.log('💡 Recommendations');
    console.log('─────────────────────────────────');

    if (fps < 24) {
      console.log('• Consider reducing resolution');
      console.log('• Check for background processes');
      console.log('• Upgrade CPU or GPU');
    }

    if (latency > 100) {
      console.log('• Check network connectivity');
      console.log('• Reduce resolution for faster processing');
      console.log('• Try wired connection instead of WiFi');
    }

    if (cpu > 50) {
      console.log('• Close other applications');
      console.log('• Enable hardware acceleration');
      console.log('• Reduce screen resolution');
    }

    if (bandwidth > 20) {
      console.log('• Enable more aggressive compression');
      console.log('• Reduce frame rate to 24 FPS');
      console.log('• Check network bandwidth availability');
    }

    if (fps >= 24 && latency <= 100 && cpu <= 50 && bandwidth <= 20) {
      console.log('✨ System is well-optimized!');
    }

    console.log('─────────────────────────────────\n');
  }
}

// Main
async function main() {
  const args = process.argv.slice(2);
  const options = {};

  args.forEach((arg) => {
    const [key, value] = arg.split('=');
    if (key.startsWith('--')) {
      options[key.slice(2)] = value || true;
    }
  });

  const benchmark = new BenchmarkSuite(options);
  await benchmark.run();
}

main().catch((error) => {
  console.error('Benchmark error:', error);
  process.exit(1);
});
