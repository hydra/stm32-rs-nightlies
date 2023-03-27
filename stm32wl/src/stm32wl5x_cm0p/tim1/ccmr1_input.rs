///Register `CCMR1_Input` reader
pub struct R(crate::R<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR1_Input` writer
pub struct W(crate::W<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_INPUT_SPEC>;
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
impl From<crate::W<CCMR1_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC1S` reader - Capture/Compare 1 selection
pub type CC1S_R = crate::FieldReader<u8, CC1S_A>;
///Capture/Compare 1 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1S_A {
    ///0: CCx channel is configured as output
    Output = 0,
    ///1: CCx channel is configured as input, ICx is mapped on TI1
    Ti1 = 1,
    ///2: CCx channel is configured as input, ICx is mapped on TI2
    Ti2 = 2,
    ///3: CCx channel is configured as input, ICx is mapped on TRC
    Trc = 3,
}
impl From<CC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1S_A) -> Self {
        variant as _
    }
}
impl CC1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1S_A {
        match self.bits {
            0 => CC1S_A::Output,
            1 => CC1S_A::Ti1,
            2 => CC1S_A::Ti2,
            3 => CC1S_A::Trc,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Output`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC1S_A::Output
    }
    ///Checks if the value of the field is `Ti1`
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        *self == CC1S_A::Ti1
    }
    ///Checks if the value of the field is `Ti2`
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        *self == CC1S_A::Ti2
    }
    ///Checks if the value of the field is `Trc`
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC1S_A::Trc
    }
}
///Field `CC1S` writer - Capture/Compare 1 selection
pub type CC1S_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR1_INPUT_SPEC, u8, CC1S_A, 2, O>;
impl<'a, const O: u8> CC1S_W<'a, O> {
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC1S_A::Output)
    }
    ///CCx channel is configured as input, ICx is mapped on TI1
    #[inline(always)]
    pub fn ti1(self) -> &'a mut W {
        self.variant(CC1S_A::Ti1)
    }
    ///CCx channel is configured as input, ICx is mapped on TI2
    #[inline(always)]
    pub fn ti2(self) -> &'a mut W {
        self.variant(CC1S_A::Ti2)
    }
    ///CCx channel is configured as input, ICx is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC1S_A::Trc)
    }
}
///Field `IC1PSC` reader - Input capture 1 prescaler
pub type IC1PSC_R = crate::FieldReader<u8, IC1PSC_A>;
///Input capture 1 prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1PSC_A {
    ///0: CCx channel is configured as output
    Output = 0,
    ///1: Capture is done once every 2 events
    Capture2 = 1,
    ///2: Capture is done once every 4 events
    Capture4 = 2,
    ///3: Capture is done once every 8 events
    Capture8 = 3,
}
impl From<IC1PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1PSC_A) -> Self {
        variant as _
    }
}
impl IC1PSC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IC1PSC_A {
        match self.bits {
            0 => IC1PSC_A::Output,
            1 => IC1PSC_A::Capture2,
            2 => IC1PSC_A::Capture4,
            3 => IC1PSC_A::Capture8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Output`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == IC1PSC_A::Output
    }
    ///Checks if the value of the field is `Capture2`
    #[inline(always)]
    pub fn is_capture2(&self) -> bool {
        *self == IC1PSC_A::Capture2
    }
    ///Checks if the value of the field is `Capture4`
    #[inline(always)]
    pub fn is_capture4(&self) -> bool {
        *self == IC1PSC_A::Capture4
    }
    ///Checks if the value of the field is `Capture8`
    #[inline(always)]
    pub fn is_capture8(&self) -> bool {
        *self == IC1PSC_A::Capture8
    }
}
///Field `IC1PSC` writer - Input capture 1 prescaler
pub type IC1PSC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR1_INPUT_SPEC, u8, IC1PSC_A, 2, O>;
impl<'a, const O: u8> IC1PSC_W<'a, O> {
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(IC1PSC_A::Output)
    }
    ///Capture is done once every 2 events
    #[inline(always)]
    pub fn capture2(self) -> &'a mut W {
        self.variant(IC1PSC_A::Capture2)
    }
    ///Capture is done once every 4 events
    #[inline(always)]
    pub fn capture4(self) -> &'a mut W {
        self.variant(IC1PSC_A::Capture4)
    }
    ///Capture is done once every 8 events
    #[inline(always)]
    pub fn capture8(self) -> &'a mut W {
        self.variant(IC1PSC_A::Capture8)
    }
}
///Field `IC1F` reader - Input capture 1 filter
pub type IC1F_R = crate::FieldReader<u8, IC1F_A>;
///Input capture 1 filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1F_A {
    ///0: No filter, sampling is done at fDTS
    NoFilter = 0,
    ///1: fSAMPLING=fCK_INT, N=2
    FckIntN2 = 1,
    ///2: fSAMPLING=fCK_INT, N=4
    FckIntN4 = 2,
    ///3: fSAMPLING=fCK_INT, N=8
    FckIntN8 = 3,
    ///4: fSAMPLING=fDTS/2, N=6
    FdtsDiv2N6 = 4,
    ///5: fSAMPLING=fDTS/2, N=8
    FdtsDiv2N8 = 5,
    ///6: fSAMPLING=fDTS/4, N=6
    FdtsDiv4N6 = 6,
    ///7: fSAMPLING=fDTS/4, N=8
    FdtsDiv4N8 = 7,
    ///8: fSAMPLING=fDTS/8, N=6
    FdtsDiv8N6 = 8,
    ///9: fSAMPLING=fDTS/8, N=8
    FdtsDiv8N8 = 9,
    ///10: fSAMPLING=fDTS/16, N=5
    FdtsDiv16N5 = 10,
    ///11: fSAMPLING=fDTS/16, N=6
    FdtsDiv16N6 = 11,
    ///12: fSAMPLING=fDTS/16, N=8
    FdtsDiv16N8 = 12,
    ///13: fSAMPLING=fDTS/32, N=5
    FdtsDiv32N5 = 13,
    ///14: fSAMPLING=fDTS/32, N=6
    FdtsDiv32N6 = 14,
    ///15: fSAMPLING=fDTS/32, N=8
    FdtsDiv32N8 = 15,
}
impl From<IC1F_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1F_A) -> Self {
        variant as _
    }
}
impl IC1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IC1F_A {
        match self.bits {
            0 => IC1F_A::NoFilter,
            1 => IC1F_A::FckIntN2,
            2 => IC1F_A::FckIntN4,
            3 => IC1F_A::FckIntN8,
            4 => IC1F_A::FdtsDiv2N6,
            5 => IC1F_A::FdtsDiv2N8,
            6 => IC1F_A::FdtsDiv4N6,
            7 => IC1F_A::FdtsDiv4N8,
            8 => IC1F_A::FdtsDiv8N6,
            9 => IC1F_A::FdtsDiv8N8,
            10 => IC1F_A::FdtsDiv16N5,
            11 => IC1F_A::FdtsDiv16N6,
            12 => IC1F_A::FdtsDiv16N8,
            13 => IC1F_A::FdtsDiv32N5,
            14 => IC1F_A::FdtsDiv32N6,
            15 => IC1F_A::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoFilter`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == IC1F_A::NoFilter
    }
    ///Checks if the value of the field is `FckIntN2`
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == IC1F_A::FckIntN2
    }
    ///Checks if the value of the field is `FckIntN4`
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == IC1F_A::FckIntN4
    }
    ///Checks if the value of the field is `FckIntN8`
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == IC1F_A::FckIntN8
    }
    ///Checks if the value of the field is `FdtsDiv2N6`
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == IC1F_A::FdtsDiv2N6
    }
    ///Checks if the value of the field is `FdtsDiv2N8`
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == IC1F_A::FdtsDiv2N8
    }
    ///Checks if the value of the field is `FdtsDiv4N6`
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == IC1F_A::FdtsDiv4N6
    }
    ///Checks if the value of the field is `FdtsDiv4N8`
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == IC1F_A::FdtsDiv4N8
    }
    ///Checks if the value of the field is `FdtsDiv8N6`
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == IC1F_A::FdtsDiv8N6
    }
    ///Checks if the value of the field is `FdtsDiv8N8`
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == IC1F_A::FdtsDiv8N8
    }
    ///Checks if the value of the field is `FdtsDiv16N5`
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == IC1F_A::FdtsDiv16N5
    }
    ///Checks if the value of the field is `FdtsDiv16N6`
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == IC1F_A::FdtsDiv16N6
    }
    ///Checks if the value of the field is `FdtsDiv16N8`
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == IC1F_A::FdtsDiv16N8
    }
    ///Checks if the value of the field is `FdtsDiv32N5`
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == IC1F_A::FdtsDiv32N5
    }
    ///Checks if the value of the field is `FdtsDiv32N6`
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == IC1F_A::FdtsDiv32N6
    }
    ///Checks if the value of the field is `FdtsDiv32N8`
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == IC1F_A::FdtsDiv32N8
    }
}
///Field `IC1F` writer - Input capture 1 filter
pub type IC1F_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR1_INPUT_SPEC, u8, IC1F_A, 4, O>;
impl<'a, const O: u8> IC1F_W<'a, O> {
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC1F_A::NoFilter)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(IC1F_A::FckIntN2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(IC1F_A::FckIntN4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FckIntN8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv2N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv2N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv4N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv4N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv8N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv8N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv16N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv16N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv16N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv32N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv32N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FdtsDiv32N8)
    }
}
///Field `CC2S` reader - Capture/compare 2 selection
pub use CC1S_R as CC2S_R;
///Field `CC2S` writer - Capture/compare 2 selection
pub use CC1S_W as CC2S_W;
///Field `IC2F` reader - Input capture 2 filter
pub use IC1F_R as IC2F_R;
///Field `IC2F` writer - Input capture 2 filter
pub use IC1F_W as IC2F_W;
///Field `IC2PSC` reader - Input capture 2 prescaler
pub use IC1PSC_R as IC2PSC_R;
///Field `IC2PSC` writer - Input capture 2 prescaler
pub use IC1PSC_W as IC2PSC_W;
impl R {
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Capture/compare 2 selection
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Input capture 2 prescaler
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:15 - Input capture 2 filter
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> IC1PSC_W<2> {
        IC1PSC_W::new(self)
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> IC1F_W<4> {
        IC1F_W::new(self)
    }
    ///Bits 8:9 - Capture/compare 2 selection
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<8> {
        CC2S_W::new(self)
    }
    ///Bits 10:11 - Input capture 2 prescaler
    #[inline(always)]
    #[must_use]
    pub fn ic2psc(&mut self) -> IC2PSC_W<10> {
        IC2PSC_W::new(self)
    }
    ///Bits 12:15 - Input capture 2 filter
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> IC2F_W<12> {
        IC2F_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare mode register 1 (input mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr1_input](index.html) module
pub struct CCMR1_INPUT_SPEC;
impl crate::RegisterSpec for CCMR1_INPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr1_input::R](R) reader structure
impl crate::Readable for CCMR1_INPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr1_input::W](W) writer structure
impl crate::Writable for CCMR1_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCMR1_Input to value 0
impl crate::Resettable for CCMR1_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
