# Near real
# Visualization Tool

## Overview

This repository contains two tools designed to help developers understand and utilize the Solana blockchain:

1. **Visualization Tool**: A tool for visualizing program interactions and Cross Program Invocation (CPI) logic within the Solana blockchain.
## Features

### Visualization Tool
- Visualizes memory management and program interactions on the Solana blockchain.
- Provides clear and accurate representations of CPI calls.
- Helps developers better understand the underlying mechanics of Solana programs.


To run this project, you will need:
- Rust and Cargo installed on your machine.
- Solana CLI and Anchor installed.
- Node.js and npm for the frontend.

## Setup Instructions

### Backend Setup

1. **Clone the Repository**
   ```bash
   git clone https://github.com/yourusername/visualization_tool.git
   cd visualization_tool
Navigate to the project directory and initialize the Anchor project:
bash
Copy code
anchor init visualization_tool
Add CPI Logic in lib.rs

Write your CPI code in src/lib.rs. Refer to the example code provided in this repository.
Build the Project

bash
Copy code
anchor build
Frontend Setup
Install Dependencies

Navigate to the frontend directory and install the required packages:
bash
Copy code
cd frontend
npm install
Run the Application

bash
Copy code
npm start
Usage
Visualization Tool: Launch the backend and access the visualization tool via the web interface. Use it to explore memory management and program interactions.
Real-Time Collaboration Tool: Open the collaboration tool in your browser. Share the document link with others for real-time editing.
 
