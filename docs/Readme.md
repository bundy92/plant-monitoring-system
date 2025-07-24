# Plant Monitoring System

## Overview

This project is a plant monitoring solution using ESP32-C3 (Rust), Raspberry Pi (Rust backend), Yew (Frontend), SQLite (Database), and Machine Learning (Python).

## Components

1. **ESP32-C3 Firmware**: Reads sensor data (temperature, moisture, light) and sends it to the server.
2. **Server (Raspberry Pi)**: Collects sensor data and stores it in a SQLite database.
3. **Frontend (Yew)**: A web interface that displays sensor data in real-time.
4. **Machine Learning**: Predicts moisture levels using historical data.
5. **Logging**: Daily sensor data exported as CSV for analysis.

## Setup Instructions

### 1. ESP32-C3 Firmware:

   - Install Rust and the ESP32 toolchain.
   - Build and upload the firmware.

### 2. Raspberry Pi Server:

   - Install Rust and dependencies.
   - Run the Axum server on the Raspberry Pi.

### 3. Frontend (Yew):

   - Install `wasm-pack` and build the frontend.

### 4. Machine Learning:

   - Install Python dependencies and train the model.
