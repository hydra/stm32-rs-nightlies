///Register `EXTCFGR` reader
pub struct R(crate::R<EXTCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTCFGR` writer
pub struct W(crate::W<EXTCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTCFGR_SPEC>;
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
impl From<crate::W<EXTCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SHDHPRE` reader - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
pub type SHDHPRE_R = crate::FieldReader<u8, SHDHPRE_A>;
///HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHDHPRE_A {
    ///0: SYSCLK not divided
    Div1 = 0,
    ///1: SYSCLK divided by 3
    Div3 = 1,
    ///2: SYSCLK divided by 5
    Div5 = 2,
    ///5: SYSCLK divided by 6
    Div6 = 5,
    ///6: SYSCLK divided by 10
    Div10 = 6,
    ///7: SYSCLK divided by 32
    Div32 = 7,
    ///8: SYSCLK divided by 2
    Div2 = 8,
    ///9: SYSCLK divided by 4
    Div4 = 9,
    ///10: SYSCLK divided by 8
    Div8 = 10,
    ///11: SYSCLK divided by 16
    Div16 = 11,
    ///12: SYSCLK divided by 64
    Div64 = 12,
    ///13: SYSCLK divided by 128
    Div128 = 13,
    ///14: SYSCLK divided by 128
    Div256 = 14,
    ///15: SYSCLK divided by 512
    Div512 = 15,
}
impl From<SHDHPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: SHDHPRE_A) -> Self {
        variant as _
    }
}
impl SHDHPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SHDHPRE_A> {
        match self.bits {
            0 => Some(SHDHPRE_A::Div1),
            1 => Some(SHDHPRE_A::Div3),
            2 => Some(SHDHPRE_A::Div5),
            5 => Some(SHDHPRE_A::Div6),
            6 => Some(SHDHPRE_A::Div10),
            7 => Some(SHDHPRE_A::Div32),
            8 => Some(SHDHPRE_A::Div2),
            9 => Some(SHDHPRE_A::Div4),
            10 => Some(SHDHPRE_A::Div8),
            11 => Some(SHDHPRE_A::Div16),
            12 => Some(SHDHPRE_A::Div64),
            13 => Some(SHDHPRE_A::Div128),
            14 => Some(SHDHPRE_A::Div256),
            15 => Some(SHDHPRE_A::Div512),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SHDHPRE_A::Div1
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == SHDHPRE_A::Div3
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == SHDHPRE_A::Div5
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == SHDHPRE_A::Div6
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == SHDHPRE_A::Div10
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == SHDHPRE_A::Div32
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SHDHPRE_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SHDHPRE_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SHDHPRE_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == SHDHPRE_A::Div16
    }
    ///Checks if the value of the field is `Div64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == SHDHPRE_A::Div64
    }
    ///Checks if the value of the field is `Div128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == SHDHPRE_A::Div128
    }
    ///Checks if the value of the field is `Div256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == SHDHPRE_A::Div256
    }
    ///Checks if the value of the field is `Div512`
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == SHDHPRE_A::Div512
    }
}
///Field `SHDHPRE` writer - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
pub type SHDHPRE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTCFGR_SPEC, u8, SHDHPRE_A, 4, O>;
impl<'a, const O: u8> SHDHPRE_W<'a, O> {
    ///SYSCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div1)
    }
    ///SYSCLK divided by 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div3)
    }
    ///SYSCLK divided by 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div5)
    }
    ///SYSCLK divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div6)
    }
    ///SYSCLK divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div10)
    }
    ///SYSCLK divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div32)
    }
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div2)
    }
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div4)
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div8)
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div16)
    }
    ///SYSCLK divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div64)
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div128)
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div256)
    }
    ///SYSCLK divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(SHDHPRE_A::Div512)
    }
}
///Field `SHDHPREF` reader - HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)
pub type SHDHPREF_R = crate::BitReader<SHDHPREF_A>;
///HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHDHPREF_A {
    ///0: HCLK3 prescaler value not yet applied
    NotApplied = 0,
    ///1: HCLK3 prescaler value applied
    Applied = 1,
}
impl From<SHDHPREF_A> for bool {
    #[inline(always)]
    fn from(variant: SHDHPREF_A) -> Self {
        variant as u8 != 0
    }
}
impl SHDHPREF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SHDHPREF_A {
        match self.bits {
            false => SHDHPREF_A::NotApplied,
            true => SHDHPREF_A::Applied,
        }
    }
    ///Checks if the value of the field is `NotApplied`
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        *self == SHDHPREF_A::NotApplied
    }
    ///Checks if the value of the field is `Applied`
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        *self == SHDHPREF_A::Applied
    }
}
impl R {
    ///Bits 0:3 - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
    #[inline(always)]
    pub fn shdhpre(&self) -> SHDHPRE_R {
        SHDHPRE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 16 - HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)
    #[inline(always)]
    pub fn shdhpref(&self) -> SHDHPREF_R {
        SHDHPREF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
    #[inline(always)]
    #[must_use]
    pub fn shdhpre(&mut self) -> SHDHPRE_W<0> {
        SHDHPRE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Extended clock recovery register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [extcfgr](index.html) module
pub struct EXTCFGR_SPEC;
impl crate::RegisterSpec for EXTCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [extcfgr::R](R) reader structure
impl crate::Readable for EXTCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [extcfgr::W](W) writer structure
impl crate::Writable for EXTCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EXTCFGR to value 0x0003_0000
impl crate::Resettable for EXTCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0000;
}
