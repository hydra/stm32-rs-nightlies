///Register `ETH_MACPPSTTNR` reader
pub struct R(crate::R<ETH_MACPPSTTNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPPSTTNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPPSTTNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPPSTTNR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_MACPPSTTNR` writer
pub struct W(crate::W<ETH_MACPPSTTNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPPSTTNR_SPEC>;
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
impl From<crate::W<ETH_MACPPSTTNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPPSTTNR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TTSL0` reader - TTSL0
pub type TTSL0_R = crate::FieldReader<u32, u32>;
///Field `TTSL0` writer - TTSL0
pub type TTSL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACPPSTTNR_SPEC, u32, u32, 31, O>;
///Field `TRGTBUSY0` reader - TRGTBUSY0
pub type TRGTBUSY0_R = crate::BitReader<bool>;
///Field `TRGTBUSY0` writer - TRGTBUSY0
pub type TRGTBUSY0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPPSTTNR_SPEC, bool, O>;
impl R {
    ///Bits 0:30 - TTSL0
    #[inline(always)]
    pub fn ttsl0(&self) -> TTSL0_R {
        TTSL0_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - TRGTBUSY0
    #[inline(always)]
    pub fn trgtbusy0(&self) -> TRGTBUSY0_R {
        TRGTBUSY0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:30 - TTSL0
    #[inline(always)]
    #[must_use]
    pub fn ttsl0(&mut self) -> TTSL0_W<0> {
        TTSL0_W::new(self)
    }
    ///Bit 31 - TRGTBUSY0
    #[inline(always)]
    #[must_use]
    pub fn trgtbusy0(&mut self) -> TRGTBUSY0_W<31> {
        TRGTBUSY0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_macppsttnr](index.html) module
pub struct ETH_MACPPSTTNR_SPEC;
impl crate::RegisterSpec for ETH_MACPPSTTNR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_macppsttnr::R](R) reader structure
impl crate::Readable for ETH_MACPPSTTNR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_macppsttnr::W](W) writer structure
impl crate::Writable for ETH_MACPPSTTNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_MACPPSTTNR to value 0
impl crate::Resettable for ETH_MACPPSTTNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
