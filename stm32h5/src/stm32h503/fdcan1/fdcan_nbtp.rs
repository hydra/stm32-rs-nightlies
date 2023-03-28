///Register `FDCAN_NBTP` reader
pub struct R(crate::R<FDCAN_NBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_NBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_NBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_NBTP_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_NBTP` writer
pub struct W(crate::W<FDCAN_NBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_NBTP_SPEC>;
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
impl From<crate::W<FDCAN_NBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_NBTP_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NTSEG2` reader - Nominal time segment after sample point Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used.
pub type NTSEG2_R = crate::FieldReader<u8, u8>;
///Field `NTSEG2` writer - Nominal time segment after sample point Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used.
pub type NTSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_NBTP_SPEC, u8, u8, 7, O>;
///Field `NTSEG1` reader - Nominal time segment before sample point Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub type NTSEG1_R = crate::FieldReader<u8, u8>;
///Field `NTSEG1` writer - Nominal time segment before sample point Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub type NTSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_NBTP_SPEC, u8, u8, 8, O>;
///Field `NBRP` reader - Bit rate prescaler Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub type NBRP_R = crate::FieldReader<u16, u16>;
///Field `NBRP` writer - Bit rate prescaler Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub type NBRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_NBTP_SPEC, u16, u16, 9, O>;
///Field `NSJW` reader - Nominal (re)synchronization jump width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub type NSJW_R = crate::FieldReader<u8, u8>;
///Field `NSJW` writer - Nominal (re)synchronization jump width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub type NSJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_NBTP_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:6 - Nominal time segment after sample point Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used.
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:15 - Nominal time segment before sample point Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:24 - Bit rate prescaler Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bits 25:31 - Nominal (re)synchronization jump width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - Nominal time segment after sample point Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used.
    #[inline(always)]
    #[must_use]
    pub fn ntseg2(&mut self) -> NTSEG2_W<0> {
        NTSEG2_W::new(self)
    }
    ///Bits 8:15 - Nominal time segment before sample point Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> NTSEG1_W<8> {
        NTSEG1_W::new(self)
    }
    ///Bits 16:24 - Bit rate prescaler Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NBRP_W<16> {
        NBRP_W::new(self)
    }
    ///Bits 25:31 - Nominal (re)synchronization jump width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NSJW_W<25> {
        NSJW_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN nominal bit timing and prescaler register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_nbtp](index.html) module
pub struct FDCAN_NBTP_SPEC;
impl crate::RegisterSpec for FDCAN_NBTP_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_nbtp::R](R) reader structure
impl crate::Readable for FDCAN_NBTP_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_nbtp::W](W) writer structure
impl crate::Writable for FDCAN_NBTP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_NBTP to value 0x0600_0a03
impl crate::Resettable for FDCAN_NBTP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600_0a03;
}
