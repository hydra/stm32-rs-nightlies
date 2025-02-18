///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - The MAC Configuration Register establishes the operating mode of the MAC.
    pub eth_maccr: ETH_MACCR,
    ///0x04 - The MAC Extended Configuration Register establishes the operating mode of the MAC.
    pub eth_macecr: ETH_MACECR,
    ///0x08 - The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets.
    pub eth_macpfr: ETH_MACPFR,
    ///0x0c - The Watchdog Timeout register controls the watchdog timeout for received packets.
    pub eth_macwtr: ETH_MACWTR,
    ///0x10 - The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\]
    ///(in little-endian mode) or Bits\[7:0\]
    ///(in big-endian mode) of the Hash Table Register X registers are written.
    pub eth_macht0r: ETH_MACHT0R,
    ///0x14 - The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\]
    ///(in little-endian mode) or Bits\[7:0\]
    ///(in big-endian mode) of the Hash Table Register X registers are written.
    pub eth_macht1r: ETH_MACHT1R,
    _reserved6: [u8; 0x38],
    ///0x50 - The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets.
    pub eth_macvtr: ETH_MACVTR,
    _reserved7: [u8; 0x04],
    ///0x58 - When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[15:8\]
    ///(in little-endian mode) or Bits\[7:0\]
    ///(in big-endian mode) of this register are written.
    pub eth_macvhtr: ETH_MACVHTR,
    _reserved8: [u8; 0x04],
    ///0x60 - The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls.
    pub eth_macvir: ETH_MACVIR,
    ///0x64 - The Inner VLAN Tag Inclusion or Replacement register contains the inner VLAN tag to be inserted or replaced in the Transmit packet. It also contains the inner VLAN tag insertion controls.
    pub eth_macivir: ETH_MACIVIR,
    _reserved10: [u8; 0x08],
    ///0x70 - The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register.
    pub eth_macq0tx_fcr: ETH_MACQ0TX_FCR,
    _reserved11: [u8; 0x1c],
    ///0x90 - The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet.
    pub eth_macrx_fcr: ETH_MACRX_FCR,
    _reserved12: [u8; 0x04],
    ///0x98 - The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1.
    pub eth_mactx_qpmr: ETH_MACTX_QPMR,
    _reserved13: [u8; 0x04],
    ///0xa0 - The Receive Queue Control 0 register controls the queue management in the MAC Receiver.
    pub eth_macrx_qc0r: ETH_MACRX_QC0R,
    ///0xa4 - The Receive Queue Control 1 register controls queue 1 management in the MAC receiver.
    pub eth_macrx_qc1r: ETH_MACRX_QC1R,
    ///0xa8 - This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1.
    pub eth_macrx_qc2r: ETH_MACRX_QC2R,
    _reserved16: [u8; 0x04],
    ///0xb0 - The Interrupt Status register contains the status of interrupts.
    pub eth_macisr: ETH_MACISR,
    ///0xb4 - The Interrupt Enable register contains the masks for generating the interrupts.
    pub eth_macier: ETH_MACIER,
    ///0xb8 - The Receive Transmit Status register contains the Receive and Transmit Error status.
    pub eth_macrx_tx_sr: ETH_MACRX_TX_SR,
    _reserved19: [u8; 0x04],
    ///0xc0 - The PMT Control and Status Register is present only when you select the PMT module in coreConsultant.
    pub eth_macpcsr: ETH_MACPCSR,
    ///0xc4 - The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.
    pub eth_macrwkpfr: ETH_MACRWKPFR,
    _reserved21: [u8; 0x08],
    ///0xd0 - The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.
    pub eth_maclcsr: ETH_MACLCSR,
    ///0xd4 - The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission.
    pub eth_macltcr: ETH_MACLTCR,
    ///0xd8 - The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds.
    pub eth_macletr: ETH_MACLETR,
    ///0xdc - This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially.
    pub eth_mac1ustcr: ETH_MAC1USTCR,
    _reserved25: [u8; 0x18],
    ///0xf8 - The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY.
    pub eth_macphycsr: ETH_MACPHYCSR,
    _reserved26: [u8; 0x14],
    ///0x110 - The version register identifies the version of the Ethernet peripheral.
    pub eth_macvr: ETH_MACVR,
    ///0x114 - The Debug register provides the debug status of various MAC blocks.
    pub eth_macdr: ETH_MACDR,
    _reserved28: [u8; 0x08],
    ///0x120 - This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.
    pub eth_machwf1r: ETH_MACHWF1R,
    ///0x124 - This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.
    pub eth_machwf2r: ETH_MACHWF2R,
    _reserved30: [u8; 0xd8],
    ///0x200 - The MDIO Address register controls the management cycles to external PHY through a management interface.
    pub eth_macmdioar: ETH_MACMDIOAR,
    ///0x204 - The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register.
    pub eth_macmdiodr: ETH_MACMDIODR,
    _reserved32: [u8; 0xf8],
    ///0x300 - The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \[7:0\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \[47:0\]
    ///is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\[31:24\]
    ///(in little-endian mode) or Bits\[7:0\]
    ///(in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain.
    pub eth_maca0hr: ETH_MACA0HR,
    ///0x304 - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
    pub eth_maca0lr: ETH_MACA0LR,
    ///0x308 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
    pub eth_maca1hr: ETH_MACA1HR,
    ///0x30c - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
    pub eth_maca1lr: ETH_MACA1LR,
    ///0x310 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
    pub eth_maca2hr: ETH_MACA2HR,
    ///0x314 - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
    pub eth_maca2lr: ETH_MACA2LR,
    ///0x318 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
    pub eth_maca3hr: ETH_MACA3HR,
    ///0x31c - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
    pub eth_maca3lr: ETH_MACA3LR,
    _reserved40: [u8; 0x03e0],
    ///0x700 - This register configures the MMC operating mode.
    pub mmc_control: MMC_CONTROL,
    ///0x704 - This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\[7:0\]) of the respective counter must be read to clear the interrupt bit.
    pub mmc_rx_interrupt: MMC_RX_INTERRUPT,
    ///0x708 - This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\[7:0\]) of the respective counter must be read to clear the interrupt bit.
    pub mmc_tx_interrupt: MMC_TX_INTERRUPT,
    ///0x70c - The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values.
    pub mmc_rx_interrupt_mask: MMC_RX_INTERRUPT_MASK,
    ///0x710 - This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration.
    pub mmc_tx_interrupt_mask: MMC_TX_INTERRUPT_MASK,
    _reserved45: [u8; 0x38],
    ///0x74c - This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode.
    pub tx_single_collision_good_packets: TX_SINGLE_COLLISION_GOOD_PACKETS,
    ///0x750 - This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode.
    pub tx_multiple_collision_good_packets: TX_MULTIPLE_COLLISION_GOOD_PACKETS,
    _reserved47: [u8; 0x14],
    ///0x768 - This register provides the number of good packets transmitted by Ethernet peripheral.
    pub tx_packet_count_good: TX_PACKET_COUNT_GOOD,
    _reserved48: [u8; 0x28],
    ///0x794 - This register provides the number of packets received by Ethernet peripheral with CRC error.
    pub rx_crc_error_packets: RX_CRC_ERROR_PACKETS,
    ///0x798 - This register provides the number of packets received by Ethernet peripheral with alignment (dribble) error. It is valid only in 10/100 mode.
    pub rx_alignment_error_packets: RX_ALIGNMENT_ERROR_PACKETS,
    _reserved50: [u8; 0x28],
    ///0x7c4 - This register provides the number of good unicast packets received by Ethernet peripheral.
    pub rx_unicast_packets_good: RX_UNICAST_PACKETS_GOOD,
    _reserved51: [u8; 0x24],
    ///0x7ec - This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral.
    pub tx_lpi_usec_cntr: TX_LPI_USEC_CNTR,
    ///0x7f0 - This register provides the number of times Ethernet peripheral has entered Tx LPI.
    pub tx_lpi_tran_cntr: TX_LPI_TRAN_CNTR,
    ///0x7f4 - This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral.
    pub rx_lpi_usec_cntr: RX_LPI_USEC_CNTR,
    ///0x7f8 - This register provides the number of times Ethernet peripheral has entered Rx LPI.
    pub rx_lpi_tran_cntr: RX_LPI_TRAN_CNTR,
    _reserved55: [u8; 0x0104],
    ///0x900 - The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration.
    pub eth_macl3l4c0r: ETH_MACL3L4C0R,
    ///0x904 - Layer4 address filter 0 register
    pub eth_macl4a0r: ETH_MACL4A0R,
    _reserved57: [u8; 0x08],
    ///0x910 - For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\[31:0\]
    ///of the 128-bit IP Source Address or Destination Address field.
    pub eth_macl3a00r: ETH_MACL3A00R,
    ///0x914 - For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\]
    ///of the 128-bit IP Source Address or Destination Address field.
    pub eth_macl3a10r: ETH_MACL3A10R,
    ///0x918 - The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[95:64\]
    ///of 128-bit IP Source Address or Destination Address field.
    pub eth_macl3a20: ETH_MACL3A20,
    ///0x91c - The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[127:96\]
    ///of 128-bit IP Source Address or Destination Address field.
    pub eth_macl3a30: ETH_MACL3A30,
    _reserved61: [u8; 0x10],
    ///0x930 - The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4.
    pub eth_macl3l4c1r: ETH_MACL3L4C1R,
    ///0x934 - The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\[31:24\]
    ///(in little-endian mode) or Bits\[7:0\]
    ///(in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock.
    pub eth_macl4a1r: ETH_MACL4A1R,
    _reserved63: [u8; 0x08],
    ///0x940 - For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\[31:0\]
    ///of the 128-bit IP Source Address or Destination Address field.
    pub eth_macl3a01r: ETH_MACL3A01R,
    ///0x944 - For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\]
    ///of the 128-bit IP Source Address or Destination Address field.
    pub eth_macl3a11r: ETH_MACL3A11R,
    ///0x948 - The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[95:64\]
    ///of 128-bit IP Source Address or Destination Address field.
    pub eth_macl3a21r: ETH_MACL3A21R,
    ///0x94c - The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[127:96\]
    ///of 128-bit IP Source Address or Destination Address field.
    pub eth_macl3a31r: ETH_MACL3A31R,
    _reserved67: [u8; 0x0190],
    ///0xae0 - The ARP Address register contains the IPv4 Destination Address of the MAC.
    pub eth_macarpar: ETH_MACARPAR,
    _reserved68: [u8; 0x1c],
    ///0xb00 - This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver.
    pub eth_mactscr: ETH_MACTSCR,
    ///0xb04 - The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow.
    pub eth_macssir: ETH_MACSSIR,
    ///0xb08 - The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
    pub eth_macstsr: ETH_MACSTSR,
    ///0xb0c - The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
    pub eth_macstnr: ETH_MACSTNR,
    ///0xb10 - The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
    pub eth_macstsur: ETH_MACSTSUR,
    ///0xb14 - This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input.
    pub eth_macstnur: ETH_MACSTNUR,
    ///0xb18 - The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows.
    pub eth_mactsar: ETH_MACTSAR,
    _reserved75: [u8; 0x04],
    ///0xb20 - The Timestamp Status register is present only when the IEEE 1588 Timestamp feature is selected. All bits except Bits\[27:25\]
    ///gets cleared when the application reads this register.
    pub eth_mactssr: ETH_MACTSSR,
    _reserved76: [u8; 0x0c],
    ///0xb30 - This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled.
    pub eth_mactx_tssnr: ETH_MACTX_TSSNR,
    ///0xb34 - The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted.
    pub eth_mactx_tsssr: ETH_MACTX_TSSSR,
    _reserved78: [u8; 0x08],
    ///0xb40 - The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot.
    pub eth_macacr: ETH_MACACR,
    _reserved79: [u8; 0x04],
    ///0xb48 - The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\[29:25\]
    ///in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\[31:24\]
    ///are read and in big-endian mode, Bits\[7:0\]
    ///are read.
    pub eth_macatsnr: ETH_MACATSNR,
    ///0xb4c - The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register.
    pub eth_macatssr: ETH_MACATSSR,
    ///0xb50 - The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages.
    pub eth_mactsiacr: ETH_MACTSIACR,
    ///0xb54 - The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages.
    pub eth_mactseacr: ETH_MACTSEACR,
    ///0xb58 - This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path.
    pub eth_mactsicnr: ETH_MACTSICNR,
    ///0xb5c - This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path.
    pub eth_mactsecnr: ETH_MACTSECNR,
    _reserved85: [u8; 0x10],
    ///0xb70 - The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\[30:24\]
    ///of this register are valid only when four Flexible PPS outputs are selected. Bits\[22:16\]
    ///are valid only when three or more Flexible PPS outputs are selected. Bits\[14:8\]
    ///are valid only when two or more Flexible PPS outputs are selected. Bits\[6:4\]
    ///are valid only when Flexible PPS feature is selected.
    pub eth_macppscr: ETH_MACPPSCR,
    _reserved86: [u8; 0x0c],
    ///0xb80 - The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \[Bit 1 of ETH_MACTSSR\]
    ///when the system time exceeds the value programmed in these registers.
    pub eth_macppsttsr: ETH_MACPPSTTSR,
    ///0xb84 - The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected.
    pub eth_macppsttnr: ETH_MACPPSTTNR,
    ///0xb88 - The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\[0\]).
    pub eth_macppsir: ETH_MACPPSIR,
    ///0xb8c - The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o).
    pub eth_macppswr: ETH_MACPPSWR,
    _reserved90: [u8; 0x30],
    ///0xbc0 - This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected.
    pub eth_macpocr: ETH_MACPOCR,
    ///0xbc4 - This register contains Bits\[31:0\]
    ///of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.
    pub eth_macspi0r: ETH_MACSPI0R,
    ///0xbc8 - This register contains Bits\[63:32\]
    ///of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.
    pub eth_macspi1r: ETH_MACSPI1R,
    ///0xbcc - This register contains Bits\[79:64\]
    ///of the 80-bit Source Port Identity of the PTP node.
    pub eth_macspi2r: ETH_MACSPI2R,
    ///0xbd0 - This register contains the periodic intervals for automatic PTP packet generation.
    pub eth_maclmir: ETH_MACLMIR,
}
///ETH_MACCR (rw) register accessor: an alias for `Reg<ETH_MACCR_SPEC>`
pub type ETH_MACCR = crate::Reg<eth_maccr::ETH_MACCR_SPEC>;
///The MAC Configuration Register establishes the operating mode of the MAC.
pub mod eth_maccr;
///ETH_MACECR (rw) register accessor: an alias for `Reg<ETH_MACECR_SPEC>`
pub type ETH_MACECR = crate::Reg<eth_macecr::ETH_MACECR_SPEC>;
///The MAC Extended Configuration Register establishes the operating mode of the MAC.
pub mod eth_macecr;
///ETH_MACPFR (rw) register accessor: an alias for `Reg<ETH_MACPFR_SPEC>`
pub type ETH_MACPFR = crate::Reg<eth_macpfr::ETH_MACPFR_SPEC>;
///The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets.
pub mod eth_macpfr;
///ETH_MACWTR (rw) register accessor: an alias for `Reg<ETH_MACWTR_SPEC>`
pub type ETH_MACWTR = crate::Reg<eth_macwtr::ETH_MACWTR_SPEC>;
///The Watchdog Timeout register controls the watchdog timeout for received packets.
pub mod eth_macwtr;
///ETH_MACHT0R (rw) register accessor: an alias for `Reg<ETH_MACHT0R_SPEC>`
pub type ETH_MACHT0R = crate::Reg<eth_macht0r::ETH_MACHT0R_SPEC>;
///The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\]
///(in little-endian mode) or Bits\[7:0\]
///(in big-endian mode) of the Hash Table Register X registers are written.
pub mod eth_macht0r;
///ETH_MACHT1R (rw) register accessor: an alias for `Reg<ETH_MACHT1R_SPEC>`
pub type ETH_MACHT1R = crate::Reg<eth_macht1r::ETH_MACHT1R_SPEC>;
///The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\]
///(in little-endian mode) or Bits\[7:0\]
///(in big-endian mode) of the Hash Table Register X registers are written.
pub mod eth_macht1r;
///ETH_MACVTR (rw) register accessor: an alias for `Reg<ETH_MACVTR_SPEC>`
pub type ETH_MACVTR = crate::Reg<eth_macvtr::ETH_MACVTR_SPEC>;
///The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets.
pub mod eth_macvtr;
///ETH_MACVHTR (rw) register accessor: an alias for `Reg<ETH_MACVHTR_SPEC>`
pub type ETH_MACVHTR = crate::Reg<eth_macvhtr::ETH_MACVHTR_SPEC>;
///When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[15:8\]
///(in little-endian mode) or Bits\[7:0\]
///(in big-endian mode) of this register are written.
pub mod eth_macvhtr;
///ETH_MACVIR (rw) register accessor: an alias for `Reg<ETH_MACVIR_SPEC>`
pub type ETH_MACVIR = crate::Reg<eth_macvir::ETH_MACVIR_SPEC>;
///The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls.
pub mod eth_macvir;
///ETH_MACIVIR (rw) register accessor: an alias for `Reg<ETH_MACIVIR_SPEC>`
pub type ETH_MACIVIR = crate::Reg<eth_macivir::ETH_MACIVIR_SPEC>;
///The Inner VLAN Tag Inclusion or Replacement register contains the inner VLAN tag to be inserted or replaced in the Transmit packet. It also contains the inner VLAN tag insertion controls.
pub mod eth_macivir;
///ETH_MACQ0TxFCR (rw) register accessor: an alias for `Reg<ETH_MACQ0TX_FCR_SPEC>`
pub type ETH_MACQ0TX_FCR = crate::Reg<eth_macq0tx_fcr::ETH_MACQ0TX_FCR_SPEC>;
///The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register.
pub mod eth_macq0tx_fcr;
///ETH_MACRxFCR (rw) register accessor: an alias for `Reg<ETH_MACRX_FCR_SPEC>`
pub type ETH_MACRX_FCR = crate::Reg<eth_macrx_fcr::ETH_MACRX_FCR_SPEC>;
///The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet.
pub mod eth_macrx_fcr;
///ETH_MACTxQPMR (r) register accessor: an alias for `Reg<ETH_MACTX_QPMR_SPEC>`
pub type ETH_MACTX_QPMR = crate::Reg<eth_mactx_qpmr::ETH_MACTX_QPMR_SPEC>;
///The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1.
pub mod eth_mactx_qpmr;
///ETH_MACRxQC0R (rw) register accessor: an alias for `Reg<ETH_MACRX_QC0R_SPEC>`
pub type ETH_MACRX_QC0R = crate::Reg<eth_macrx_qc0r::ETH_MACRX_QC0R_SPEC>;
///The Receive Queue Control 0 register controls the queue management in the MAC Receiver.
pub mod eth_macrx_qc0r;
///ETH_MACRxQC1R (rw) register accessor: an alias for `Reg<ETH_MACRX_QC1R_SPEC>`
pub type ETH_MACRX_QC1R = crate::Reg<eth_macrx_qc1r::ETH_MACRX_QC1R_SPEC>;
///The Receive Queue Control 1 register controls queue 1 management in the MAC receiver.
pub mod eth_macrx_qc1r;
///ETH_MACRxQC2R (rw) register accessor: an alias for `Reg<ETH_MACRX_QC2R_SPEC>`
pub type ETH_MACRX_QC2R = crate::Reg<eth_macrx_qc2r::ETH_MACRX_QC2R_SPEC>;
///This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1.
pub mod eth_macrx_qc2r;
///ETH_MACISR (r) register accessor: an alias for `Reg<ETH_MACISR_SPEC>`
pub type ETH_MACISR = crate::Reg<eth_macisr::ETH_MACISR_SPEC>;
///The Interrupt Status register contains the status of interrupts.
pub mod eth_macisr;
///ETH_MACIER (rw) register accessor: an alias for `Reg<ETH_MACIER_SPEC>`
pub type ETH_MACIER = crate::Reg<eth_macier::ETH_MACIER_SPEC>;
///The Interrupt Enable register contains the masks for generating the interrupts.
pub mod eth_macier;
///ETH_MACRxTxSR (r) register accessor: an alias for `Reg<ETH_MACRX_TX_SR_SPEC>`
pub type ETH_MACRX_TX_SR = crate::Reg<eth_macrx_tx_sr::ETH_MACRX_TX_SR_SPEC>;
///The Receive Transmit Status register contains the Receive and Transmit Error status.
pub mod eth_macrx_tx_sr;
///ETH_MACPCSR (rw) register accessor: an alias for `Reg<ETH_MACPCSR_SPEC>`
pub type ETH_MACPCSR = crate::Reg<eth_macpcsr::ETH_MACPCSR_SPEC>;
///The PMT Control and Status Register is present only when you select the PMT module in coreConsultant.
pub mod eth_macpcsr;
///ETH_MACRWKPFR (rw) register accessor: an alias for `Reg<ETH_MACRWKPFR_SPEC>`
pub type ETH_MACRWKPFR = crate::Reg<eth_macrwkpfr::ETH_MACRWKPFR_SPEC>;
///The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.
pub mod eth_macrwkpfr;
///ETH_MACLCSR (rw) register accessor: an alias for `Reg<ETH_MACLCSR_SPEC>`
pub type ETH_MACLCSR = crate::Reg<eth_maclcsr::ETH_MACLCSR_SPEC>;
///The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.
pub mod eth_maclcsr;
///ETH_MACLTCR (rw) register accessor: an alias for `Reg<ETH_MACLTCR_SPEC>`
pub type ETH_MACLTCR = crate::Reg<eth_macltcr::ETH_MACLTCR_SPEC>;
///The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission.
pub mod eth_macltcr;
///ETH_MACLETR (rw) register accessor: an alias for `Reg<ETH_MACLETR_SPEC>`
pub type ETH_MACLETR = crate::Reg<eth_macletr::ETH_MACLETR_SPEC>;
///The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds.
pub mod eth_macletr;
///ETH_MAC1USTCR (rw) register accessor: an alias for `Reg<ETH_MAC1USTCR_SPEC>`
pub type ETH_MAC1USTCR = crate::Reg<eth_mac1ustcr::ETH_MAC1USTCR_SPEC>;
///This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially.
pub mod eth_mac1ustcr;
///ETH_MACPHYCSR (rw) register accessor: an alias for `Reg<ETH_MACPHYCSR_SPEC>`
pub type ETH_MACPHYCSR = crate::Reg<eth_macphycsr::ETH_MACPHYCSR_SPEC>;
///The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY.
pub mod eth_macphycsr;
///ETH_MACVR (r) register accessor: an alias for `Reg<ETH_MACVR_SPEC>`
pub type ETH_MACVR = crate::Reg<eth_macvr::ETH_MACVR_SPEC>;
///The version register identifies the version of the Ethernet peripheral.
pub mod eth_macvr;
///ETH_MACDR (r) register accessor: an alias for `Reg<ETH_MACDR_SPEC>`
pub type ETH_MACDR = crate::Reg<eth_macdr::ETH_MACDR_SPEC>;
///The Debug register provides the debug status of various MAC blocks.
pub mod eth_macdr;
///ETH_MACHWF1R (r) register accessor: an alias for `Reg<ETH_MACHWF1R_SPEC>`
pub type ETH_MACHWF1R = crate::Reg<eth_machwf1r::ETH_MACHWF1R_SPEC>;
///This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.
pub mod eth_machwf1r;
///ETH_MACHWF2R (r) register accessor: an alias for `Reg<ETH_MACHWF2R_SPEC>`
pub type ETH_MACHWF2R = crate::Reg<eth_machwf2r::ETH_MACHWF2R_SPEC>;
///This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.
pub mod eth_machwf2r;
///ETH_MACMDIOAR (rw) register accessor: an alias for `Reg<ETH_MACMDIOAR_SPEC>`
pub type ETH_MACMDIOAR = crate::Reg<eth_macmdioar::ETH_MACMDIOAR_SPEC>;
///The MDIO Address register controls the management cycles to external PHY through a management interface.
pub mod eth_macmdioar;
///ETH_MACMDIODR (rw) register accessor: an alias for `Reg<ETH_MACMDIODR_SPEC>`
pub type ETH_MACMDIODR = crate::Reg<eth_macmdiodr::ETH_MACMDIODR_SPEC>;
///The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register.
pub mod eth_macmdiodr;
///ETH_MACA0HR (rw) register accessor: an alias for `Reg<ETH_MACA0HR_SPEC>`
pub type ETH_MACA0HR = crate::Reg<eth_maca0hr::ETH_MACA0HR_SPEC>;
///The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \[7:0\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \[47:0\]
///is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\[31:24\]
///(in little-endian mode) or Bits\[7:0\]
///(in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain.
pub mod eth_maca0hr;
///ETH_MACA0LR (rw) register accessor: an alias for `Reg<ETH_MACA0LR_SPEC>`
pub type ETH_MACA0LR = crate::Reg<eth_maca0lr::ETH_MACA0LR_SPEC>;
///The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
pub mod eth_maca0lr;
///ETH_MACA1HR (rw) register accessor: an alias for `Reg<ETH_MACA1HR_SPEC>`
pub type ETH_MACA1HR = crate::Reg<eth_maca1hr::ETH_MACA1HR_SPEC>;
///The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
pub mod eth_maca1hr;
///ETH_MACA1LR (rw) register accessor: an alias for `Reg<ETH_MACA1LR_SPEC>`
pub type ETH_MACA1LR = crate::Reg<eth_maca1lr::ETH_MACA1LR_SPEC>;
///The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
pub mod eth_maca1lr;
///ETH_MACA2HR (rw) register accessor: an alias for `Reg<ETH_MACA2HR_SPEC>`
pub type ETH_MACA2HR = crate::Reg<eth_maca2hr::ETH_MACA2HR_SPEC>;
///The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
pub mod eth_maca2hr;
///ETH_MACA2LR (rw) register accessor: an alias for `Reg<ETH_MACA2LR_SPEC>`
pub type ETH_MACA2LR = crate::Reg<eth_maca2lr::ETH_MACA2LR_SPEC>;
///The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
pub mod eth_maca2lr;
///ETH_MACA3HR (rw) register accessor: an alias for `Reg<ETH_MACA3HR_SPEC>`
pub type ETH_MACA3HR = crate::Reg<eth_maca3hr::ETH_MACA3HR_SPEC>;
///The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
pub mod eth_maca3hr;
///ETH_MACA3LR (rw) register accessor: an alias for `Reg<ETH_MACA3LR_SPEC>`
pub type ETH_MACA3LR = crate::Reg<eth_maca3lr::ETH_MACA3LR_SPEC>;
///The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
pub mod eth_maca3lr;
///MMC_CONTROL (rw) register accessor: an alias for `Reg<MMC_CONTROL_SPEC>`
pub type MMC_CONTROL = crate::Reg<mmc_control::MMC_CONTROL_SPEC>;
///This register configures the MMC operating mode.
pub mod mmc_control;
///MMC_RX_INTERRUPT (r) register accessor: an alias for `Reg<MMC_RX_INTERRUPT_SPEC>`
pub type MMC_RX_INTERRUPT = crate::Reg<mmc_rx_interrupt::MMC_RX_INTERRUPT_SPEC>;
///This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\[7:0\]) of the respective counter must be read to clear the interrupt bit.
pub mod mmc_rx_interrupt;
///MMC_TX_INTERRUPT (r) register accessor: an alias for `Reg<MMC_TX_INTERRUPT_SPEC>`
pub type MMC_TX_INTERRUPT = crate::Reg<mmc_tx_interrupt::MMC_TX_INTERRUPT_SPEC>;
///This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\[7:0\]) of the respective counter must be read to clear the interrupt bit.
pub mod mmc_tx_interrupt;
///MMC_RX_INTERRUPT_MASK (rw) register accessor: an alias for `Reg<MMC_RX_INTERRUPT_MASK_SPEC>`
pub type MMC_RX_INTERRUPT_MASK = crate::Reg<mmc_rx_interrupt_mask::MMC_RX_INTERRUPT_MASK_SPEC>;
///The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values.
pub mod mmc_rx_interrupt_mask;
///MMC_TX_INTERRUPT_MASK (rw) register accessor: an alias for `Reg<MMC_TX_INTERRUPT_MASK_SPEC>`
pub type MMC_TX_INTERRUPT_MASK = crate::Reg<mmc_tx_interrupt_mask::MMC_TX_INTERRUPT_MASK_SPEC>;
///This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration.
pub mod mmc_tx_interrupt_mask;
///TX_SINGLE_COLLISION_GOOD_PACKETS (r) register accessor: an alias for `Reg<TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>`
pub type TX_SINGLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_single_collision_good_packets::TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>;
///This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode.
pub mod tx_single_collision_good_packets;
///TX_MULTIPLE_COLLISION_GOOD_PACKETS (r) register accessor: an alias for `Reg<TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>`
pub type TX_MULTIPLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_multiple_collision_good_packets::TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>;
///This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode.
pub mod tx_multiple_collision_good_packets;
///TX_PACKET_COUNT_GOOD (r) register accessor: an alias for `Reg<TX_PACKET_COUNT_GOOD_SPEC>`
pub type TX_PACKET_COUNT_GOOD = crate::Reg<tx_packet_count_good::TX_PACKET_COUNT_GOOD_SPEC>;
///This register provides the number of good packets transmitted by Ethernet peripheral.
pub mod tx_packet_count_good;
///RX_CRC_ERROR_PACKETS (r) register accessor: an alias for `Reg<RX_CRC_ERROR_PACKETS_SPEC>`
pub type RX_CRC_ERROR_PACKETS = crate::Reg<rx_crc_error_packets::RX_CRC_ERROR_PACKETS_SPEC>;
///This register provides the number of packets received by Ethernet peripheral with CRC error.
pub mod rx_crc_error_packets;
///RX_ALIGNMENT_ERROR_PACKETS (r) register accessor: an alias for `Reg<RX_ALIGNMENT_ERROR_PACKETS_SPEC>`
pub type RX_ALIGNMENT_ERROR_PACKETS =
    crate::Reg<rx_alignment_error_packets::RX_ALIGNMENT_ERROR_PACKETS_SPEC>;
