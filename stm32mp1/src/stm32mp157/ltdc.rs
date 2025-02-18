///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - LTDC identification register
    pub ltdc_idr: LTDC_IDR,
    ///0x04 - LDTC layer count register
    pub ltdc_lcr: LTDC_LCR,
    ///0x08 - This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    pub ltdc_sscr: LTDC_SSCR,
    ///0x0c - This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    pub ltdc_bpcr: LTDC_BPCR,
    ///0x10 - This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    pub ltdc_awcr: LTDC_AWCR,
    ///0x14 - This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    pub ltdc_twcr: LTDC_TWCR,
    ///0x18 - This register defines the global configuration of the LCD-TFT controller.
    pub ltdc_gcr: LTDC_GCR,
    ///0x1c - LTDC global configuration 1 register
    pub ltdc_gc1r: LTDC_GC1R,
    ///0x20 - LTDC global configuration 2 register
    pub ltdc_gc2r: LTDC_GC2R,
    ///0x24 - This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.
    pub ltdc_srcr: LTDC_SRCR,
    _reserved10: [u8; 0x04],
    ///0x2c - This register defines the background color (RGB888).
    pub ltdc_bccr: LTDC_BCCR,
    _reserved11: [u8; 0x04],
    ///0x34 - This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
    pub ltdc_ier: LTDC_IER,
    ///0x38 - This register returns the interrupt status flag.
    pub ltdc_isr: LTDC_ISR,
    ///0x3c - LTDC Interrupt Clear Register
    pub ltdc_icr: LTDC_ICR,
    ///0x40 - This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274.
    pub ltdc_lipcr: LTDC_LIPCR,
    ///0x44 - LTDC current position status register
    pub ltdc_cpsr: LTDC_CPSR,
    ///0x48 - This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.
    pub ltdc_cdsr: LTDC_CDSR,
    _reserved17: [u8; 0x38],
    ///0x84 - LTDC layer 1 control register
    pub ltdc_l1cr: LTDC_L1CR,
    ///0x88 - This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\]
    ///bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\]
    ///bits in the LTDC_AWCR register.
    pub ltdc_l1whpcr: LTDC_L1WHPCR,
    ///0x8c - This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\]
    ///bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\]
    ///bits in the LTDC_AWCR register.
    pub ltdc_l1wvpcr: LTDC_L1WVPCR,
    ///0x90 - This register defines the color key value (RGB), that is used by the color keying.
    pub ltdc_l1ckcr: LTDC_L1CKCR,
    ///0x94 - This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
    pub ltdc_l1pfcr: LTDC_L1PFCR,
    ///0x98 - This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
    pub ltdc_l1cacr: LTDC_L1CACR,
    ///0x9c - This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
    pub ltdc_l1dccr: LTDC_L1DCCR,
    ///0xa0 - This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
    pub ltdc_l1bfcr: LTDC_L1BFCR,
    _reserved25: [u8; 0x08],
    ///0xac - This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
    pub ltdc_l1cfbar: LTDC_L1CFBAR,
    ///0xb0 - This register defines the color frame buffer line length and pitch.
    pub ltdc_l1cfblr: LTDC_L1CFBLR,
    ///0xb4 - This register defines the number of lines in the color frame buffer.
    pub ltdc_l1cfblnr: LTDC_L1CFBLNR,
    _reserved28: [u8; 0x0c],
    ///0xc4 - This register defines the CLUT address and the RGB value.
    pub ltdc_l1clutwr: LTDC_L1CLUTWR,
    _reserved29: [u8; 0x3c],
    ///0x104 - LTDC layer 2 control register
    pub ltdc_l2cr: LTDC_L2CR,
    ///0x108 - This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\]
    ///bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\]
    ///bits in the LTDC_AWCR register.
    pub ltdc_l2whpcr: LTDC_L2WHPCR,
    ///0x10c - This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\]
    ///bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\]
    ///bits in the LTDC_AWCR register.
    pub ltdc_l2wvpcr: LTDC_L2WVPCR,
    ///0x110 - This register defines the color key value (RGB), that is used by the color keying.
    pub ltdc_l2ckcr: LTDC_L2CKCR,
    ///0x114 - This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
    pub ltdc_l2pfcr: LTDC_L2PFCR,
    ///0x118 - This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
    pub ltdc_l2cacr: LTDC_L2CACR,
    ///0x11c - This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
    pub ltdc_l2dccr: LTDC_L2DCCR,
    ///0x120 - This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
    pub ltdc_l2bfcr: LTDC_L2BFCR,
    _reserved37: [u8; 0x08],
    ///0x12c - This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
    pub ltdc_l2cfbar: LTDC_L2CFBAR,
    ///0x130 - This register defines the color frame buffer line length and pitch.
    pub ltdc_l2cfblr: LTDC_L2CFBLR,
    ///0x134 - This register defines the number of lines in the color frame buffer.
    pub ltdc_l2cfblnr: LTDC_L2CFBLNR,
    _reserved40: [u8; 0x0c],
    ///0x144 - This register defines the CLUT address and the RGB value.
    pub ltdc_l2clutwr: LTDC_L2CLUTWR,
}
///LTDC_IDR (r) register accessor: an alias for `Reg<LTDC_IDR_SPEC>`
pub type LTDC_IDR = crate::Reg<ltdc_idr::LTDC_IDR_SPEC>;
///LTDC identification register
pub mod ltdc_idr;
///LTDC_LCR (r) register accessor: an alias for `Reg<LTDC_LCR_SPEC>`
pub type LTDC_LCR = crate::Reg<ltdc_lcr::LTDC_LCR_SPEC>;
///LDTC layer count register
pub mod ltdc_lcr;
///LTDC_SSCR (rw) register accessor: an alias for `Reg<LTDC_SSCR_SPEC>`
pub type LTDC_SSCR = crate::Reg<ltdc_sscr::LTDC_SSCR_SPEC>;
///This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod ltdc_sscr;
///LTDC_BPCR (rw) register accessor: an alias for `Reg<LTDC_BPCR_SPEC>`
pub type LTDC_BPCR = crate::Reg<ltdc_bpcr::LTDC_BPCR_SPEC>;
///This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod ltdc_bpcr;
///LTDC_AWCR (rw) register accessor: an alias for `Reg<LTDC_AWCR_SPEC>`
pub type LTDC_AWCR = crate::Reg<ltdc_awcr::LTDC_AWCR_SPEC>;
///This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod ltdc_awcr;
///LTDC_TWCR (rw) register accessor: an alias for `Reg<LTDC_TWCR_SPEC>`
pub type LTDC_TWCR = crate::Reg<ltdc_twcr::LTDC_TWCR_SPEC>;
///This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod ltdc_twcr;
///LTDC_GCR (rw) register accessor: an alias for `Reg<LTDC_GCR_SPEC>`
pub type LTDC_GCR = crate::Reg<ltdc_gcr::LTDC_GCR_SPEC>;
///This register defines the global configuration of the LCD-TFT controller.
pub mod ltdc_gcr;
///LTDC_GC1R (r) register accessor: an alias for `Reg<LTDC_GC1R_SPEC>`
pub type LTDC_GC1R = crate::Reg<ltdc_gc1r::LTDC_GC1R_SPEC>;
///LTDC global configuration 1 register
pub mod ltdc_gc1r;
///LTDC_GC2R (r) register accessor: an alias for `Reg<LTDC_GC2R_SPEC>`
pub type LTDC_GC2R = crate::Reg<ltdc_gc2r::LTDC_GC2R_SPEC>;
///LTDC global configuration 2 register
pub mod ltdc_gc2r;
///LTDC_SRCR (rw) register accessor: an alias for `Reg<LTDC_SRCR_SPEC>`
pub type LTDC_SRCR = crate::Reg<ltdc_srcr::LTDC_SRCR_SPEC>;
///This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.
pub mod ltdc_srcr;
///LTDC_BCCR (rw) register accessor: an alias for `Reg<LTDC_BCCR_SPEC>`
pub type LTDC_BCCR = crate::Reg<ltdc_bccr::LTDC_BCCR_SPEC>;
///This register defines the background color (RGB888).
pub mod ltdc_bccr;
///LTDC_IER (rw) register accessor: an alias for `Reg<LTDC_IER_SPEC>`
pub type LTDC_IER = crate::Reg<ltdc_ier::LTDC_IER_SPEC>;
///This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
pub mod ltdc_ier;
///LTDC_ISR (r) register accessor: an alias for `Reg<LTDC_ISR_SPEC>`
pub type LTDC_ISR = crate::Reg<ltdc_isr::LTDC_ISR_SPEC>;
///This register returns the interrupt status flag.
pub mod ltdc_isr;
///LTDC_ICR (w) register accessor: an alias for `Reg<LTDC_ICR_SPEC>`
pub type LTDC_ICR = crate::Reg<ltdc_icr::LTDC_ICR_SPEC>;
///LTDC Interrupt Clear Register
pub mod ltdc_icr;
///LTDC_LIPCR (rw) register accessor: an alias for `Reg<LTDC_LIPCR_SPEC>`
pub type LTDC_LIPCR = crate::Reg<ltdc_lipcr::LTDC_LIPCR_SPEC>;
///This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274.
pub mod ltdc_lipcr;
///LTDC_CPSR (r) register accessor: an alias for `Reg<LTDC_CPSR_SPEC>`
pub type LTDC_CPSR = crate::Reg<ltdc_cpsr::LTDC_CPSR_SPEC>;
///LTDC current position status register
pub mod ltdc_cpsr;
///LTDC_CDSR (r) register accessor: an alias for `Reg<LTDC_CDSR_SPEC>`
pub type LTDC_CDSR = crate::Reg<ltdc_cdsr::LTDC_CDSR_SPEC>;
///This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.
pub mod ltdc_cdsr;
///LTDC_L1CR (rw) register accessor: an alias for `Reg<LTDC_L1CR_SPEC>`
pub type LTDC_L1CR = crate::Reg<ltdc_l1cr::LTDC_L1CR_SPEC>;
///LTDC layer 1 control register
pub mod ltdc_l1cr;
///LTDC_L1WHPCR (rw) register accessor: an alias for `Reg<LTDC_L1WHPCR_SPEC>`
pub type LTDC_L1WHPCR = crate::Reg<ltdc_l1whpcr::LTDC_L1WHPCR_SPEC>;
///This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\]
///bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\]
///bits in the LTDC_AWCR register.
pub mod ltdc_l1whpcr;
///LTDC_L1WVPCR (rw) register accessor: an alias for `Reg<LTDC_L1WVPCR_SPEC>`
pub type LTDC_L1WVPCR = crate::Reg<ltdc_l1wvpcr::LTDC_L1WVPCR_SPEC>;
///This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\]
///bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\]
///bits in the LTDC_AWCR register.
pub mod ltdc_l1wvpcr;
///LTDC_L1CKCR (rw) register accessor: an alias for `Reg<LTDC_L1CKCR_SPEC>`
pub type LTDC_L1CKCR = crate::Reg<ltdc_l1ckcr::LTDC_L1CKCR_SPEC>;
///This register defines the color key value (RGB), that is used by the color keying.
pub mod ltdc_l1ckcr;
///LTDC_L1PFCR (rw) register accessor: an alias for `Reg<LTDC_L1PFCR_SPEC>`
pub type LTDC_L1PFCR = crate::Reg<ltdc_l1pfcr::LTDC_L1PFCR_SPEC>;
///This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
pub mod ltdc_l1pfcr;
///LTDC_L1CACR (rw) register accessor: an alias for `Reg<LTDC_L1CACR_SPEC>`
pub type LTDC_L1CACR = crate::Reg<ltdc_l1cacr::LTDC_L1CACR_SPEC>;
///This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
pub mod ltdc_l1cacr;
///LTDC_L1DCCR (rw) register accessor: an alias for `Reg<LTDC_L1DCCR_SPEC>`
pub type LTDC_L1DCCR = crate::Reg<ltdc_l1dccr::LTDC_L1DCCR_SPEC>;
///This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
pub mod ltdc_l1dccr;
///LTDC_L1BFCR (rw) register accessor: an alias for `Reg<LTDC_L1BFCR_SPEC>`
pub type LTDC_L1BFCR = crate::Reg<ltdc_l1bfcr::LTDC_L1BFCR_SPEC>;
///This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
pub mod ltdc_l1bfcr;
///LTDC_L1CFBAR (rw) register accessor: an alias for `Reg<LTDC_L1CFBAR_SPEC>`
pub type LTDC_L1CFBAR = crate::Reg<ltdc_l1cfbar::LTDC_L1CFBAR_SPEC>;
///This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
pub mod ltdc_l1cfbar;
///LTDC_L1CFBLR (rw) register accessor: an alias for `Reg<LTDC_L1CFBLR_SPEC>`
pub type LTDC_L1CFBLR = crate::Reg<ltdc_l1cfblr::LTDC_L1CFBLR_SPEC>;
///This register defines the color frame buffer line length and pitch.
pub mod ltdc_l1cfblr;
///LTDC_L1CFBLNR (rw) register accessor: an alias for `Reg<LTDC_L1CFBLNR_SPEC>`
pub type LTDC_L1CFBLNR = crate::Reg<ltdc_l1cfblnr::LTDC_L1CFBLNR_SPEC>;
///This register defines the number of lines in the color frame buffer.
pub mod ltdc_l1cfblnr;
///LTDC_L1CLUTWR (w) register accessor: an alias for `Reg<LTDC_L1CLUTWR_SPEC>`
pub type LTDC_L1CLUTWR = crate::Reg<ltdc_l1clutwr::LTDC_L1CLUTWR_SPEC>;
///This register defines the CLUT address and the RGB value.
pub mod ltdc_l1clutwr;
///LTDC_L2CR (rw) register accessor: an alias for `Reg<LTDC_L2CR_SPEC>`
pub type LTDC_L2CR = crate::Reg<ltdc_l2cr::LTDC_L2CR_SPEC>;
///LTDC layer 2 control register
pub mod ltdc_l2cr;
///LTDC_L2WHPCR (rw) register accessor: an alias for `Reg<LTDC_L2WHPCR_SPEC>`
pub type LTDC_L2WHPCR = crate::Reg<ltdc_l2whpcr::LTDC_L2WHPCR_SPEC>;
///This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\]
///bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\]
///bits in the LTDC_AWCR register.
pub mod ltdc_l2whpcr;
///LTDC_L2WVPCR (rw) register accessor: an alias for `Reg<LTDC_L2WVPCR_SPEC>`
pub type LTDC_L2WVPCR = crate::Reg<ltdc_l2wvpcr::LTDC_L2WVPCR_SPEC>;
///This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\]
///bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\]
///bits in the LTDC_AWCR register.
pub mod ltdc_l2wvpcr;
///LTDC_L2CKCR (rw) register accessor: an alias for `Reg<LTDC_L2CKCR_SPEC>`
pub type LTDC_L2CKCR = crate::Reg<ltdc_l2ckcr::LTDC_L2CKCR_SPEC>;
///This register defines the color key value (RGB), that is used by the color keying.
pub mod ltdc_l2ckcr;
///LTDC_L2PFCR (rw) register accessor: an alias for `Reg<LTDC_L2PFCR_SPEC>`
pub type LTDC_L2PFCR = crate::Reg<ltdc_l2pfcr::LTDC_L2PFCR_SPEC>;
///This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
pub mod ltdc_l2pfcr;
///LTDC_L2CACR (rw) register accessor: an alias for `Reg<LTDC_L2CACR_SPEC>`
pub type LTDC_L2CACR = crate::Reg<ltdc_l2cacr::LTDC_L2CACR_SPEC>;
///This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
pub mod ltdc_l2cacr;
///LTDC_L2DCCR (rw) register accessor: an alias for `Reg<LTDC_L2DCCR_SPEC>`
pub type LTDC_L2DCCR = crate::Reg<ltdc_l2dccr::LTDC_L2DCCR_SPEC>;
///This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
pub mod ltdc_l2dccr;
///LTDC_L2BFCR (rw) register accessor: an alias for `Reg<LTDC_L2BFCR_SPEC>`
pub type LTDC_L2BFCR = crate::Reg<ltdc_l2bfcr::LTDC_L2BFCR_SPEC>;
///This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
pub mod ltdc_l2bfcr;
///LTDC_L2CFBAR (rw) register accessor: an alias for `Reg<LTDC_L2CFBAR_SPEC>`
pub type LTDC_L2CFBAR = crate::Reg<ltdc_l2cfbar::LTDC_L2CFBAR_SPEC>;
///This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
pub mod ltdc_l2cfbar;
///LTDC_L2CFBLR (rw) register accessor: an alias for `Reg<LTDC_L2CFBLR_SPEC>`
pub type LTDC_L2CFBLR = crate::Reg<ltdc_l2cfblr::LTDC_L2CFBLR_SPEC>;
///This register defines the color frame buffer line length and pitch.
pub mod ltdc_l2cfblr;
///LTDC_L2CFBLNR (rw) register accessor: an alias for `Reg<LTDC_L2CFBLNR_SPEC>`
pub type LTDC_L2CFBLNR = crate::Reg<ltdc_l2cfblnr::LTDC_L2CFBLNR_SPEC>;
///This register defines the number of lines in the color frame buffer.
pub mod ltdc_l2cfblnr;
///LTDC_L2CLUTWR (w) register accessor: an alias for `Reg<LTDC_L2CLUTWR_SPEC>`
pub type LTDC_L2CLUTWR = crate::Reg<ltdc_l2clutwr::LTDC_L2CLUTWR_SPEC>;
///This register defines the CLUT address and the RGB value.
pub mod ltdc_l2clutwr;
