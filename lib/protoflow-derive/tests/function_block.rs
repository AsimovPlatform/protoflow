// This is free and unencumbered software released into the public domain.

// FIXME:
//#[test]
//fn define_function_block() {
//    use protoflow::derive::FunctionBlock;
//    use protoflow::{BlockResult, FunctionBlock, InputPort, OutputPort};
//
//    ///A block that simply echoes inputs to outputs.
//    #[derive(FunctionBlock, Clone)]
//    pub struct Echo(pub InputPort<i64>, pub OutputPort<i64>);
//
//    impl FunctionBlock<i64, i64> for Echo {
//        fn compute(&self, input: i64) -> BlockResult<i64> {
//            Ok(input)
//        }
//    }
//}
