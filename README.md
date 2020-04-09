# test_rtt_nrf52832

A simple app that demonstrates how to use RTT debugging with
the nrf52832 embedded microcontroller. 


## Running

You will need:

- A board of some kind with the nrf52832 microcontroller 
- A Segger J-Link
- A SWD connection to your nrf52 microcontroller. Only SWDIO, SWCLK, GND, and VREF connections are required. 
- Segger J-Link software.  We use JLinkGDBServer and JLinkRTTViewer


Go:

- Connect the nrf52 to your J-Link, J-Link to your host computer
- Start the gdb server using eg [script](./start_gdb_server_jlink.sh) on your host
- Start the JLinkRTTViewer software (or similar)
- `cargo run` should build, install, and run this application on the target nrf52
- Your JLinkRTTViewer should display lots of debug output


