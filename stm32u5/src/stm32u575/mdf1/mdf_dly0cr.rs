///Register `MDF_DLY0CR` reader
pub struct R(crate::R<MDF_DLY0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_DLY0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_DLY0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_DLY0CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_DLY0CR` writer
pub struct W(crate::W<MDF_DLY0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_DLY0CR_SPEC>;
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
impl From<crate::W<MDF_DLY0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_DLY0CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SKPDLY` reader - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,
pub type SKPDLY_R = crate::FieldReader<u8, u8>;
///Field `SKPDLY` writer - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,
pub type SKPDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_DLY0CR_SPEC, u8, u8, 7, O>;
///Field `SKPBF` reader - Skip Busy flag Set and cleared by hardware. Shall be used in order to control if the delay sequence is completed. - 0: Reading 0 means that the MDF is ready to accept a new value into SKPDLY\[6:0\]. - 1: Reading 1 means that last valid SKPDLY\[6:0\]
///is still under precessing.
pub type SKPBF_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:6 - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,
    #[inline(always)]
    pub fn skpdly(&self) -> SKPDLY_R {
        SKPDLY_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 31 - Skip Busy flag Set and cleared by hardware. Shall be used in order to control if the delay sequence is completed. - 0: Reading 0 means that the MDF is ready to accept a new value into SKPDLY\[6:0\]. - 1: Reading 1 means that last valid SKPDLY\[6:0\]
    ///is still under precessing.
    #[inline(always)]
    pub fn skpbf(&self) -> SKPBF_R {
        SKPBF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:6 - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,
    #[inline(always)]
    #[must_use]
    pub fn skpdly(&mut self) -> SKPDLY_W<0> {
        SKPDLY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used for the adjustment stream delays.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_dly0cr](index.html) module
pub struct MDF_DLY0CR_SPEC;
impl crate::RegisterSpec for MDF_DLY0CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_dly0cr::R](R) reader structure
impl crate::Readable for MDF_DLY0CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_dly0cr::W](W) writer structure
impl crate::Writable for MDF_DLY0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_DLY0CR to value 0
impl crate::Resettable for MDF_DLY0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
