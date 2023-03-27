///Register `CSELR` reader
pub struct R(crate::R<CSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSELR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSELR` writer
pub struct W(crate::W<CSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSELR_SPEC>;
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
impl From<crate::W<CSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSELR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `C1S` reader - DMA channel 1 selection
pub type C1S_R = crate::FieldReader<u8, C1S_A>;
///DMA channel 1 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1S_A {
    ///0: Default mapping
    NoMapping = 0,
    ///1: Mapping 1
    Map1 = 1,
    ///2: Mapping 2
    Map2 = 2,
    ///3: Mapping 3
    Map3 = 3,
    ///4: Mapping 4
    Map4 = 4,
    ///5: Mapping 5
    Map5 = 5,
    ///6: Mapping 6
    Map6 = 6,
    ///7: Mapping 7
    Map7 = 7,
    ///8: Mapping 8
    Map8 = 8,
    ///9: Mapping 9
    Map9 = 9,
    ///10: Mapping 10
    Map10 = 10,
    ///11: Mapping 11
    Map11 = 11,
    ///12: Mapping 12
    Map12 = 12,
    ///13: Mapping 13
    Map13 = 13,
    ///14: Mapping 14
    Map14 = 14,
    ///15: Mapping 15
    Map15 = 15,
}
impl From<C1S_A> for u8 {
    #[inline(always)]
    fn from(variant: C1S_A) -> Self {
        variant as _
    }
}
impl C1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> C1S_A {
        match self.bits {
            0 => C1S_A::NoMapping,
            1 => C1S_A::Map1,
            2 => C1S_A::Map2,
            3 => C1S_A::Map3,
            4 => C1S_A::Map4,
            5 => C1S_A::Map5,
            6 => C1S_A::Map6,
            7 => C1S_A::Map7,
            8 => C1S_A::Map8,
            9 => C1S_A::Map9,
            10 => C1S_A::Map10,
            11 => C1S_A::Map11,
            12 => C1S_A::Map12,
            13 => C1S_A::Map13,
            14 => C1S_A::Map14,
            15 => C1S_A::Map15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoMapping`
    #[inline(always)]
    pub fn is_no_mapping(&self) -> bool {
        *self == C1S_A::NoMapping
    }
    ///Checks if the value of the field is `Map1`
    #[inline(always)]
    pub fn is_map1(&self) -> bool {
        *self == C1S_A::Map1
    }
    ///Checks if the value of the field is `Map2`
    #[inline(always)]
    pub fn is_map2(&self) -> bool {
        *self == C1S_A::Map2
    }
    ///Checks if the value of the field is `Map3`
    #[inline(always)]
    pub fn is_map3(&self) -> bool {
        *self == C1S_A::Map3
    }
    ///Checks if the value of the field is `Map4`
    #[inline(always)]
    pub fn is_map4(&self) -> bool {
        *self == C1S_A::Map4
    }
    ///Checks if the value of the field is `Map5`
    #[inline(always)]
    pub fn is_map5(&self) -> bool {
        *self == C1S_A::Map5
    }
    ///Checks if the value of the field is `Map6`
    #[inline(always)]
    pub fn is_map6(&self) -> bool {
        *self == C1S_A::Map6
    }
    ///Checks if the value of the field is `Map7`
    #[inline(always)]
    pub fn is_map7(&self) -> bool {
        *self == C1S_A::Map7
    }
    ///Checks if the value of the field is `Map8`
    #[inline(always)]
    pub fn is_map8(&self) -> bool {
        *self == C1S_A::Map8
    }
    ///Checks if the value of the field is `Map9`
    #[inline(always)]
    pub fn is_map9(&self) -> bool {
        *self == C1S_A::Map9
    }
    ///Checks if the value of the field is `Map10`
    #[inline(always)]
    pub fn is_map10(&self) -> bool {
        *self == C1S_A::Map10
    }
    ///Checks if the value of the field is `Map11`
    #[inline(always)]
    pub fn is_map11(&self) -> bool {
        *self == C1S_A::Map11
    }
    ///Checks if the value of the field is `Map12`
    #[inline(always)]
    pub fn is_map12(&self) -> bool {
        *self == C1S_A::Map12
    }
    ///Checks if the value of the field is `Map13`
    #[inline(always)]
    pub fn is_map13(&self) -> bool {
        *self == C1S_A::Map13
    }
    ///Checks if the value of the field is `Map14`
    #[inline(always)]
    pub fn is_map14(&self) -> bool {
        *self == C1S_A::Map14
    }
    ///Checks if the value of the field is `Map15`
    #[inline(always)]
    pub fn is_map15(&self) -> bool {
        *self == C1S_A::Map15
    }
}
///Field `C1S` writer - DMA channel 1 selection
pub type C1S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSELR_SPEC, u8, C1S_A, 4, O>;
impl<'a, const O: u8> C1S_W<'a, O> {
    ///Default mapping
    #[inline(always)]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C1S_A::NoMapping)
    }
    ///Mapping 1
    #[inline(always)]
    pub fn map1(self) -> &'a mut W {
        self.variant(C1S_A::Map1)
    }
    ///Mapping 2
    #[inline(always)]
    pub fn map2(self) -> &'a mut W {
        self.variant(C1S_A::Map2)
    }
    ///Mapping 3
    #[inline(always)]
    pub fn map3(self) -> &'a mut W {
        self.variant(C1S_A::Map3)
    }
    ///Mapping 4
    #[inline(always)]
    pub fn map4(self) -> &'a mut W {
        self.variant(C1S_A::Map4)
    }
    ///Mapping 5
    #[inline(always)]
    pub fn map5(self) -> &'a mut W {
        self.variant(C1S_A::Map5)
    }
    ///Mapping 6
    #[inline(always)]
    pub fn map6(self) -> &'a mut W {
        self.variant(C1S_A::Map6)
    }
    ///Mapping 7
    #[inline(always)]
    pub fn map7(self) -> &'a mut W {
        self.variant(C1S_A::Map7)
    }
    ///Mapping 8
    #[inline(always)]
    pub fn map8(self) -> &'a mut W {
        self.variant(C1S_A::Map8)
    }
    ///Mapping 9
    #[inline(always)]
    pub fn map9(self) -> &'a mut W {
        self.variant(C1S_A::Map9)
    }
    ///Mapping 10
    #[inline(always)]
    pub fn map10(self) -> &'a mut W {
        self.variant(C1S_A::Map10)
    }
    ///Mapping 11
    #[inline(always)]
    pub fn map11(self) -> &'a mut W {
        self.variant(C1S_A::Map11)
    }
    ///Mapping 12
    #[inline(always)]
    pub fn map12(self) -> &'a mut W {
        self.variant(C1S_A::Map12)
    }
    ///Mapping 13
    #[inline(always)]
    pub fn map13(self) -> &'a mut W {
        self.variant(C1S_A::Map13)
    }
    ///Mapping 14
    #[inline(always)]
    pub fn map14(self) -> &'a mut W {
        self.variant(C1S_A::Map14)
    }
    ///Mapping 15
    #[inline(always)]
    pub fn map15(self) -> &'a mut W {
        self.variant(C1S_A::Map15)
    }
}
///Field `C2S` reader - DMA channel 2 selection
pub use C1S_R as C2S_R;
///Field `C3S` reader - DMA channel 3 selection
pub use C1S_R as C3S_R;
///Field `C4S` reader - DMA channel 4 selection
pub use C1S_R as C4S_R;
///Field `C5S` reader - DMA channel 5 selection
pub use C1S_R as C5S_R;
///Field `C6S` reader - DMA channel 6 selection
pub use C1S_R as C6S_R;
///Field `C7S` reader - DMA channel 7 selection
pub use C1S_R as C7S_R;
///Field `C2S` writer - DMA channel 2 selection
pub use C1S_W as C2S_W;
///Field `C3S` writer - DMA channel 3 selection
pub use C1S_W as C3S_W;
///Field `C4S` writer - DMA channel 4 selection
pub use C1S_W as C4S_W;
///Field `C5S` writer - DMA channel 5 selection
pub use C1S_W as C5S_W;
///Field `C6S` writer - DMA channel 6 selection
pub use C1S_W as C6S_W;
///Field `C7S` writer - DMA channel 7 selection
pub use C1S_W as C7S_W;
impl R {
    ///Bits 0:3 - DMA channel 1 selection
    #[inline(always)]
    pub fn c1s(&self) -> C1S_R {
        C1S_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - DMA channel 2 selection
    #[inline(always)]
    pub fn c2s(&self) -> C2S_R {
        C2S_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - DMA channel 3 selection
    #[inline(always)]
    pub fn c3s(&self) -> C3S_R {
        C3S_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - DMA channel 4 selection
    #[inline(always)]
    pub fn c4s(&self) -> C4S_R {
        C4S_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - DMA channel 5 selection
    #[inline(always)]
    pub fn c5s(&self) -> C5S_R {
        C5S_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - DMA channel 6 selection
    #[inline(always)]
    pub fn c6s(&self) -> C6S_R {
        C6S_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - DMA channel 7 selection
    #[inline(always)]
    pub fn c7s(&self) -> C7S_R {
        C7S_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - DMA channel 1 selection
    #[inline(always)]
    #[must_use]
    pub fn c1s(&mut self) -> C1S_W<0> {
        C1S_W::new(self)
    }
    ///Bits 4:7 - DMA channel 2 selection
    #[inline(always)]
    #[must_use]
    pub fn c2s(&mut self) -> C2S_W<4> {
        C2S_W::new(self)
    }
    ///Bits 8:11 - DMA channel 3 selection
    #[inline(always)]
    #[must_use]
    pub fn c3s(&mut self) -> C3S_W<8> {
        C3S_W::new(self)
    }
    ///Bits 12:15 - DMA channel 4 selection
    #[inline(always)]
    #[must_use]
    pub fn c4s(&mut self) -> C4S_W<12> {
        C4S_W::new(self)
    }
    ///Bits 16:19 - DMA channel 5 selection
    #[inline(always)]
    #[must_use]
    pub fn c5s(&mut self) -> C5S_W<16> {
        C5S_W::new(self)
    }
    ///Bits 20:23 - DMA channel 6 selection
    #[inline(always)]
    #[must_use]
    pub fn c6s(&mut self) -> C6S_W<20> {
        C6S_W::new(self)
    }
    ///Bits 24:27 - DMA channel 7 selection
    #[inline(always)]
    #[must_use]
    pub fn c7s(&mut self) -> C7S_W<24> {
        C7S_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///channel selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cselr](index.html) module
pub struct CSELR_SPEC;
impl crate::RegisterSpec for CSELR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cselr::R](R) reader structure
impl crate::Readable for CSELR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cselr::W](W) writer structure
impl crate::Writable for CSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSELR to value 0
impl crate::Resettable for CSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
