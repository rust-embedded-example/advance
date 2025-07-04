#[doc = "Register `SQR2` reader"]
pub type R = crate::R<Sqr2Spec>;
#[doc = "Register `SQR2` writer"]
pub type W = crate::W<Sqr2Spec>;
#[doc = "Field `SQ5` reader - SQ5"]
pub type Sq5R = crate::FieldReader;
#[doc = "Field `SQ5` writer - SQ5"]
pub type Sq5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ6` reader - SQ6"]
pub type Sq6R = crate::FieldReader;
#[doc = "Field `SQ6` writer - SQ6"]
pub type Sq6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ7` reader - SQ7"]
pub type Sq7R = crate::FieldReader;
#[doc = "Field `SQ7` writer - SQ7"]
pub type Sq7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ8` reader - SQ8"]
pub type Sq8R = crate::FieldReader;
#[doc = "Field `SQ8` writer - SQ8"]
pub type Sq8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ9` reader - SQ9"]
pub type Sq9R = crate::FieldReader;
#[doc = "Field `SQ9` writer - SQ9"]
pub type Sq9W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - SQ5"]
    #[inline(always)]
    pub fn sq5(&self) -> Sq5R {
        Sq5R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - SQ6"]
    #[inline(always)]
    pub fn sq6(&self) -> Sq6R {
        Sq6R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - SQ7"]
    #[inline(always)]
    pub fn sq7(&self) -> Sq7R {
        Sq7R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - SQ8"]
    #[inline(always)]
    pub fn sq8(&self) -> Sq8R {
        Sq8R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - SQ9"]
    #[inline(always)]
    pub fn sq9(&self) -> Sq9R {
        Sq9R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SQ5"]
    #[inline(always)]
    pub fn sq5(&mut self) -> Sq5W<'_, Sqr2Spec> {
        Sq5W::new(self, 0)
    }
    #[doc = "Bits 6:10 - SQ6"]
    #[inline(always)]
    pub fn sq6(&mut self) -> Sq6W<'_, Sqr2Spec> {
        Sq6W::new(self, 6)
    }
    #[doc = "Bits 12:16 - SQ7"]
    #[inline(always)]
    pub fn sq7(&mut self) -> Sq7W<'_, Sqr2Spec> {
        Sq7W::new(self, 12)
    }
    #[doc = "Bits 18:22 - SQ8"]
    #[inline(always)]
    pub fn sq8(&mut self) -> Sq8W<'_, Sqr2Spec> {
        Sq8W::new(self, 18)
    }
    #[doc = "Bits 24:28 - SQ9"]
    #[inline(always)]
    pub fn sq9(&mut self) -> Sq9W<'_, Sqr2Spec> {
        Sq9W::new(self, 24)
    }
}
#[doc = "regular sequence register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sqr2Spec;
impl crate::RegisterSpec for Sqr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr2::R`](R) reader structure"]
impl crate::Readable for Sqr2Spec {}
#[doc = "`write(|w| ..)` method takes [`sqr2::W`](W) writer structure"]
impl crate::Writable for Sqr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SQR2 to value 0"]
impl crate::Resettable for Sqr2Spec {}
