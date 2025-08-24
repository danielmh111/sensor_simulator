# Sensor Data Simulator CLI
## Problem Statement
Create a flexible, command line sensor data generator in rust that produces realistic iot sensor readings for testing and development. This tool will simulate various sensor types (temperature, humidity, pressure, etc) with configurable parameters, realistic noise patterns, and multiple output formats. The simulator will serve as the foundation for testing data pipelines, validating database schemas, and prototyping analytics workflows for the larger sensor network project.
I'm using this project to touch up on rust before working on some actual projects with rust and sbcs/iot hardware. Hopefully it will come in useful for some of those.

## Success Criteria & Deliverables
By the end of the project, my overall goal is to have:

- A rust cli tool that generates realistic sensor data
- Support for 3-4 sensor data types with configurable parameters
- multiple output formats: JSON, CSV, direct db insertion
- Realistic data patterns (trends, noise, seasonal variations) (idk how to do this yet)
- a mature command-line interface i.e. good names, help documentation
- proper error handling and input validation
- Foundation code that can be extended for future sensor types (like a template)

## Tasks
1: Project setup and CLI framework
Goal: Get the basic project structure and CLI parsing working
Tasks:

    - [x] Create new rust project
    - [x] dependencies: clap (CLI parsing), serde (serialization), ~~chrono~~ time (timestamps)
    - [ ] CLI structure with subcommands for different sensor types
    - [ ] help text and version information
    - [ ] Test argument parsing

outcome: CLI that accepts arguments but doesn't generate data yet

2: Core Data Structures & Temperature Sensor
Goal: Build foundational data types and implement first sensor
Tasks:

    - [ ] sensor reading data structures (timestamp, sensor_id, value, metadata)
    - [ ] implement temperature sensor with realistic parameters (base temp, daily variation, noise)
    - [ ] Add basic random number generation (seeds for repeatability)
    - [ ] create console output format

outcome: CLI that generates temperature readings and prints to console

3: Multiple Sensor Types & Realistic Patterns
Goal: Add humidity, pressure sensors with realistic correlations
Tasks:

    - [ ] Implement humidity sensor (correlate with temperature for realism)
    - [ ] Add pressure sensor
    - [ ] Implement motion sensor (binary on/off with realistic timing)
    - [ ] Add time-based patterns (daily cycles, yearly cycles, trending)

outcome: CLI generating multiple sensor types with realistic data patterns

4: Output Formats - JSON, CSV
Goal: Support multiple data export formats
Tasks:

    - [ ] Add JSON output format with proper serialization
    - [ ] Implement CSV output with headers and proper escaping
    - [ ] Add file output options (write to files vs. stdout)
    - [ ] Implement batch generation (generate N readings at once)

outcome: Tool that can export data in JSON and CSV formats

5: Database integration
Goal: Direct database insertion capability
Tasks:

    - [ ] Add sqlite dependency and database connectivity
    - [ ] Create schema for sensor readings table
    - [ ] Implement inserts
    - [ ] Add database connection config

outcome: Tool that can insert data directly into SQLite database

## Nice-to-haves

6: config, extra features, docs
Goal: Make the tool flexible and production quality
Tasks:

    - [ ] add configuration file support (TOML or JSON)
    - [ ] implement data validation and bounds checking
    - [ ] time range generation (historical/future data)
    - [ ] Add sensor metadata (location, calibration info, version num, brand(?))
    - [ ] Add comprehensive error messages
    - [ ] Write a presentation README with user guide, examples, install guide, debugging etc.
    - [ ] add unit tests for core functions
    - [ ] Performance optimization
    - [ ] example/template config files


7. maybes
    - [ ] docker image + deployment guide
    - [ ] option for adding anomilies
    - [ ] streaming data mode / set event frequency