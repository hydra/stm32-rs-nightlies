///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - The GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.
    pub gotgctl: GOTGCTL,
    ///0x04 - The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.
    pub gotgint: GOTGINT,
    ///0x08 - This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.
    pub gahbcfg: GAHBCFG,
    ///0x0c - This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.
    pub gusbcfg: GUSBCFG,
    ///0x10 - The application uses this register to reset various hardware features inside the core.
    pub grstctl: GRSTCTL,
    ///0x14 - This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.
    pub gintsts: GINTSTS,
    ///0x18 - This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (GINTSTS) register bit corresponding to that interrupt is still set.
    pub gintmsk: GINTMSK,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    ///0x24 - The application can program the RAM size that must be allocated to the Rx FIFO.
    pub grxfsiz: GRXFSIZ,
    ///0x28 - Host mode
    pub hnptxfsiz: HNPTXFSIZ,
    ///0x2c - In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.
    pub hnptxsts: HNPTXSTS,
    _reserved12: [u8; 0x08],
    ///0x38 - OTG general core configuration register
    pub gccfg: GCCFG,
    ///0x3c - This is a register containing the Product ID as reset value.
    pub cid: CID,
    _reserved14: [u8; 0x14],
    ///0x54 - OTG core LPM configuration register
    pub glpmcfg: GLPMCFG,
    _reserved15: [u8; 0xa8],
    ///0x100 - OTG host periodic transmit FIFO size register
    pub hptxfsiz: HPTXFSIZ,
    ///0x104 - OTG device IN endpoint transmit FIFO 1 size register
    pub dieptxf1: DIEPTXF1,
    ///0x108 - OTG device IN endpoint transmit FIFO 2 size register
    pub dieptxf2: DIEPTXF2,
    ///0x10c - OTG device IN endpoint transmit FIFO 3 size register
    pub dieptxf3: DIEPTXF3,
    ///0x110 - OTG device IN endpoint transmit FIFO 4 size register
    pub dieptxf4: DIEPTXF4,
    ///0x114 - OTG device IN endpoint transmit FIFO 5 size register
    pub dieptxf5: DIEPTXF5,
    _reserved21: [u8; 0x02e8],
    ///0x400 - This register configures the core after power-on. Do not make changes to this register after initializing the host.
    pub hcfg: HCFG,
    ///0x404 - This register stores the frame interval information for the current speed to which the OTG controller has enumerated.
    pub hfir: HFIR,
    ///0x408 - This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.
    pub hfnum: HFNUM,
    _reserved24: [u8; 0x04],
    ///0x410 - This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.
    pub hptxsts: HPTXSTS,
    ///0x414 - When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.
    pub haint: HAINT,
    ///0x418 - The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.
    pub haintmsk: HAINTMSK,
    _reserved27: [u8; 0x24],
    ///0x440 - This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.
    pub hprt: HPRT,
    _reserved28: [u8; 0xbc],
    ///0x500 - OTG host channel 0 characteristics register
    pub hcchar0: HCCHAR0,
    _reserved29: [u8; 0x04],
    ///0x508 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint0: HCINT0,
    ///0x50c - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk0: HCINTMSK0,
    ///0x510 - OTG host channel 0 transfer size register
    pub hctsiz0: HCTSIZ0,
    _reserved32: [u8; 0x0c],
    ///0x520 - OTG host channel 1 characteristics register
    pub hcchar1: HCCHAR1,
    _reserved33: [u8; 0x04],
    ///0x528 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint1_device: HCINT1_DEVICE,
    ///0x52c - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk1: HCINTMSK1,
    ///0x530 - OTG host channel 1 transfer size register
    pub hctsiz1: HCTSIZ1,
    _reserved36: [u8; 0x0c],
    ///0x540 - OTG host channel 2 characteristics register
    pub hcchar2: HCCHAR2,
    _reserved37: [u8; 0x04],
    ///0x548 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint2: HCINT2,
    ///0x54c - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk2: HCINTMSK2,
    ///0x550 - OTG host channel 2 transfer size register
    pub hctsiz2: HCTSIZ2,
    _reserved40: [u8; 0x0c],
    ///0x560 - OTG host channel 3 characteristics register
    pub hcchar3: HCCHAR3,
    _reserved41: [u8; 0x04],
    ///0x568 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint3: HCINT3,
    ///0x56c - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk3: HCINTMSK3,
    ///0x570 - OTG host channel 3 transfer size register
    pub hctsiz3: HCTSIZ3,
    _reserved44: [u8; 0x0c],
    ///0x580 - OTG host channel 4 characteristics register
    pub hcchar4: HCCHAR4,
    _reserved45: [u8; 0x04],
    ///0x588 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint4: HCINT4,
    ///0x58c - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk4: HCINTMSK4,
    ///0x590 - OTG host channel 4 transfer size register
    pub hctsiz4: HCTSIZ4,
    _reserved48: [u8; 0x0c],
    ///0x5a0 - OTG host channel 5 characteristics register
    pub hcchar5: HCCHAR5,
    _reserved49: [u8; 0x04],
    ///0x5a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint5: HCINT5,
    ///0x5ac - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk5: HCINTMSK5,
    ///0x5b0 - OTG host channel 5 transfer size register
    pub hctsiz5: HCTSIZ5,
    _reserved52: [u8; 0x0c],
    ///0x5c0 - OTG host channel 6 characteristics register
    pub hcchar6: HCCHAR6,
    _reserved53: [u8; 0x04],
    ///0x5c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint6: HCINT6,
    ///0x5cc - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk6: HCINTMSK6,
    ///0x5d0 - OTG host channel 6 transfer size register
    pub hctsiz6: HCTSIZ6,
    _reserved56: [u8; 0x0c],
    ///0x5e0 - OTG host channel 7 characteristics register
    pub hcchar7: HCCHAR7,
    _reserved57: [u8; 0x04],
    ///0x5e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint7: HCINT7,
    ///0x5ec - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk7: HCINTMSK7,
    ///0x5f0 - OTG host channel 7 transfer size register
    pub hctsiz7: HCTSIZ7,
    _reserved60: [u8; 0x0c],
    ///0x600 - OTG host channel 8 characteristics register
    pub hcchar8: HCCHAR8,
    _reserved61: [u8; 0x04],
    ///0x608 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint8: HCINT8,
    ///0x60c - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk8: HCINTMSK8,
    ///0x610 - OTG host channel 8 transfer size register
    pub hctsiz8: HCTSIZ8,
    _reserved64: [u8; 0x0c],
    ///0x620 - OTG host channel 9 characteristics register
    pub hcchar9: HCCHAR9,
    _reserved65: [u8; 0x04],
    ///0x628 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint9: HCINT9,
    ///0x62c - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk9: HCINTMSK9,
    ///0x630 - OTG host channel 9 transfer size register
    pub hctsiz9: HCTSIZ9,
    _reserved68: [u8; 0x0c],
    ///0x640 - OTG host channel 10 characteristics register
    pub hcchar10: HCCHAR10,
    _reserved69: [u8; 0x04],
    ///0x648 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint10: HCINT10,
    ///0x64c - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk10: HCINTMSK10,
    ///0x650 - OTG host channel 10 transfer size register
    pub hctsiz10: HCTSIZ10,
    _reserved72: [u8; 0x0c],
    ///0x660 - OTG host channel 11 characteristics register
    pub hcchar11: HCCHAR11,
    _reserved73: [u8; 0x04],
    ///0x668 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    pub hcint11: HCINT11,
    ///0x66c - This register reflects the mask for each channel status described in the previous section.
    pub hcintmsk11: HCINTMSK11,
    ///0x670 - OTG host channel 11 transfer size register
    pub hctsiz11: HCTSIZ11,
    _reserved76: [u8; 0x018c],
    ///0x800 - This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.
    pub dcfg: DCFG,
    ///0x804 - OTG device control register
    pub dctl: DCTL,
    ///0x808 - This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (DAINT) register.
    pub dsts: DSTS,
    _reserved79: [u8; 0x04],
    ///0x810 - This register works with each of the DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.
    pub diepmsk: DIEPMSK,
    ///0x814 - This register works with each of the DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.
    pub doepmsk: DOEPMSK,
    ///0x818 - When a significant event occurs on an endpoint, a DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the GINTSTS register (OEPINT or IEPINT in GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (DIEPINTx/DOEPINTx).
    pub daint: DAINT,
    ///0x81c - The DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the DAINT register bit corresponding to that interrupt is still set.
    pub daintmsk: DAINTMSK,
    _reserved83: [u8; 0x08],
    ///0x828 - This register specifies the VBUS discharge time after VBUS pulsing during SRP.
    pub dvbusdis: DVBUSDIS,
    ///0x82c - This register specifies the VBUS pulsing time during SRP.
    pub dvbuspulse: DVBUSPULSE,
    _reserved85: [u8; 0x04],
    ///0x834 - This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_DIEPINTx).
    pub diepempmsk: DIEPEMPMSK,
    _reserved86: [u8; 0xc8],
    ///0x900 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub diepctl0: DIEPCTL0,
    _reserved87: [u8; 0x04],
    ///0x908 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub diepint0: DIEPINT0,
    _reserved88: [u8; 0x04],
    ///0x910 - The application must modify this register before enabling endpoint 0.
    pub dieptsiz0: DIEPTSIZ0,
    _reserved89: [u8; 0x04],
    ///0x918 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub dtxfsts0: DTXFSTS0,
    _reserved90: [u8; 0x04],
    ///0x920 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub diepctl1: DIEPCTL1,
    _reserved91: [u8; 0x04],
    ///0x928 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub diepint1: DIEPINT1,
    _reserved92: [u8; 0x04],
    ///0x930 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub dieptsiz1: DIEPTSIZ1,
    ///0x934 - OTG device IN endpoint 1 DMA address register
    pub diepdma1: DIEPDMA1,
    ///0x938 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub dtxfsts1: DTXFSTS1,
    _reserved95: [u8; 0x04],
    ///0x940 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub diepctl2: DIEPCTL2,
    _reserved96: [u8; 0x04],
    ///0x948 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub diepint2: DIEPINT2,
    _reserved97: [u8; 0x04],
    ///0x950 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub dieptsiz2: DIEPTSIZ2,
    ///0x954 - OTG device IN endpoint 2 DMA address register
    pub diepdma2: DIEPDMA2,
    ///0x958 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub dtxfsts2: DTXFSTS2,
    _reserved100: [u8; 0x04],
    ///0x960 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub diepctl3: DIEPCTL3,
    _reserved101: [u8; 0x04],
    ///0x968 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub diepint3: DIEPINT3,
    _reserved102: [u8; 0x04],
    ///0x970 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub dieptsiz3: DIEPTSIZ3,
    ///0x974 - OTG device IN endpoint 3 DMA address register
    pub diepdma3: DIEPDMA3,
    ///0x978 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub dtxfsts3: DTXFSTS3,
    _reserved105: [u8; 0x04],
    ///0x980 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub diepctl4: DIEPCTL4,
    _reserved106: [u8; 0x04],
    ///0x988 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub diepint4: DIEPINT4,
    _reserved107: [u8; 0x04],
    ///0x990 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub dieptsiz4: DIEPTSIZ4,
    ///0x994 - OTG device IN endpoint 4 DMA address register
    pub diepdma4: DIEPDMA4,
    ///0x998 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub dtxfsts4: DTXFSTS4,
    _reserved110: [u8; 0x04],
    ///0x9a0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub diepctl5: DIEPCTL5,
    _reserved111: [u8; 0x04],
    ///0x9a8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub diepint5: DIEPINT5,
    _reserved112: [u8; 0x04],
    ///0x9b0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub dieptsiz5: DIEPTSIZ5,
    ///0x9b4 - OTG device IN endpoint 5 DMA address register
    pub diepdma5: DIEPDMA5,
    ///0x9b8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub dtxfsts5: DTXFSTS5,
    _reserved115: [u8; 0x0c],
    ///0x9c8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub diepint6: DIEPINT6,
    _reserved116: [u8; 0x04],
    ///0x9d0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub dieptsiz6: DIEPTSIZ6,
    ///0x9d4 - OTG device IN endpoint 6 DMA address register
    pub diepdma6: DIEPDMA6,
    _reserved118: [u8; 0x10],
    ///0x9e8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub diepint7: DIEPINT7,
    _reserved119: [u8; 0x04],
    ///0x9f0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub dieptsiz7: DIEPTSIZ7,
    ///0x9f4 - OTG device IN endpoint 7 DMA address register
    pub diepdma7: DIEPDMA7,
    _reserved121: [u8; 0x10],
    ///0xa08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub diepint8: DIEPINT8,
    _reserved122: [u8; 0x04],
    ///0xa10 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub dieptsiz8: DIEPTSIZ8,
    ///0xa14 - OTG device IN endpoint 8 DMA address register
    pub diepdma8: DIEPDMA8,
    _reserved124: [u8; 0xe8],
    ///0xb00 - This section describes the DOEPCTL0 register.
    pub doepctl0: DOEPCTL0,
    _reserved125: [u8; 0x04],
    ///0xb08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub doepint0: DOEPINT0,
    _reserved126: [u8; 0x04],
    ///0xb10 - The application must modify this register before enabling endpoint 0.
    pub doeptsiz0: DOEPTSIZ0,
    ///0xb14 - OTG device OUT endpoint 0 DMA address register
    pub doepdma0: DOEPDMA0,
    _reserved128: [u8; 0x08],
    ///0xb20 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub doepctl1: DOEPCTL1,
    _reserved129: [u8; 0x04],
    ///0xb28 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub doepint1: DOEPINT1,
    _reserved130: [u8; 0x04],
    ///0xb30 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub doeptsiz1: DOEPTSIZ1,
    ///0xb34 - OTG device OUT endpoint 1 DMA address register
    pub doepdma1: DOEPDMA1,
    _reserved132: [u8; 0x08],
    ///0xb40 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub doepctl2: DOEPCTL2,
    _reserved133: [u8; 0x04],
    ///0xb48 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub doepint2: DOEPINT2,
    _reserved134: [u8; 0x04],
    ///0xb50 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub doeptsiz2: DOEPTSIZ2,
    ///0xb54 - OTG device OUT endpoint 2 DMA address register
    pub doepdma2: DOEPDMA2,
    _reserved136: [u8; 0x08],
    ///0xb60 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub doepctl3: DOEPCTL3,
    _reserved137: [u8; 0x04],
    ///0xb68 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub doepint3: DOEPINT3,
    _reserved138: [u8; 0x04],
    ///0xb70 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub doeptsiz3: DOEPTSIZ3,
    ///0xb74 - OTG device OUT endpoint 3 DMA address register
    pub doepdma3: DOEPDMA3,
    _reserved140: [u8; 0x08],
    ///0xb80 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub doepctl4: DOEPCTL4,
    _reserved141: [u8; 0x04],
    ///0xb88 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub doepint4: DOEPINT4,
    _reserved142: [u8; 0x04],
    ///0xb90 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub doeptsiz4: DOEPTSIZ4,
    ///0xb94 - OTG device OUT endpoint 4 DMA address register
    pub doepdma4: DOEPDMA4,
    _reserved144: [u8; 0x08],
    ///0xba0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub doepctl5: DOEPCTL5,
    _reserved145: [u8; 0x04],
    ///0xba8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub doepint5: DOEPINT5,
    _reserved146: [u8; 0x04],
    ///0xbb0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub doeptsiz5: DOEPTSIZ5,
    ///0xbb4 - OTG device OUT endpoint 5 DMA address register
    pub doepdma5: DOEPDMA5,
    _reserved148: [u8; 0x08],
    ///0xbc0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub doepctl6: DOEPCTL6,
    _reserved149: [u8; 0x04],
    ///0xbc8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub doepint6: DOEPINT6,
    _reserved150: [u8; 0x04],
    ///0xbd0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub doeptsiz6: DOEPTSIZ6,
    ///0xbd4 - OTG device OUT endpoint 6 DMA address register
    pub doepdma6: DOEPDMA6,
    _reserved152: [u8; 0x08],
    ///0xbe0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub doepctl7: DOEPCTL7,
    _reserved153: [u8; 0x04],
    ///0xbe8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub doepint7: DOEPINT7,
    _reserved154: [u8; 0x04],
    ///0xbf0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub doeptsiz7: DOEPTSIZ7,
    ///0xbf4 - OTG device OUT endpoint 7 DMA address register
    pub doepdma7: DOEPDMA7,
    _reserved156: [u8; 0x08],
    ///0xc00 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub doepctl8: DOEPCTL8,
    _reserved157: [u8; 0x04],
    ///0xc08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    pub doepint8: DOEPINT8,
    _reserved158: [u8; 0x04],
    ///0xc10 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub doeptsiz8: DOEPTSIZ8,
    ///0xc14 - OTG device OUT endpoint 8 DMA address register
    pub doepdma8: DOEPDMA8,
    _reserved160: [u8; 0x01e8],
    ///0xe00 - This register is available in host and device modes.
    pub pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    ///0x1c - This description is for register GRXSTSR in Host mode
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - This description is for register GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000.
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x20 - This description is for register GRXSTSP in HOST mode
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    ///0x20 - This description is for register GRXSTSP in Device mode. Similarly to GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in GINTSTS) is asserted.
    #[inline(always)]
    pub const fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
}
///GOTGCTL (rw) register accessor: an alias for `Reg<GOTGCTL_SPEC>`
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
///The GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.
pub mod gotgctl;
///GOTGINT (rw) register accessor: an alias for `Reg<GOTGINT_SPEC>`
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
///The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.
pub mod gotgint;
///GAHBCFG (rw) register accessor: an alias for `Reg<GAHBCFG_SPEC>`
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
///This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.
pub mod gahbcfg;
///GUSBCFG (rw) register accessor: an alias for `Reg<GUSBCFG_SPEC>`
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
///This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.
pub mod gusbcfg;
///GRSTCTL (rw) register accessor: an alias for `Reg<GRSTCTL_SPEC>`
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
///The application uses this register to reset various hardware features inside the core.
pub mod grstctl;
///GINTSTS (rw) register accessor: an alias for `Reg<GINTSTS_SPEC>`
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
///This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.
pub mod gintsts;
///GINTMSK (rw) register accessor: an alias for `Reg<GINTMSK_SPEC>`
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
///This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (GINTSTS) register bit corresponding to that interrupt is still set.
pub mod gintmsk;
///GRXSTSR_DEVICE (r) register accessor: an alias for `Reg<GRXSTSR_DEVICE_SPEC>`
pub type GRXSTSR_DEVICE = crate::Reg<grxstsr_device::GRXSTSR_DEVICE_SPEC>;
///This description is for register GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000.
pub mod grxstsr_device;
///GRXSTSR_HOST (r) register accessor: an alias for `Reg<GRXSTSR_HOST_SPEC>`
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOST_SPEC>;
///This description is for register GRXSTSR in Host mode
pub mod grxstsr_host;
///GRXSTSP_DEVICE (r) register accessor: an alias for `Reg<GRXSTSP_DEVICE_SPEC>`
pub type GRXSTSP_DEVICE = crate::Reg<grxstsp_device::GRXSTSP_DEVICE_SPEC>;
///This description is for register GRXSTSP in Device mode. Similarly to GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in GINTSTS) is asserted.
pub mod grxstsp_device;
///GRXSTSP_HOST (r) register accessor: an alias for `Reg<GRXSTSP_HOST_SPEC>`
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOST_SPEC>;
///This description is for register GRXSTSP in HOST mode
pub mod grxstsp_host;
///GRXFSIZ (rw) register accessor: an alias for `Reg<GRXFSIZ_SPEC>`
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
///The application can program the RAM size that must be allocated to the Rx FIFO.
pub mod grxfsiz;
///HNPTXFSIZ (rw) register accessor: an alias for `Reg<HNPTXFSIZ_SPEC>`
pub type HNPTXFSIZ = crate::Reg<hnptxfsiz::HNPTXFSIZ_SPEC>;
///Host mode
pub mod hnptxfsiz;
///HNPTXSTS (r) register accessor: an alias for `Reg<HNPTXSTS_SPEC>`
pub type HNPTXSTS = crate::Reg<hnptxsts::HNPTXSTS_SPEC>;
///In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.
pub mod hnptxsts;
///GCCFG (rw) register accessor: an alias for `Reg<GCCFG_SPEC>`
pub type GCCFG = crate::Reg<gccfg::GCCFG_SPEC>;
///OTG general core configuration register
pub mod gccfg;
///CID (rw) register accessor: an alias for `Reg<CID_SPEC>`
pub type CID = crate::Reg<cid::CID_SPEC>;
///This is a register containing the Product ID as reset value.
pub mod cid;
///GLPMCFG (rw) register accessor: an alias for `Reg<GLPMCFG_SPEC>`
pub type GLPMCFG = crate::Reg<glpmcfg::GLPMCFG_SPEC>;
///OTG core LPM configuration register
pub mod glpmcfg;
///HPTXFSIZ (rw) register accessor: an alias for `Reg<HPTXFSIZ_SPEC>`
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
///OTG host periodic transmit FIFO size register
pub mod hptxfsiz;
///DIEPTXF1 (rw) register accessor: an alias for `Reg<DIEPTXF1_SPEC>`
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
///OTG device IN endpoint transmit FIFO 1 size register
pub mod dieptxf1;
///DIEPTXF2 (rw) register accessor: an alias for `Reg<DIEPTXF2_SPEC>`
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
///OTG device IN endpoint transmit FIFO 2 size register
pub mod dieptxf2;
///DIEPTXF3 (rw) register accessor: an alias for `Reg<DIEPTXF3_SPEC>`
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
///OTG device IN endpoint transmit FIFO 3 size register
pub mod dieptxf3;
///DIEPTXF4 (rw) register accessor: an alias for `Reg<DIEPTXF4_SPEC>`
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
///OTG device IN endpoint transmit FIFO 4 size register
pub mod dieptxf4;
///DIEPTXF5 (rw) register accessor: an alias for `Reg<DIEPTXF5_SPEC>`
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
///OTG device IN endpoint transmit FIFO 5 size register
pub mod dieptxf5;
///HCFG (rw) register accessor: an alias for `Reg<HCFG_SPEC>`
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
///This register configures the core after power-on. Do not make changes to this register after initializing the host.
pub mod hcfg;
///HFIR (rw) register accessor: an alias for `Reg<HFIR_SPEC>`
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
///This register stores the frame interval information for the current speed to which the OTG controller has enumerated.
pub mod hfir;
///HFNUM (r) register accessor: an alias for `Reg<HFNUM_SPEC>`
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
///This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.
pub mod hfnum;
///HPTXSTS (r) register accessor: an alias for `Reg<HPTXSTS_SPEC>`
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
///This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.
pub mod hptxsts;
///HAINT (r) register accessor: an alias for `Reg<HAINT_SPEC>`
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
///When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.
pub mod haint;
///HAINTMSK (rw) register accessor: an alias for `Reg<HAINTMSK_SPEC>`
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
///The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.
pub mod haintmsk;
///HPRT (rw) register accessor: an alias for `Reg<HPRT_SPEC>`
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
///This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.
pub mod hprt;
///HCCHAR0 (rw) register accessor: an alias for `Reg<HCCHAR0_SPEC>`
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0_SPEC>;
///OTG host channel 0 characteristics register
pub mod hcchar0;
///HCINT0 (rw) register accessor: an alias for `Reg<HCINT0_SPEC>`
pub type HCINT0 = crate::Reg<hcint0::HCINT0_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint0;
///HCINTMSK0 (rw) register accessor: an alias for `Reg<HCINTMSK0_SPEC>`
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk0;
///HCTSIZ0 (rw) register accessor: an alias for `Reg<HCTSIZ0_SPEC>`
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0_SPEC>;
///OTG host channel 0 transfer size register
pub mod hctsiz0;
///HCCHAR1 (rw) register accessor: an alias for `Reg<HCCHAR1_SPEC>`
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1_SPEC>;
///OTG host channel 1 characteristics register
pub mod hcchar1;
///HCINT1_DEVICE (rw) register accessor: an alias for `Reg<HCINT1_DEVICE_SPEC>`
pub type HCINT1_DEVICE = crate::Reg<hcint1_device::HCINT1_DEVICE_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint1_device;
///HCINTMSK1 (rw) register accessor: an alias for `Reg<HCINTMSK1_SPEC>`
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk1;
///HCTSIZ1 (rw) register accessor: an alias for `Reg<HCTSIZ1_SPEC>`
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1_SPEC>;
///OTG host channel 1 transfer size register
pub mod hctsiz1;
///HCCHAR2 (rw) register accessor: an alias for `Reg<HCCHAR2_SPEC>`
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2_SPEC>;
///OTG host channel 2 characteristics register
pub mod hcchar2;
///HCINT2 (rw) register accessor: an alias for `Reg<HCINT2_SPEC>`
pub type HCINT2 = crate::Reg<hcint2::HCINT2_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint2;
///HCINTMSK2 (rw) register accessor: an alias for `Reg<HCINTMSK2_SPEC>`
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk2;
///HCTSIZ2 (rw) register accessor: an alias for `Reg<HCTSIZ2_SPEC>`
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2_SPEC>;
///OTG host channel 2 transfer size register
pub mod hctsiz2;
///HCCHAR3 (rw) register accessor: an alias for `Reg<HCCHAR3_SPEC>`
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3_SPEC>;
///OTG host channel 3 characteristics register
pub mod hcchar3;
///HCINT3 (rw) register accessor: an alias for `Reg<HCINT3_SPEC>`
pub type HCINT3 = crate::Reg<hcint3::HCINT3_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint3;
///HCINTMSK3 (rw) register accessor: an alias for `Reg<HCINTMSK3_SPEC>`
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk3;
///HCTSIZ3 (rw) register accessor: an alias for `Reg<HCTSIZ3_SPEC>`
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3_SPEC>;
///OTG host channel 3 transfer size register
pub mod hctsiz3;
///HCCHAR4 (rw) register accessor: an alias for `Reg<HCCHAR4_SPEC>`
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4_SPEC>;
///OTG host channel 4 characteristics register
pub mod hcchar4;
///HCINT4 (rw) register accessor: an alias for `Reg<HCINT4_SPEC>`
pub type HCINT4 = crate::Reg<hcint4::HCINT4_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint4;
///HCINTMSK4 (rw) register accessor: an alias for `Reg<HCINTMSK4_SPEC>`
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk4;
///HCTSIZ4 (rw) register accessor: an alias for `Reg<HCTSIZ4_SPEC>`
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4_SPEC>;
///OTG host channel 4 transfer size register
pub mod hctsiz4;
///HCCHAR5 (rw) register accessor: an alias for `Reg<HCCHAR5_SPEC>`
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5_SPEC>;
///OTG host channel 5 characteristics register
pub mod hcchar5;
///HCINT5 (rw) register accessor: an alias for `Reg<HCINT5_SPEC>`
pub type HCINT5 = crate::Reg<hcint5::HCINT5_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint5;
///HCINTMSK5 (rw) register accessor: an alias for `Reg<HCINTMSK5_SPEC>`
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk5;
///HCTSIZ5 (rw) register accessor: an alias for `Reg<HCTSIZ5_SPEC>`
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5_SPEC>;
///OTG host channel 5 transfer size register
pub mod hctsiz5;
///HCCHAR6 (rw) register accessor: an alias for `Reg<HCCHAR6_SPEC>`
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6_SPEC>;
///OTG host channel 6 characteristics register
pub mod hcchar6;
///HCINT6 (rw) register accessor: an alias for `Reg<HCINT6_SPEC>`
pub type HCINT6 = crate::Reg<hcint6::HCINT6_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint6;
///HCINTMSK6 (rw) register accessor: an alias for `Reg<HCINTMSK6_SPEC>`
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk6;
///HCTSIZ6 (rw) register accessor: an alias for `Reg<HCTSIZ6_SPEC>`
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6_SPEC>;
///OTG host channel 6 transfer size register
pub mod hctsiz6;
///HCCHAR7 (rw) register accessor: an alias for `Reg<HCCHAR7_SPEC>`
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7_SPEC>;
///OTG host channel 7 characteristics register
pub mod hcchar7;
///HCINT7 (rw) register accessor: an alias for `Reg<HCINT7_SPEC>`
pub type HCINT7 = crate::Reg<hcint7::HCINT7_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint7;
///HCINTMSK7 (rw) register accessor: an alias for `Reg<HCINTMSK7_SPEC>`
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk7;
///HCTSIZ7 (rw) register accessor: an alias for `Reg<HCTSIZ7_SPEC>`
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7_SPEC>;
///OTG host channel 7 transfer size register
pub mod hctsiz7;
///HCCHAR8 (rw) register accessor: an alias for `Reg<HCCHAR8_SPEC>`
pub type HCCHAR8 = crate::Reg<hcchar8::HCCHAR8_SPEC>;
///OTG host channel 8 characteristics register
pub mod hcchar8;
///HCINT8 (rw) register accessor: an alias for `Reg<HCINT8_SPEC>`
pub type HCINT8 = crate::Reg<hcint8::HCINT8_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint8;
///HCINTMSK8 (rw) register accessor: an alias for `Reg<HCINTMSK8_SPEC>`
pub type HCINTMSK8 = crate::Reg<hcintmsk8::HCINTMSK8_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk8;
///HCTSIZ8 (rw) register accessor: an alias for `Reg<HCTSIZ8_SPEC>`
pub type HCTSIZ8 = crate::Reg<hctsiz8::HCTSIZ8_SPEC>;
///OTG host channel 8 transfer size register
pub mod hctsiz8;
///HCCHAR9 (rw) register accessor: an alias for `Reg<HCCHAR9_SPEC>`
pub type HCCHAR9 = crate::Reg<hcchar9::HCCHAR9_SPEC>;
///OTG host channel 9 characteristics register
pub mod hcchar9;
///HCINT9 (rw) register accessor: an alias for `Reg<HCINT9_SPEC>`
pub type HCINT9 = crate::Reg<hcint9::HCINT9_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint9;
///HCINTMSK9 (rw) register accessor: an alias for `Reg<HCINTMSK9_SPEC>`
pub type HCINTMSK9 = crate::Reg<hcintmsk9::HCINTMSK9_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk9;
///HCTSIZ9 (rw) register accessor: an alias for `Reg<HCTSIZ9_SPEC>`
pub type HCTSIZ9 = crate::Reg<hctsiz9::HCTSIZ9_SPEC>;
///OTG host channel 9 transfer size register
pub mod hctsiz9;
///HCCHAR10 (rw) register accessor: an alias for `Reg<HCCHAR10_SPEC>`
pub type HCCHAR10 = crate::Reg<hcchar10::HCCHAR10_SPEC>;
///OTG host channel 10 characteristics register
pub mod hcchar10;
///HCINT10 (rw) register accessor: an alias for `Reg<HCINT10_SPEC>`
pub type HCINT10 = crate::Reg<hcint10::HCINT10_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint10;
///HCINTMSK10 (rw) register accessor: an alias for `Reg<HCINTMSK10_SPEC>`
pub type HCINTMSK10 = crate::Reg<hcintmsk10::HCINTMSK10_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk10;
///HCTSIZ10 (rw) register accessor: an alias for `Reg<HCTSIZ10_SPEC>`
pub type HCTSIZ10 = crate::Reg<hctsiz10::HCTSIZ10_SPEC>;
///OTG host channel 10 transfer size register
pub mod hctsiz10;
///HCCHAR11 (rw) register accessor: an alias for `Reg<HCCHAR11_SPEC>`
pub type HCCHAR11 = crate::Reg<hcchar11::HCCHAR11_SPEC>;
///OTG host channel 11 characteristics register
pub mod hcchar11;
///HCINT11 (rw) register accessor: an alias for `Reg<HCINT11_SPEC>`
pub type HCINT11 = crate::Reg<hcint11::HCINT11_SPEC>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint11;
///HCINTMSK11 (rw) register accessor: an alias for `Reg<HCINTMSK11_SPEC>`
pub type HCINTMSK11 = crate::Reg<hcintmsk11::HCINTMSK11_SPEC>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk11;
///HCTSIZ11 (rw) register accessor: an alias for `Reg<HCTSIZ11_SPEC>`
pub type HCTSIZ11 = crate::Reg<hctsiz11::HCTSIZ11_SPEC>;
///OTG host channel 11 transfer size register
pub mod hctsiz11;
///DCFG (rw) register accessor: an alias for `Reg<DCFG_SPEC>`
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
///This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.
pub mod dcfg;
///DCTL (rw) register accessor: an alias for `Reg<DCTL_SPEC>`
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
///OTG device control register
pub mod dctl;
///DSTS (r) register accessor: an alias for `Reg<DSTS_SPEC>`
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
///This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (DAINT) register.
pub mod dsts;
///DIEPMSK (rw) register accessor: an alias for `Reg<DIEPMSK_SPEC>`
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
///This register works with each of the DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.
pub mod diepmsk;
///DOEPMSK (rw) register accessor: an alias for `Reg<DOEPMSK_SPEC>`
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
///This register works with each of the DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.
pub mod doepmsk;
///DAINT (r) register accessor: an alias for `Reg<DAINT_SPEC>`
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
///When a significant event occurs on an endpoint, a DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the GINTSTS register (OEPINT or IEPINT in GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (DIEPINTx/DOEPINTx).
pub mod daint;
///DAINTMSK (rw) register accessor: an alias for `Reg<DAINTMSK_SPEC>`
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
///The DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the DAINT register bit corresponding to that interrupt is still set.
pub mod daintmsk;
///DVBUSDIS (rw) register accessor: an alias for `Reg<DVBUSDIS_SPEC>`
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
///This register specifies the VBUS discharge time after VBUS pulsing during SRP.
pub mod dvbusdis;
///DVBUSPULSE (rw) register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
///This register specifies the VBUS pulsing time during SRP.
pub mod dvbuspulse;
///DIEPEMPMSK (rw) register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
///This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_DIEPINTx).
pub mod diepempmsk;
///DIEPCTL0 (rw) register accessor: an alias for `Reg<DIEPCTL0_SPEC>`
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl0;
///DIEPINT0 (rw) register accessor: an alias for `Reg<DIEPINT0_SPEC>`
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint0;
///DIEPTSIZ0 (rw) register accessor: an alias for `Reg<DIEPTSIZ0_SPEC>`
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
///The application must modify this register before enabling endpoint 0.
pub mod dieptsiz0;
///DTXFSTS0 (r) register accessor: an alias for `Reg<DTXFSTS0_SPEC>`
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts0;
///DIEPCTL1 (rw) register accessor: an alias for `Reg<DIEPCTL1_SPEC>`
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl1;
///DIEPINT1 (rw) register accessor: an alias for `Reg<DIEPINT1_SPEC>`
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint1;
///DIEPTSIZ1 (rw) register accessor: an alias for `Reg<DIEPTSIZ1_SPEC>`
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz1;
///DIEPDMA1 (rw) register accessor: an alias for `Reg<DIEPDMA1_SPEC>`
pub type DIEPDMA1 = crate::Reg<diepdma1::DIEPDMA1_SPEC>;
///OTG device IN endpoint 1 DMA address register
pub mod diepdma1;
///DTXFSTS1 (r) register accessor: an alias for `Reg<DTXFSTS1_SPEC>`
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1_SPEC>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts1;
///DIEPCTL2 (rw) register accessor: an alias for `Reg<DIEPCTL2_SPEC>`
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl2;
///DIEPINT2 (rw) register accessor: an alias for `Reg<DIEPINT2_SPEC>`
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint2;
///DIEPTSIZ2 (rw) register accessor: an alias for `Reg<DIEPTSIZ2_SPEC>`
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz2;
///DIEPDMA2 (rw) register accessor: an alias for `Reg<DIEPDMA2_SPEC>`
pub type DIEPDMA2 = crate::Reg<diepdma2::DIEPDMA2_SPEC>;
///OTG device IN endpoint 2 DMA address register
pub mod diepdma2;
///DTXFSTS2 (r) register accessor: an alias for `Reg<DTXFSTS2_SPEC>`
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2_SPEC>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts2;
///DIEPCTL3 (rw) register accessor: an alias for `Reg<DIEPCTL3_SPEC>`
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl3;
///DIEPINT3 (rw) register accessor: an alias for `Reg<DIEPINT3_SPEC>`
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint3;
///DIEPTSIZ3 (rw) register accessor: an alias for `Reg<DIEPTSIZ3_SPEC>`
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz3;
///DIEPDMA3 (rw) register accessor: an alias for `Reg<DIEPDMA3_SPEC>`
pub type DIEPDMA3 = crate::Reg<diepdma3::DIEPDMA3_SPEC>;
///OTG device IN endpoint 3 DMA address register
pub mod diepdma3;
///DTXFSTS3 (r) register accessor: an alias for `Reg<DTXFSTS3_SPEC>`
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3_SPEC>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts3;
///DIEPCTL4 (rw) register accessor: an alias for `Reg<DIEPCTL4_SPEC>`
pub type DIEPCTL4 = crate::Reg<diepctl4::DIEPCTL4_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl4;
///DIEPINT4 (rw) register accessor: an alias for `Reg<DIEPINT4_SPEC>`
pub type DIEPINT4 = crate::Reg<diepint4::DIEPINT4_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint4;
///DIEPTSIZ4 (rw) register accessor: an alias for `Reg<DIEPTSIZ4_SPEC>`
pub type DIEPTSIZ4 = crate::Reg<dieptsiz4::DIEPTSIZ4_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz4;
///DIEPDMA4 (rw) register accessor: an alias for `Reg<DIEPDMA4_SPEC>`
pub type DIEPDMA4 = crate::Reg<diepdma4::DIEPDMA4_SPEC>;
///OTG device IN endpoint 4 DMA address register
pub mod diepdma4;
///DTXFSTS4 (r) register accessor: an alias for `Reg<DTXFSTS4_SPEC>`
pub type DTXFSTS4 = crate::Reg<dtxfsts4::DTXFSTS4_SPEC>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts4;
///DIEPCTL5 (rw) register accessor: an alias for `Reg<DIEPCTL5_SPEC>`
pub type DIEPCTL5 = crate::Reg<diepctl5::DIEPCTL5_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl5;
///DIEPINT5 (rw) register accessor: an alias for `Reg<DIEPINT5_SPEC>`
pub type DIEPINT5 = crate::Reg<diepint5::DIEPINT5_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint5;
///DIEPTSIZ5 (rw) register accessor: an alias for `Reg<DIEPTSIZ5_SPEC>`
pub type DIEPTSIZ5 = crate::Reg<dieptsiz5::DIEPTSIZ5_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz5;
///DIEPDMA5 (rw) register accessor: an alias for `Reg<DIEPDMA5_SPEC>`
pub type DIEPDMA5 = crate::Reg<diepdma5::DIEPDMA5_SPEC>;
///OTG device IN endpoint 5 DMA address register
pub mod diepdma5;
///DTXFSTS5 (r) register accessor: an alias for `Reg<DTXFSTS5_SPEC>`
pub type DTXFSTS5 = crate::Reg<dtxfsts5::DTXFSTS5_SPEC>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts5;
///DIEPINT6 (rw) register accessor: an alias for `Reg<DIEPINT6_SPEC>`
pub type DIEPINT6 = crate::Reg<diepint6::DIEPINT6_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint6;
///DIEPTSIZ6 (rw) register accessor: an alias for `Reg<DIEPTSIZ6_SPEC>`
pub type DIEPTSIZ6 = crate::Reg<dieptsiz6::DIEPTSIZ6_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz6;
///DIEPDMA6 (rw) register accessor: an alias for `Reg<DIEPDMA6_SPEC>`
pub type DIEPDMA6 = crate::Reg<diepdma6::DIEPDMA6_SPEC>;
///OTG device IN endpoint 6 DMA address register
pub mod diepdma6;
///DIEPINT7 (rw) register accessor: an alias for `Reg<DIEPINT7_SPEC>`
pub type DIEPINT7 = crate::Reg<diepint7::DIEPINT7_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint7;
///DIEPTSIZ7 (rw) register accessor: an alias for `Reg<DIEPTSIZ7_SPEC>`
pub type DIEPTSIZ7 = crate::Reg<dieptsiz7::DIEPTSIZ7_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz7;
///DIEPDMA7 (rw) register accessor: an alias for `Reg<DIEPDMA7_SPEC>`
pub type DIEPDMA7 = crate::Reg<diepdma7::DIEPDMA7_SPEC>;
///OTG device IN endpoint 7 DMA address register
pub mod diepdma7;
///DIEPINT8 (rw) register accessor: an alias for `Reg<DIEPINT8_SPEC>`
pub type DIEPINT8 = crate::Reg<diepint8::DIEPINT8_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint8;
///DIEPTSIZ8 (rw) register accessor: an alias for `Reg<DIEPTSIZ8_SPEC>`
pub type DIEPTSIZ8 = crate::Reg<dieptsiz8::DIEPTSIZ8_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz8;
///DIEPDMA8 (rw) register accessor: an alias for `Reg<DIEPDMA8_SPEC>`
pub type DIEPDMA8 = crate::Reg<diepdma8::DIEPDMA8_SPEC>;
///OTG device IN endpoint 8 DMA address register
pub mod diepdma8;
///DOEPCTL0 (rw) register accessor: an alias for `Reg<DOEPCTL0_SPEC>`
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
///This section describes the DOEPCTL0 register.
pub mod doepctl0;
///DOEPINT0 (rw) register accessor: an alias for `Reg<DOEPINT0_SPEC>`
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint0;
///DOEPTSIZ0 (rw) register accessor: an alias for `Reg<DOEPTSIZ0_SPEC>`
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
///The application must modify this register before enabling endpoint 0.
pub mod doeptsiz0;
///DOEPDMA0 (rw) register accessor: an alias for `Reg<DOEPDMA0_SPEC>`
pub type DOEPDMA0 = crate::Reg<doepdma0::DOEPDMA0_SPEC>;
///OTG device OUT endpoint 0 DMA address register
pub mod doepdma0;
///DOEPCTL1 (rw) register accessor: an alias for `Reg<DOEPCTL1_SPEC>`
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl1;
///DOEPINT1 (rw) register accessor: an alias for `Reg<DOEPINT1_SPEC>`
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint1;
///DOEPTSIZ1 (rw) register accessor: an alias for `Reg<DOEPTSIZ1_SPEC>`
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz1;
///DOEPDMA1 (rw) register accessor: an alias for `Reg<DOEPDMA1_SPEC>`
pub type DOEPDMA1 = crate::Reg<doepdma1::DOEPDMA1_SPEC>;
///OTG device OUT endpoint 1 DMA address register
pub mod doepdma1;
///DOEPCTL2 (rw) register accessor: an alias for `Reg<DOEPCTL2_SPEC>`
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl2;
///DOEPINT2 (rw) register accessor: an alias for `Reg<DOEPINT2_SPEC>`
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint2;
///DOEPTSIZ2 (rw) register accessor: an alias for `Reg<DOEPTSIZ2_SPEC>`
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz2;
///DOEPDMA2 (rw) register accessor: an alias for `Reg<DOEPDMA2_SPEC>`
pub type DOEPDMA2 = crate::Reg<doepdma2::DOEPDMA2_SPEC>;
///OTG device OUT endpoint 2 DMA address register
pub mod doepdma2;
///DOEPCTL3 (rw) register accessor: an alias for `Reg<DOEPCTL3_SPEC>`
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl3;
///DOEPINT3 (rw) register accessor: an alias for `Reg<DOEPINT3_SPEC>`
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint3;
///DOEPTSIZ3 (rw) register accessor: an alias for `Reg<DOEPTSIZ3_SPEC>`
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz3;
///DOEPDMA3 (rw) register accessor: an alias for `Reg<DOEPDMA3_SPEC>`
pub type DOEPDMA3 = crate::Reg<doepdma3::DOEPDMA3_SPEC>;
///OTG device OUT endpoint 3 DMA address register
pub mod doepdma3;
///DOEPCTL4 (rw) register accessor: an alias for `Reg<DOEPCTL4_SPEC>`
pub type DOEPCTL4 = crate::Reg<doepctl4::DOEPCTL4_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl4;
///DOEPINT4 (rw) register accessor: an alias for `Reg<DOEPINT4_SPEC>`
pub type DOEPINT4 = crate::Reg<doepint4::DOEPINT4_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint4;
///DOEPTSIZ4 (rw) register accessor: an alias for `Reg<DOEPTSIZ4_SPEC>`
pub type DOEPTSIZ4 = crate::Reg<doeptsiz4::DOEPTSIZ4_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz4;
///DOEPDMA4 (rw) register accessor: an alias for `Reg<DOEPDMA4_SPEC>`
pub type DOEPDMA4 = crate::Reg<doepdma4::DOEPDMA4_SPEC>;
///OTG device OUT endpoint 4 DMA address register
pub mod doepdma4;
///DOEPCTL5 (rw) register accessor: an alias for `Reg<DOEPCTL5_SPEC>`
pub type DOEPCTL5 = crate::Reg<doepctl5::DOEPCTL5_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl5;
///DOEPINT5 (rw) register accessor: an alias for `Reg<DOEPINT5_SPEC>`
pub type DOEPINT5 = crate::Reg<doepint5::DOEPINT5_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint5;
///DOEPTSIZ5 (rw) register accessor: an alias for `Reg<DOEPTSIZ5_SPEC>`
pub type DOEPTSIZ5 = crate::Reg<doeptsiz5::DOEPTSIZ5_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz5;
///DOEPDMA5 (rw) register accessor: an alias for `Reg<DOEPDMA5_SPEC>`
pub type DOEPDMA5 = crate::Reg<doepdma5::DOEPDMA5_SPEC>;
///OTG device OUT endpoint 5 DMA address register
pub mod doepdma5;
///DOEPCTL6 (rw) register accessor: an alias for `Reg<DOEPCTL6_SPEC>`
pub type DOEPCTL6 = crate::Reg<doepctl6::DOEPCTL6_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl6;
///DOEPINT6 (rw) register accessor: an alias for `Reg<DOEPINT6_SPEC>`
pub type DOEPINT6 = crate::Reg<doepint6::DOEPINT6_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint6;
///DOEPTSIZ6 (rw) register accessor: an alias for `Reg<DOEPTSIZ6_SPEC>`
pub type DOEPTSIZ6 = crate::Reg<doeptsiz6::DOEPTSIZ6_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz6;
///DOEPDMA6 (rw) register accessor: an alias for `Reg<DOEPDMA6_SPEC>`
pub type DOEPDMA6 = crate::Reg<doepdma6::DOEPDMA6_SPEC>;
///OTG device OUT endpoint 6 DMA address register
pub mod doepdma6;
///DOEPCTL7 (rw) register accessor: an alias for `Reg<DOEPCTL7_SPEC>`
pub type DOEPCTL7 = crate::Reg<doepctl7::DOEPCTL7_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl7;
///DOEPINT7 (rw) register accessor: an alias for `Reg<DOEPINT7_SPEC>`
pub type DOEPINT7 = crate::Reg<doepint7::DOEPINT7_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint7;
///DOEPTSIZ7 (rw) register accessor: an alias for `Reg<DOEPTSIZ7_SPEC>`
pub type DOEPTSIZ7 = crate::Reg<doeptsiz7::DOEPTSIZ7_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz7;
///DOEPDMA7 (rw) register accessor: an alias for `Reg<DOEPDMA7_SPEC>`
pub type DOEPDMA7 = crate::Reg<doepdma7::DOEPDMA7_SPEC>;
///OTG device OUT endpoint 7 DMA address register
pub mod doepdma7;
///DOEPCTL8 (rw) register accessor: an alias for `Reg<DOEPCTL8_SPEC>`
pub type DOEPCTL8 = crate::Reg<doepctl8::DOEPCTL8_SPEC>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl8;
///DOEPINT8 (rw) register accessor: an alias for `Reg<DOEPINT8_SPEC>`
pub type DOEPINT8 = crate::Reg<doepint8::DOEPINT8_SPEC>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint8;
///DOEPTSIZ8 (rw) register accessor: an alias for `Reg<DOEPTSIZ8_SPEC>`
pub type DOEPTSIZ8 = crate::Reg<doeptsiz8::DOEPTSIZ8_SPEC>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz8;
///DOEPDMA8 (rw) register accessor: an alias for `Reg<DOEPDMA8_SPEC>`
pub type DOEPDMA8 = crate::Reg<doepdma8::DOEPDMA8_SPEC>;
///OTG device OUT endpoint 8 DMA address register
pub mod doepdma8;
///PCGCCTL (rw) register accessor: an alias for `Reg<PCGCCTL_SPEC>`
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
///This register is available in host and device modes.
pub mod pcgcctl;
