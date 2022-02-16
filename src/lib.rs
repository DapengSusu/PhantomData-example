use std::{marker::PhantomData, sync::atomic::{AtomicU64, Ordering}};

// AtomicU64：可在线程间安全地共享的整数类型
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>, // 幽灵类型：广泛用在处理，数据结构定义过程中不需要，但是在实现过程中需要的泛型参数
}

impl<T> Customer<T> {
    pub fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature1 for {}.", self.name);
    }

    fn feature2(&self) {
        println!("feature2 for {}.", self.name);
    }
}

pub struct FreePlan;
pub struct PersonalPlan(f64);

impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!(
            "Dear {}(as our valuable customer {}), enjoy this advanced feature.",
            self.name, self.id
        );
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name)
    }
}

/// 订阅成为付费用户
pub fn subscribe(customer: Customer<FreePlan>, payment: f64) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);

    // 存储 plan 到 DB
    // ...

    customer.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer() {
        // 免费用户
        let customer = Customer::<FreePlan>::new("Jumper".into());

        customer.feature1();
        customer.feature2();
        // customer.advance_feature(); // Error: 不能享受高级 feature

        // 付费升级
        let customer = subscribe(customer, 6.99);

        customer.feature1();
        customer.feature2();
        // 付费用户解锁高级 feature
        customer.advance_feature();
    }
}
