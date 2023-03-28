///Register `D1CFGR` reader
pub struct R(crate::R<D1CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D1CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D1CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D1CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `D1CFGR` writer
pub struct W(crate::W<D1CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D1CFGR_SPEC>;
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
impl From<crate::W<D1CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D1CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HPRE` reader - D1 domain AHB prescaler
pub type HPRE_R = crate::FieldReader<u8, HPRE_A>;
///D1 domain AHB prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE_A {
    ///0: sys_ck not divided
    Div1 = 0,
    ///8: sys_ck divided by 2
    Div2 = 8,
    ///9: sys_ck divided by 4
    Div4 = 9,
    ///10: sys_ck divided by 8
    Div8 = 10,
    ///11: sys_ck divided by 16
    Div16 = 11,
    ///12: sys_ck divided by 64
    Div64 = 12,
    ///13: sys_ck divided by 128
    Div128 = 13,
    ///14: sys_ck divided by 256
    Div256 = 14,
    ///15: sys_ck divided by 512
    Div512 = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
impl HPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            0 => Some(HPRE_A::Div1),
            8 => Some(HPRE_A::Div2),
            9 => Some(HPRE_A::Div4),
            10 => Some(HPRE_A::Div8),
            11 => Some(HPRE_A::Div16),
            12 => Some(HPRE_A::Div64),
            13 => Some(HPRE_A::Div128),
            14 => Some(HPRE_A::Div256),
            15 => Some(HPRE_A::Div512),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HPRE_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE_A::Div16
    }
    ///Checks if the value of the field is `Div64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE_A::Div64
    }
    ///Checks if the value of the field is `Div128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE_A::Div128
    }
    ///Checks if the value of the field is `Div256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE_A::Div256
    }
    ///Checks if the value of the field is `Div512`
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE_A::Div512
    }
}
///Field `HPRE` writer - D1 domain AHB prescaler
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, D1CFGR_SPEC, u8, HPRE_A, 4, O>;
impl<'a, const O: u8> HPRE_W<'a, O> {
    ///sys_ck not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::Div1)
    }
    ///sys_ck divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::Div2)
    }
    ///sys_ck divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::Div4)
    }
    ///sys_ck divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::Div8)
    }
    ///sys_ck divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::Div16)
    }
    ///sys_ck divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::Div64)
    }
    ///sys_ck divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::Div128)
    }
    ///sys_ck divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::Div256)
    }
    ///sys_ck divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::Div512)
    }
}
///Field `D1PPRE` reader - D1 domain APB3 prescaler
pub type D1PPRE_R = crate::FieldReader<u8, D1PPRE_A>;
///D1 domain APB3 prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum D1PPRE_A {
    ///0: rcc_hclk not divided
    Div1 = 0,
    ///4: rcc_hclk divided by 2
    Div2 = 4,
    ///5: rcc_hclk divided by 4
    Div4 = 5,
    ///6: rcc_hclk divided by 8
    Div8 = 6,
    ///7: rcc_hclk divided by 16
    Div16 = 7,
}
impl From<D1PPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: D1PPRE_A) -> Self {
        variant as _
    }
}
impl D1PPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<D1PPRE_A> {
        match self.bits {
            0 => Some(D1PPRE_A::Div1),
            4 => Some(D1PPRE_A::Div2),
            5 => Some(D1PPRE_A::Div4),
            6 => Some(D1PPRE_A::Div8),
            7 => Some(D1PPRE_A::Div16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == D1PPRE_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == D1PPRE_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == D1PPRE_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == D1PPRE_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == D1PPRE_A::Div16
    }
}
///Field `D1PPRE` writer - D1 domain APB3 prescaler
pub type D1PPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, D1CFGR_SPEC, u8, D1PPRE_A, 3, O>;
impl<'a, const O: u8> D1PPRE_W<'a, O> {
    ///rcc_hclk not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(D1PPRE_A::Div1)
    }
    ///rcc_hclk divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(D1PPRE_A::Div2)
    }
    ///rcc_hclk divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(D1PPRE_A::Div4)
    }
    ///rcc_hclk divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(D1PPRE_A::Div8)
    }
    ///rcc_hclk divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(D1PPRE_A::Div16)
    }
}
///Field `D1CPRE` reader - D1 domain Core prescaler
pub use HPRE_R as D1CPRE_R;
///Field `D1CPRE` writer - D1 domain Core prescaler
pub use HPRE_W as D1CPRE_W;
impl R {
    ///Bits 0:3 - D1 domain AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - D1 domain APB3 prescaler
    #[inline(always)]
    pub fn d1ppre(&self) -> D1PPRE_R {
        D1PPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:11 - D1 domain Core prescaler
    #[inline(always)]
    pub fn d1cpre(&self) -> D1CPRE_R {
        D1CPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - D1 domain AHB prescaler
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<0> {
        HPRE_W::new(self)
    }
    ///Bits 4:6 - D1 domain APB3 prescaler
    #[inline(always)]
    #[must_use]
    pub fn d1ppre(&mut self) -> D1PPRE_W<4> {
        D1PPRE_W::new(self)
    }
    ///Bits 8:11 - D1 domain Core prescaler
    #[inline(always)]
    #[must_use]
    pub fn d1cpre(&mut self) -> D1CPRE_W<8> {
        D1CPRE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC Domain 1 Clock Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [d1cfgr](index.html) module
pub struct D1CFGR_SPEC;
impl crate::RegisterSpec for D1CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [d1cfgr::R](R) reader structure
impl crate::Readable for D1CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [d1cfgr::W](W) writer structure
impl crate::Writable for D1CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets D1CFGR to value 0
impl crate::Resettable for D1CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
