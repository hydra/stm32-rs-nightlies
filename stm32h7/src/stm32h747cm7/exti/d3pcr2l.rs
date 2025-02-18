///Register `D3PCR2L` reader
pub struct R(crate::R<D3PCR2L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PCR2L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PCR2L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PCR2L_SPEC>) -> Self {
        R(reader)
    }
}
///Register `D3PCR2L` writer
pub struct W(crate::W<D3PCR2L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PCR2L_SPEC>;
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
impl From<crate::W<D3PCR2L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PCR2L_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCS34` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub type PCS34_R = crate::FieldReader<u8, PCS34_A>;
///D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS34_A {
    ///0: DMA ch6 event selected as D3 domain pendclear source
    DmaCh6 = 0,
    ///1: DMA ch7 event selected as D3 domain pendclear source
    DmaCh7 = 1,
    ///2: LPTIM4 out selected as D3 domain pendclear source
    Lptim4 = 2,
    ///3: LPTIM5 out selected as D3 domain pendclear source
    Lptim5 = 3,
}
impl From<PCS34_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS34_A) -> Self {
        variant as _
    }
}
impl PCS34_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PCS34_A {
        match self.bits {
            0 => PCS34_A::DmaCh6,
            1 => PCS34_A::DmaCh7,
            2 => PCS34_A::Lptim4,
            3 => PCS34_A::Lptim5,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DmaCh6`
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS34_A::DmaCh6
    }
    ///Checks if the value of the field is `DmaCh7`
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS34_A::DmaCh7
    }
    ///Checks if the value of the field is `Lptim4`
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS34_A::Lptim4
    }
    ///Checks if the value of the field is `Lptim5`
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS34_A::Lptim5
    }
}
///Field `PCS34` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub type PCS34_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, D3PCR2L_SPEC, u8, PCS34_A, 2, O>;
impl<'a, const O: u8> PCS34_W<'a, O> {
    ///DMA ch6 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS34_A::DmaCh6)
    }
    ///DMA ch7 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS34_A::DmaCh7)
    }
    ///LPTIM4 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS34_A::Lptim4)
    }
    ///LPTIM5 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS34_A::Lptim5)
    }
}
///Field `PCS35` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub use PCS34_R as PCS35_R;
///Field `PCS41` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub use PCS34_R as PCS41_R;
///Field `PCS35` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub use PCS34_W as PCS35_W;
///Field `PCS41` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub use PCS34_W as PCS41_W;
impl R {
    ///Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    pub fn pcs34(&self) -> PCS34_R {
        PCS34_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    pub fn pcs35(&self) -> PCS35_R {
        PCS35_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    pub fn pcs41(&self) -> PCS41_R {
        PCS41_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    ///Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    #[must_use]
    pub fn pcs34(&mut self) -> PCS34_W<4> {
        PCS34_W::new(self)
    }
    ///Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    #[must_use]
    pub fn pcs35(&mut self) -> PCS35_W<6> {
        PCS35_W::new(self)
    }
    ///Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    #[must_use]
    pub fn pcs41(&mut self) -> PCS41_W<18> {
        PCS41_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI D3 pending clear selection register low
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [d3pcr2l](index.html) module
pub struct D3PCR2L_SPEC;
impl crate::RegisterSpec for D3PCR2L_SPEC {
    type Ux = u32;
}
///`read()` method returns [d3pcr2l::R](R) reader structure
impl crate::Readable for D3PCR2L_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [d3pcr2l::W](W) writer structure
impl crate::Writable for D3PCR2L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets D3PCR2L to value 0
impl crate::Resettable for D3PCR2L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
