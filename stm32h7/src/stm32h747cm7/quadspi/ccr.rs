///Register `CCR` reader
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR` writer
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INSTRUCTION` reader - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0.
pub type INSTRUCTION_R = crate::FieldReader<u8, u8>;
///Field `INSTRUCTION` writer - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0.
pub type INSTRUCTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 8, O>;
///Field `IMODE` reader - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0.
pub type IMODE_R = crate::FieldReader<u8, u8>;
///Field `IMODE` writer - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0.
pub type IMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
///Field `ADMODE` reader - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0.
pub type ADMODE_R = crate::FieldReader<u8, u8>;
///Field `ADMODE` writer - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0.
pub type ADMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
///Field `ADSIZE` reader - Address size This bit defines address size: This field can be written only when BUSY = 0.
pub type ADSIZE_R = crate::FieldReader<u8, u8>;
///Field `ADSIZE` writer - Address size This bit defines address size: This field can be written only when BUSY = 0.
pub type ADSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
///Field `ABMODE` reader - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0.
pub type ABMODE_R = crate::FieldReader<u8, u8>;
///Field `ABMODE` writer - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0.
pub type ABMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
///Field `ABSIZE` reader - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0.
pub type ABSIZE_R = crate::FieldReader<u8, u8>;
///Field `ABSIZE` writer - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0.
pub type ABSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
///Field `DCYC` reader - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0.
pub type DCYC_R = crate::FieldReader<u8, u8>;
///Field `DCYC` writer - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0.
pub type DCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 5, O>;
///Field `DMODE` reader - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0.
pub type DMODE_R = crate::FieldReader<u8, u8>;
///Field `DMODE` writer - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0.
pub type DMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
///Field `FMODE` reader - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0.
pub type FMODE_R = crate::FieldReader<u8, u8>;
///Field `FMODE` writer - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0.
pub type FMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
///Field `SIOO` reader - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0.
pub type SIOO_R = crate::BitReader<bool>;
///Field `SIOO` writer - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0.
pub type SIOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
///Field `DHHC` reader - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0.
pub type DHHC_R = crate::BitReader<bool>;
///Field `DHHC` writer - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0.
pub type DHHC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
///Field `DDRM` reader - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0.
pub type DDRM_R = crate::BitReader<bool>;
///Field `DDRM` writer - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0.
pub type DDRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Address size This bit defines address size: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:22 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:25 - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bit 28 - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn sioo(&self) -> SIOO_R {
        SIOO_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn dhhc(&self) -> DHHC_R {
        DHHC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn ddrm(&self) -> DDRM_R {
        DDRM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> INSTRUCTION_W<0> {
        INSTRUCTION_W::new(self)
    }
    ///Bits 8:9 - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn imode(&mut self) -> IMODE_W<8> {
        IMODE_W::new(self)
    }
    ///Bits 10:11 - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn admode(&mut self) -> ADMODE_W<10> {
        ADMODE_W::new(self)
    }
    ///Bits 12:13 - Address size This bit defines address size: This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn adsize(&mut self) -> ADSIZE_W<12> {
        ADSIZE_W::new(self)
    }
    ///Bits 14:15 - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn abmode(&mut self) -> ABMODE_W<14> {
        ABMODE_W::new(self)
    }
    ///Bits 16:17 - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn absize(&mut self) -> ABSIZE_W<16> {
        ABSIZE_W::new(self)
    }
    ///Bits 18:22 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<18> {
        DCYC_W::new(self)
    }
    ///Bits 24:25 - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<24> {
        DMODE_W::new(self)
    }
    ///Bits 26:27 - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn fmode(&mut self) -> FMODE_W<26> {
        FMODE_W::new(self)
    }
    ///Bit 28 - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn sioo(&mut self) -> SIOO_W<28> {
        SIOO_W::new(self)
    }
    ///Bit 30 - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn dhhc(&mut self) -> DHHC_W<30> {
        DHHC_W::new(self)
    }
    ///Bit 31 - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0.
    #[inline(always)]
    #[must_use]
    pub fn ddrm(&mut self) -> DDRM_W<31> {
        DDRM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///QUADSPI communication configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](index.html) module
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr::R](R) reader structure
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr::W](W) writer structure
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