///This register provides the number of packets received by Ethernet peripheral with alignment (dribble) error. It is valid only in 10/100 mode.
pub mod rx_alignment_error_packets;
///RX_UNICAST_PACKETS_GOOD (r) register accessor: an alias for `Reg<RX_UNICAST_PACKETS_GOOD_SPEC>`
pub type RX_UNICAST_PACKETS_GOOD =
    crate::Reg<rx_unicast_packets_good::RX_UNICAST_PACKETS_GOOD_SPEC>;
///This register provides the number of good unicast packets received by Ethernet peripheral.
pub mod rx_unicast_packets_good;
///TX_LPI_USEC_CNTR (r) register accessor: an alias for `Reg<TX_LPI_USEC_CNTR_SPEC>`
pub type TX_LPI_USEC_CNTR = crate::Reg<tx_lpi_usec_cntr::TX_LPI_USEC_CNTR_SPEC>;
///This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral.
pub mod tx_lpi_usec_cntr;
///TX_LPI_TRAN_CNTR (r) register accessor: an alias for `Reg<TX_LPI_TRAN_CNTR_SPEC>`
pub type TX_LPI_TRAN_CNTR = crate::Reg<tx_lpi_tran_cntr::TX_LPI_TRAN_CNTR_SPEC>;
///This register provides the number of times Ethernet peripheral has entered Tx LPI.
pub mod tx_lpi_tran_cntr;
///RX_LPI_USEC_CNTR (r) register accessor: an alias for `Reg<RX_LPI_USEC_CNTR_SPEC>`
pub type RX_LPI_USEC_CNTR = crate::Reg<rx_lpi_usec_cntr::RX_LPI_USEC_CNTR_SPEC>;
///This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral.
pub mod rx_lpi_usec_cntr;
///RX_LPI_TRAN_CNTR (r) register accessor: an alias for `Reg<RX_LPI_TRAN_CNTR_SPEC>`
pub type RX_LPI_TRAN_CNTR = crate::Reg<rx_lpi_tran_cntr::RX_LPI_TRAN_CNTR_SPEC>;
///This register provides the number of times Ethernet peripheral has entered Rx LPI.
pub mod rx_lpi_tran_cntr;
///ETH_MACL3L4C0R (rw) register accessor: an alias for `Reg<ETH_MACL3L4C0R_SPEC>`
pub type ETH_MACL3L4C0R = crate::Reg<eth_macl3l4c0r::ETH_MACL3L4C0R_SPEC>;
///The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration.
pub mod eth_macl3l4c0r;
///ETH_MACL4A0R (rw) register accessor: an alias for `Reg<ETH_MACL4A0R_SPEC>`
pub type ETH_MACL4A0R = crate::Reg<eth_macl4a0r::ETH_MACL4A0R_SPEC>;
///Layer4 address filter 0 register
pub mod eth_macl4a0r;
///ETH_MACL3A00R (rw) register accessor: an alias for `Reg<ETH_MACL3A00R_SPEC>`
pub type ETH_MACL3A00R = crate::Reg<eth_macl3a00r::ETH_MACL3A00R_SPEC>;
///For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\[31:0\]
///of the 128-bit IP Source Address or Destination Address field.
pub mod eth_macl3a00r;
///ETH_MACL3A10R (rw) register accessor: an alias for `Reg<ETH_MACL3A10R_SPEC>`
pub type ETH_MACL3A10R = crate::Reg<eth_macl3a10r::ETH_MACL3A10R_SPEC>;
///For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\]
///of the 128-bit IP Source Address or Destination Address field.
pub mod eth_macl3a10r;
///ETH_MACL3A20 (rw) register accessor: an alias for `Reg<ETH_MACL3A20_SPEC>`
pub type ETH_MACL3A20 = crate::Reg<eth_macl3a20::ETH_MACL3A20_SPEC>;
///The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[95:64\]
///of 128-bit IP Source Address or Destination Address field.
pub mod eth_macl3a20;
///ETH_MACL3A30 (rw) register accessor: an alias for `Reg<ETH_MACL3A30_SPEC>`
pub type ETH_MACL3A30 = crate::Reg<eth_macl3a30::ETH_MACL3A30_SPEC>;
///The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[127:96\]
///of 128-bit IP Source Address or Destination Address field.
pub mod eth_macl3a30;
///ETH_MACL3L4C1R (rw) register accessor: an alias for `Reg<ETH_MACL3L4C1R_SPEC>`
pub type ETH_MACL3L4C1R = crate::Reg<eth_macl3l4c1r::ETH_MACL3L4C1R_SPEC>;
///The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4.
pub mod eth_macl3l4c1r;
///ETH_MACL4A1R (rw) register accessor: an alias for `Reg<ETH_MACL4A1R_SPEC>`
pub type ETH_MACL4A1R = crate::Reg<eth_macl4a1r::ETH_MACL4A1R_SPEC>;
///The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\[31:24\]
///(in little-endian mode) or Bits\[7:0\]
///(in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock.
pub mod eth_macl4a1r;
///ETH_MACL3A01R (rw) register accessor: an alias for `Reg<ETH_MACL3A01R_SPEC>`
pub type ETH_MACL3A01R = crate::Reg<eth_macl3a01r::ETH_MACL3A01R_SPEC>;
///For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\[31:0\]
///of the 128-bit IP Source Address or Destination Address field.
pub mod eth_macl3a01r;
///ETH_MACL3A11R (rw) register accessor: an alias for `Reg<ETH_MACL3A11R_SPEC>`
pub type ETH_MACL3A11R = crate::Reg<eth_macl3a11r::ETH_MACL3A11R_SPEC>;
///For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\]
///of the 128-bit IP Source Address or Destination Address field.
pub mod eth_macl3a11r;
///ETH_MACL3A21R (rw) register accessor: an alias for `Reg<ETH_MACL3A21R_SPEC>`
pub type ETH_MACL3A21R = crate::Reg<eth_macl3a21r::ETH_MACL3A21R_SPEC>;
///The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[95:64\]
///of 128-bit IP Source Address or Destination Address field.
pub mod eth_macl3a21r;
///ETH_MACL3A31R (rw) register accessor: an alias for `Reg<ETH_MACL3A31R_SPEC>`
pub type ETH_MACL3A31R = crate::Reg<eth_macl3a31r::ETH_MACL3A31R_SPEC>;
///The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[127:96\]
///of 128-bit IP Source Address or Destination Address field.
pub mod eth_macl3a31r;
///ETH_MACARPAR (rw) register accessor: an alias for `Reg<ETH_MACARPAR_SPEC>`
pub type ETH_MACARPAR = crate::Reg<eth_macarpar::ETH_MACARPAR_SPEC>;
///The ARP Address register contains the IPv4 Destination Address of the MAC.
pub mod eth_macarpar;
///ETH_MACTSCR (rw) register accessor: an alias for `Reg<ETH_MACTSCR_SPEC>`
pub type ETH_MACTSCR = crate::Reg<eth_mactscr::ETH_MACTSCR_SPEC>;
///This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver.
pub mod eth_mactscr;
///ETH_MACSSIR (rw) register accessor: an alias for `Reg<ETH_MACSSIR_SPEC>`
pub type ETH_MACSSIR = crate::Reg<eth_macssir::ETH_MACSSIR_SPEC>;
///The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow.
pub mod eth_macssir;
///ETH_MACSTSR (r) register accessor: an alias for `Reg<ETH_MACSTSR_SPEC>`
pub type ETH_MACSTSR = crate::Reg<eth_macstsr::ETH_MACSTSR_SPEC>;
///The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
pub mod eth_macstsr;
///ETH_MACSTNR (r) register accessor: an alias for `Reg<ETH_MACSTNR_SPEC>`
pub type ETH_MACSTNR = crate::Reg<eth_macstnr::ETH_MACSTNR_SPEC>;
///The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
pub mod eth_macstnr;
///ETH_MACSTSUR (rw) register accessor: an alias for `Reg<ETH_MACSTSUR_SPEC>`
pub type ETH_MACSTSUR = crate::Reg<eth_macstsur::ETH_MACSTSUR_SPEC>;
///The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
pub mod eth_macstsur;
///ETH_MACSTNUR (rw) register accessor: an alias for `Reg<ETH_MACSTNUR_SPEC>`
pub type ETH_MACSTNUR = crate::Reg<eth_macstnur::ETH_MACSTNUR_SPEC>;
///This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input.
pub mod eth_macstnur;
///ETH_MACTSAR (rw) register accessor: an alias for `Reg<ETH_MACTSAR_SPEC>`
pub type ETH_MACTSAR = crate::Reg<eth_mactsar::ETH_MACTSAR_SPEC>;
///The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows.
pub mod eth_mactsar;
///ETH_MACTSSR (r) register accessor: an alias for `Reg<ETH_MACTSSR_SPEC>`
pub type ETH_MACTSSR = crate::Reg<eth_mactssr::ETH_MACTSSR_SPEC>;
///The Timestamp Status register is present only when the IEEE 1588 Timestamp feature is selected. All bits except Bits\[27:25\]
///gets cleared when the application reads this register.
pub mod eth_mactssr;
///ETH_MACTxTSSNR (r) register accessor: an alias for `Reg<ETH_MACTX_TSSNR_SPEC>`
pub type ETH_MACTX_TSSNR = crate::Reg<eth_mactx_tssnr::ETH_MACTX_TSSNR_SPEC>;
///This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled.
pub mod eth_mactx_tssnr;
///ETH_MACTxTSSSR (r) register accessor: an alias for `Reg<ETH_MACTX_TSSSR_SPEC>`
pub type ETH_MACTX_TSSSR = crate::Reg<eth_mactx_tsssr::ETH_MACTX_TSSSR_SPEC>;
///The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted.
pub mod eth_mactx_tsssr;
///ETH_MACACR (rw) register accessor: an alias for `Reg<ETH_MACACR_SPEC>`
pub type ETH_MACACR = crate::Reg<eth_macacr::ETH_MACACR_SPEC>;
///The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot.
pub mod eth_macacr;
///ETH_MACATSNR (r) register accessor: an alias for `Reg<ETH_MACATSNR_SPEC>`
pub type ETH_MACATSNR = crate::Reg<eth_macatsnr::ETH_MACATSNR_SPEC>;
///The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\[29:25\]
///in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\[31:24\]
///are read and in big-endian mode, Bits\[7:0\]
///are read.
pub mod eth_macatsnr;
///ETH_MACATSSR (r) register accessor: an alias for `Reg<ETH_MACATSSR_SPEC>`
pub type ETH_MACATSSR = crate::Reg<eth_macatssr::ETH_MACATSSR_SPEC>;
///The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register.
pub mod eth_macatssr;
///ETH_MACTSIACR (rw) register accessor: an alias for `Reg<ETH_MACTSIACR_SPEC>`
pub type ETH_MACTSIACR = crate::Reg<eth_mactsiacr::ETH_MACTSIACR_SPEC>;
///The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages.
pub mod eth_mactsiacr;
///ETH_MACTSEACR (rw) register accessor: an alias for `Reg<ETH_MACTSEACR_SPEC>`
pub type ETH_MACTSEACR = crate::Reg<eth_mactseacr::ETH_MACTSEACR_SPEC>;
///The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages.
pub mod eth_mactseacr;
///ETH_MACTSICNR (rw) register accessor: an alias for `Reg<ETH_MACTSICNR_SPEC>`
pub type ETH_MACTSICNR = crate::Reg<eth_mactsicnr::ETH_MACTSICNR_SPEC>;
///This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path.
pub mod eth_mactsicnr;
///ETH_MACTSECNR (rw) register accessor: an alias for `Reg<ETH_MACTSECNR_SPEC>`
pub type ETH_MACTSECNR = crate::Reg<eth_mactsecnr::ETH_MACTSECNR_SPEC>;
///This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path.
pub mod eth_mactsecnr;
///ETH_MACPPSCR (rw) register accessor: an alias for `Reg<ETH_MACPPSCR_SPEC>`
pub type ETH_MACPPSCR = crate::Reg<eth_macppscr::ETH_MACPPSCR_SPEC>;
///The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\[30:24\]
///of this register are valid only when four Flexible PPS outputs are selected. Bits\[22:16\]
///are valid only when three or more Flexible PPS outputs are selected. Bits\[14:8\]
///are valid only when two or more Flexible PPS outputs are selected. Bits\[6:4\]
///are valid only when Flexible PPS feature is selected.
pub mod eth_macppscr;
///ETH_MACPPSTTSR (rw) register accessor: an alias for `Reg<ETH_MACPPSTTSR_SPEC>`
pub type ETH_MACPPSTTSR = crate::Reg<eth_macppsttsr::ETH_MACPPSTTSR_SPEC>;
///The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \[Bit 1 of ETH_MACTSSR\]
///when the system time exceeds the value programmed in these registers.
pub mod eth_macppsttsr;
///ETH_MACPPSTTNR (rw) register accessor: an alias for `Reg<ETH_MACPPSTTNR_SPEC>`
pub type ETH_MACPPSTTNR = crate::Reg<eth_macppsttnr::ETH_MACPPSTTNR_SPEC>;
///The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected.
pub mod eth_macppsttnr;
///ETH_MACPPSIR (rw) register accessor: an alias for `Reg<ETH_MACPPSIR_SPEC>`
pub type ETH_MACPPSIR = crate::Reg<eth_macppsir::ETH_MACPPSIR_SPEC>;
///The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\[0\]).
pub mod eth_macppsir;
///ETH_MACPPSWR (rw) register accessor: an alias for `Reg<ETH_MACPPSWR_SPEC>`
pub type ETH_MACPPSWR = crate::Reg<eth_macppswr::ETH_MACPPSWR_SPEC>;
///The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o).
pub mod eth_macppswr;
///ETH_MACPOCR (rw) register accessor: an alias for `Reg<ETH_MACPOCR_SPEC>`
pub type ETH_MACPOCR = crate::Reg<eth_macpocr::ETH_MACPOCR_SPEC>;
///This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected.
pub mod eth_macpocr;
///ETH_MACSPI0R (rw) register accessor: an alias for `Reg<ETH_MACSPI0R_SPEC>`
pub type ETH_MACSPI0R = crate::Reg<eth_macspi0r::ETH_MACSPI0R_SPEC>;
///This register contains Bits\[31:0\]
///of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.
pub mod eth_macspi0r;
///ETH_MACSPI1R (rw) register accessor: an alias for `Reg<ETH_MACSPI1R_SPEC>`
pub type ETH_MACSPI1R = crate::Reg<eth_macspi1r::ETH_MACSPI1R_SPEC>;
///This register contains Bits\[63:32\]
///of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.
pub mod eth_macspi1r;
///ETH_MACSPI2R (rw) register accessor: an alias for `Reg<ETH_MACSPI2R_SPEC>`
pub type ETH_MACSPI2R = crate::Reg<eth_macspi2r::ETH_MACSPI2R_SPEC>;
///This register contains Bits\[79:64\]
///of the 80-bit Source Port Identity of the PTP node.
pub mod eth_macspi2r;
///ETH_MACLMIR (rw) register accessor: an alias for `Reg<ETH_MACLMIR_SPEC>`
pub type ETH_MACLMIR = crate::Reg<eth_maclmir::ETH_MACLMIR_SPEC>;
///This register contains the periodic intervals for automatic PTP packet generation.
pub mod eth_maclmir;
