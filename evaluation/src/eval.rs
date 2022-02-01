use board::small::SmallRequest;

pub trait StaticEval {
    fn static_eval(&self) -> i32;
}

impl StaticEval for SmallRequest {
    fn static_eval(&self) -> i32 {
        todo!();
    }
}
