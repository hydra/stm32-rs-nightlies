///Register `MDF_SCD3CR` reader
pub struct R(crate::R<MDF_SCD3CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_SCD3CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_SCD3CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_SCD3CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_SCD3CR` writer
pub struct W(crate::W<MDF_SCD3CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_SCD3CR_SPEC>;
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
impl From<crate::W<MDF_SCD3CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_SCD3CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SCDEN` reader - Short circuit detector enable Set and cleared by software. - 0: The short circuit detector is disabled, - 1: The short circuit detector is enabled,
pub type SCDEN_R = crate::BitReader<bool>;
///Field `SCDEN` writer - Short circuit detector enable Set and cleared by software. - 0: The short circuit detector is disabled, - 1: The short circuit detector is enabled,
pub type SCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDF_SCD3CR_SPEC, bool, O>;
///Field `BKSCD` reader - Break signal assignment for short circuit detector Set and cleared by software. BKSCD\[i\]
///= 0: Break signal (mdf_break\[i\]) is not assigned to this SCD event BKSCD\[i\]
///= 1: Break signal (mdf_break\[i\]) is assigned to this SCD event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type BKSCD_R = crate::FieldReader<u8, u8>;
///Field `BKSCD` writer - Break signal assignment for short circuit detector Set and cleared by software. BKSCD\[i\]
///= 0: Break signal (mdf_break\[i\]) is not assigned to this SCD event BKSCD\[i\]
///= 1: Break signal (mdf_break\[i\]) is assigned to this SCD event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type BKSCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_SCD3CR_SPEC, u8, u8, 4, O>;
///Field `SCDT` reader - Short-circuit detector threshold Set and cleared by software. These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given input stream. - 0: 2 consecutive 1 s or 0 s will generate an event, - 1: 2 consecutive 1 s or 0 s will generate an event - 2: 3 consecutive 1 s or 0 s will generate an event, ... - 255: 256 consecutive 1 s or 0 s will generate an event, This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SCDT_R = crate::FieldReader<u8, u8>;
///Field `SCDT` writer - Short-circuit detector threshold Set and cleared by software. These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given input stream. - 0: 2 consecutive 1 s or 0 s will generate an event, - 1: 2 consecutive 1 s or 0 s will generate an event - 2: 3 consecutive 1 s or 0 s will generate an event, ... - 255: 256 consecutive 1 s or 0 s will generate an event, This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SCDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_SCD3CR_SPEC, u8, u8, 8, O>;
///Field `SCDACTIVE` reader - SCD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the SCD is effectively enabled (active) or not. The protected fields of this function can only be updated when the SCDACTIVE is set to a , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SCDEN and a transition on SCDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The SCD is not active, and can be configured if needed - 1: The SCD is active, and protected fields cannot be configured.
pub type SCDACTIVE_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Short circuit detector enable Set and cleared by software. - 0: The short circuit detector is disabled, - 1: The short circuit detector is enabled,
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - Break signal assignment for short circuit detector Set and cleared by software. BKSCD\[i\]
    ///= 0: Break signal (mdf_break\[i\]) is not assigned to this SCD event BKSCD\[i\]
    ///= 1: Break signal (mdf_break\[i\]) is assigned to this SCD event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 12:19 - Short-circuit detector threshold Set and cleared by software. These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given input stream. - 0: 2 consecutive 1 s or 0 s will generate an event, - 1: 2 consecutive 1 s or 0 s will generate an event - 2: 3 consecutive 1 s or 0 s will generate an event, ... - 255: 256 consecutive 1 s or 0 s will generate an event, This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    ///Bit 31 - SCD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the SCD is effectively enabled (active) or not. The protected fields of this function can only be updated when the SCDACTIVE is set to a , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SCDEN and a transition on SCDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The SCD is not active, and can be configured if needed - 1: The SCD is active, and protected fields cannot be configured.
    #[inline(always)]
    pub fn scdactive(&self) -> SCDACTIVE_R {
        SCDACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Short circuit detector enable Set and cleared by software. - 0: The short circuit detector is disabled, - 1: The short circuit detector is enabled,
    #[inline(always)]
    #[must_use]
    pub fn scden(&mut self) -> SCDEN_W<0> {
        SCDEN_W::new(self)
    }
    ///Bits 4:7 - Break signal assignment for short circuit detector Set and cleared by software. BKSCD\[i\]
    ///= 0: Break signal (mdf_break\[i\]) is not assigned to this SCD event BKSCD\[i\]
    ///= 1: Break signal (mdf_break\[i\]) is assigned to this SCD event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn bkscd(&mut self) -> BKSCD_W<4> {
        BKSCD_W::new(self)
    }
    ///Bits 12:19 - Short-circuit detector threshold Set and cleared by software. These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given input stream. - 0: 2 consecutive 1 s or 0 s will generate an event, - 1: 2 consecutive 1 s or 0 s will generate an event - 2: 3 consecutive 1 s or 0 s will generate an event, ... - 255: 256 consecutive 1 s or 0 s will generate an event, This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn scdt(&mut self) -> SCDT_W<12> {
        SCDT_W::new(self)
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
///For information about available fields see [mdf_scd3cr](index.html) module
pub struct MDF_SCD3CR_SPEC;
impl crate::RegisterSpec for MDF_SCD3CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_scd3cr::R](R) reader structure
impl crate::Readable for MDF_SCD3CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_scd3cr::W](W) writer structure
impl crate::Writable for MDF_SCD3CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_SCD3CR to value 0
impl crate::Resettable for MDF_SCD3CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
