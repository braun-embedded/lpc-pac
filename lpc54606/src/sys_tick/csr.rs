#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CSR_SPEC>> for R {
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl core::convert::From<crate::W<CSR_SPEC>> for W {
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: counter disabled"]
    ENABLE_0 = 0,
    #[doc = "1: counter enabled"]
    ENABLE_1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - no description available"]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::ENABLE_0,
            true => ENABLE_A::ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        **self == ENABLE_A::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        **self == ENABLE_A::ENABLE_1
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - no description available"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_0)
    }
    #[doc = "counter enabled"]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKINT_A {
    #[doc = "0: counting down to 0 does not assert the SysTick exception request"]
    TICKINT_0 = 0,
    #[doc = "1: counting down to 0 asserts the SysTick exception request"]
    TICKINT_1 = 1,
}
impl From<TICKINT_A> for bool {
    #[inline(always)]
    fn from(variant: TICKINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICKINT` reader - no description available"]
pub struct TICKINT_R(crate::FieldReader<bool, TICKINT_A>);
impl TICKINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TICKINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICKINT_A {
        match self.bits {
            false => TICKINT_A::TICKINT_0,
            true => TICKINT_A::TICKINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TICKINT_0`"]
    #[inline(always)]
    pub fn is_tickint_0(&self) -> bool {
        **self == TICKINT_A::TICKINT_0
    }
    #[doc = "Checks if the value of the field is `TICKINT_1`"]
    #[inline(always)]
    pub fn is_tickint_1(&self) -> bool {
        **self == TICKINT_A::TICKINT_1
    }
}
impl core::ops::Deref for TICKINT_R {
    type Target = crate::FieldReader<bool, TICKINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICKINT` writer - no description available"]
pub struct TICKINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TICKINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICKINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn tickint_0(self) -> &'a mut W {
        self.variant(TICKINT_A::TICKINT_0)
    }
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    #[inline(always)]
    pub fn tickint_1(self) -> &'a mut W {
        self.variant(TICKINT_A::TICKINT_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSOURCE_A {
    #[doc = "0: external clock"]
    CLKSOURCE_0 = 0,
    #[doc = "1: processor clock"]
    CLKSOURCE_1 = 1,
}
impl From<CLKSOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSOURCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSOURCE` reader - no description available"]
pub struct CLKSOURCE_R(crate::FieldReader<bool, CLKSOURCE_A>);
impl CLKSOURCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKSOURCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSOURCE_A {
        match self.bits {
            false => CLKSOURCE_A::CLKSOURCE_0,
            true => CLKSOURCE_A::CLKSOURCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLKSOURCE_0`"]
    #[inline(always)]
    pub fn is_clksource_0(&self) -> bool {
        **self == CLKSOURCE_A::CLKSOURCE_0
    }
    #[doc = "Checks if the value of the field is `CLKSOURCE_1`"]
    #[inline(always)]
    pub fn is_clksource_1(&self) -> bool {
        **self == CLKSOURCE_A::CLKSOURCE_1
    }
}
impl core::ops::Deref for CLKSOURCE_R {
    type Target = crate::FieldReader<bool, CLKSOURCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSOURCE` writer - no description available"]
pub struct CLKSOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSOURCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "external clock"]
    #[inline(always)]
    pub fn clksource_0(self) -> &'a mut W {
        self.variant(CLKSOURCE_A::CLKSOURCE_0)
    }
    #[doc = "processor clock"]
    #[inline(always)]
    pub fn clksource_1(self) -> &'a mut W {
        self.variant(CLKSOURCE_A::CLKSOURCE_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `COUNTFLAG` reader - no description available"]
pub struct COUNTFLAG_R(crate::FieldReader<bool, bool>);
impl COUNTFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        COUNTFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTFLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTFLAG` writer - no description available"]
pub struct COUNTFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTFLAG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn tickint(&mut self) -> TICKINT_W {
        TICKINT_W { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn clksource(&mut self) -> CLKSOURCE_W {
        CLKSOURCE_W { w: self }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn countflag(&mut self) -> COUNTFLAG_W {
        COUNTFLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0x04"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
