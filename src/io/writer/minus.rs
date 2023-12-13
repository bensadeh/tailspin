use minus::{dynamic_paging, MinusError, Pager};
use std::fmt::Write;
use tokio::{
    io,
    task::{self, JoinHandle},
};

use crate::io::controller::Writer;

use super::AsyncLineWriter;

pub struct Minus {
    pager: Pager,
    join_handler: JoinHandle<Result<(), MinusError>>,
}

impl Minus {
    pub async fn init() -> Writer {
        let pager = Pager::new();
        let actual_pager = pager.clone();

        let join_handler = task::spawn(async { dynamic_paging(actual_pager) });

        Box::new(Self { pager, join_handler })
    }
}

#[async_trait::async_trait]
impl AsyncLineWriter for Minus {
    async fn write_line(&mut self, line: &str) -> io::Result<()> {
        let mut pager = self.pager.clone();
        writeln!(pager, "{}", line).unwrap();
        Ok(())
    }
}
