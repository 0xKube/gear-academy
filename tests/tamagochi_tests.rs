#[cfg(test)]
mod tests {
    extern crate std;

    use gstd::ToOwned;
    use gtest::{Log, Program, System};
    use tamagochi_1::{TmgAction, TmgEvent};

    #[test]
    fn it_works() {
        let sys = System::new();
        sys.init_logger();
        let program = Program::current(&sys);
        let res = program.send_bytes(42, "Hello");
        assert!(!res.main_failed());
        assert!(res.log().is_empty());

        let res = program.send(3, TmgAction::Name);
        let expected_log = Log::builder().payload(TmgEvent::Name("Valera".to_owned()));
        println!("log is: {:#?}", res.decoded_log::<TmgEvent>());

        assert!(res.contains(&expected_log));
    }
}
