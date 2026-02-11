# RTHybrid Burst Analysis for RTSyn

> [!Note]
> This repo have different language implementations, see the branches for seeking them.

Tracks membrane potential min/max and burst duration

<a target="_blank" rel="noopener noreferrer" href="https://github.com/GNB-UAM/RTHybrid"> <img src="https://github.com/GNB-UAM/RTHy_plot_tool/raw/master/assets/logo_rthy.png" width="100" height="100"> </a>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
<a target="_blank" rel="noopener noreferrer" href="https://github.com/GNB-UAM"> <img src="https://github.com/GNB-UAM/RTHy_plot_tool/raw/master/assets/logo_gnb.png" width="100" height="100"> </a>

Set of modules for RTSyn with the functionality of RTHybrid to build hybrid circuits between neuron models and living neurons, including automatic adaptation and calibration algorithms.

For the standalone version of RTHybrid check https://github.com/GNB-UAM/RTHybrid.

- **Implementation:** C

## Requirements

### Rust toolchain (stable) with Cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### C toolchain

On Debian/Ubuntu:

```bash
sudo apt install build-essential gcc
```

On Fedora/RHEL/CentOS:

```bash
sudo dnf install gcc make
```

On Arch:

```bash
sudo pacman -Syu base-devel gcc
```

## Usage

Import this plugin in RTSyn from the plugin manager/installer, add it to the runtime, connect its ports, and start it from the plugin controls.

## Installation

Build with:

```bash
cargo build --release
```

Or import it directly from RTSyn.

## Citation

If you use RTHybrid or RTHybrid modules for RTSyn, cite:

Amaducci, R., Reyes-Sanchez, M., Elices, I., Rodriguez, F.B., & Varona, P. (2019). RTHybrid: a standardized and open-source real-time software model library for experimental neuroscience. Frontiers in Neuroinformatics, 13, 11. https://doi.org/10.3389/fninf.2019.00011

Reyes-Sanchez, M., Amaducci, R., Elices, I., Rodriguez, F. B., & Varona, P. (2020). Automatic adaptation of model neurons and connections to build hybrid circuits with living networks. Neuroinformatics 18: 377-393. https://doi.org/10.1007/s12021-019-09440-z
