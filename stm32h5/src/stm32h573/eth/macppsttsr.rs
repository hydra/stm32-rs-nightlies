///Register `MACPPSTTSR` reader
pub struct R(crate::R<MACPPSTTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPPSTTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPPSTTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPPSTTSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACPPSTTSR` writer
pub struct W(crate::W<MACPPSTTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPPSTTSR_SPEC>;
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
impl From<crate::W<MACPPSTTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPPSTTSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSTRH0` reader - PPS Target Time Seconds Register This field stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on Target Time mode selected for the corresponding PPS output in the PPS control register (ETH_MACPPSCR).
pub type TSTRH0_R = crate::FieldReader<u32, u32>;
///Field `TSTRH0` writer - PPS Target Time Seconds Register This field stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on Target Time mode selected for the corresponding PPS output in the PPS control register (ETH_MACPPSCR).
pub type TSTRH0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACPPSTTSR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - PPS Target Time Seconds Register This field stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on Target Time mode selected for the corresponding PPS output in the PPS control register (ETH_MACPPSCR).
    #[inline(always)]
    pub fn tstrh0(&self) -> TSTRH0_R {
        TSTRH0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - PPS Target Time Seconds Register This field stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on Target Time mode selected for the corresponding PPS output in the PPS control register (ETH_MACPPSCR).
    #[inline(always)]
    #[must_use]
    pub fn tstrh0(&mut self) -> TSTRH0_W<0> {
        TSTRH0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PPS target time seconds register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macppsttsr](index.html) module
pub struct MACPPSTTSR_SPEC;
impl crate::RegisterSpec for MACPPSTTSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macppsttsr::R](R) reader structure
impl crate::Readable for MACPPSTTSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macppsttsr::W](W) writer structure
impl crate::Writable for MACPPSTTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACPPSTTSR to value 0
impl crate::Resettable for MACPPSTTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
