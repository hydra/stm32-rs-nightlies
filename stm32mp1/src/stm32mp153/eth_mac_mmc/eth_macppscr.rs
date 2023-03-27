///Register `ETH_MACPPSCR` reader
pub struct R(crate::R<ETH_MACPPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPPSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_MACPPSCR` writer
pub struct W(crate::W<ETH_MACPPSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPPSCR_SPEC>;
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
impl From<crate::W<ETH_MACPPSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPPSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PPSCTRL` reader - PPSCTRL
pub type PPSCTRL_R = crate::FieldReader<u8, u8>;
///Field `PPSCTRL` writer - PPSCTRL
pub type PPSCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACPPSCR_SPEC, u8, u8, 4, O>;
///Field `PPSEN0` reader - PPSEN0
pub type PPSEN0_R = crate::BitReader<bool>;
///Field `PPSEN0` writer - PPSEN0
pub type PPSEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPPSCR_SPEC, bool, O>;
///Field `TRGTMODSEL0` reader - TRGTMODSEL0
pub type TRGTMODSEL0_R = crate::FieldReader<u8, u8>;
///Field `TRGTMODSEL0` writer - TRGTMODSEL0
pub type TRGTMODSEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACPPSCR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:3 - PPSCTRL
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - PPSEN0
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - TRGTMODSEL0
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    ///Bits 0:3 - PPSCTRL
    #[inline(always)]
    #[must_use]
    pub fn ppsctrl(&mut self) -> PPSCTRL_W<0> {
        PPSCTRL_W::new(self)
    }
    ///Bit 4 - PPSEN0
    #[inline(always)]
    #[must_use]
    pub fn ppsen0(&mut self) -> PPSEN0_W<4> {
        PPSEN0_W::new(self)
    }
    ///Bits 5:6 - TRGTMODSEL0
    #[inline(always)]
    #[must_use]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<5> {
        TRGTMODSEL0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\[30:24\]
///of this register are valid only when four Flexible PPS outputs are selected. Bits\[22:16\]
///are valid only when three or more Flexible PPS outputs are selected. Bits\[14:8\]
///are valid only when two or more Flexible PPS outputs are selected. Bits\[6:4\]
///are valid only when Flexible PPS feature is selected.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_macppscr](index.html) module
pub struct ETH_MACPPSCR_SPEC;
impl crate::RegisterSpec for ETH_MACPPSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_macppscr::R](R) reader structure
impl crate::Readable for ETH_MACPPSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_macppscr::W](W) writer structure
impl crate::Writable for ETH_MACPPSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_MACPPSCR to value 0
impl crate::Resettable for ETH_MACPPSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
