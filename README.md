# Network Monitor

A **Network Monitor** tool that captures and analyzes network traffic, providing real-time statistics, anomaly detection, and historical data visualization. The backend is written in **Rust**, ensuring high performance and scalability, while the user interface is built with **SvelteKit**, delivering a responsive and interactive experience.

## Features

### Core Functionality
- **Traffic Capture**: Monitor incoming and outgoing network traffic with detailed protocol filtering.
- **Real-Time Statistics**: View bandwidth usage, connection status, latency, and packet loss rates.
- **Anomaly Detection**: Flag unusual traffic patterns and notify users of potential issues.
- **Historical Data Analysis**: Store and visualize traffic logs over time, with export options.
- **Customizable Filters**: Set traffic filters and alert thresholds.

### Dashboard Components
- **Real-Time Overview**: Dynamic charts and tables showing current traffic metrics.
- **Anomaly Panel**: Logs of flagged incidents with severity levels.
- **Network Map**: Visual representation of active connections.
- **Settings**: Configuration for alerts, filters, and preferences.

## Tech Stack

### Backend
- **Rust**
  - Provides a high-performance, memory-safe environment for network traffic processing.
  - Uses libraries like `tokio` for async I/O and `pcap` for packet capture.

### Frontend
- **SvelteKit**
  - A modern framework for building responsive, reactive, and user-friendly web interfaces.
  - Integrated with APIs to display real-time and historical data.

### Other Tools
- **SQLite** or **PostgreSQL** (for historical log storage).
- **Docker** (for containerization).

## Installation

### Prerequisites
- Rust (https://www.rust-lang.org/tools/install)
- Node.js and npm (https://nodejs.org/)
- Docker (optional, for containerized deployment)
