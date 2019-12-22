#[cfg(test)]
mod unit_tests;

use super::*;

#[derive(Debug, Default, PartialEq)]
pub struct ServerBuilder {
    bind_network_interface: Option<BindNetworkInterface>,
    terminate_condition: Option<TerminateCondition>,
}

impl ServerBuilder {
    pub fn bind_network_interface(mut self, bind_network_interface: BindNetworkInterface) -> Self {
        self.bind_network_interface = Some(bind_network_interface);
        self
    }

    pub fn build<'a>(self) -> Server<'a> {
        Server {
            bind_network_interface: self
                .bind_network_interface
                .unwrap_or(BindNetworkInterface::Any),
            ip_addr: None,
            started: false,
            terminate_condition: self
                .terminate_condition
                .unwrap_or(TerminateCondition::Never),
            phantom: PhantomData,
        }
    }

    pub fn terminate_condition(mut self, terminate_condition: TerminateCondition) -> Self {
        self.terminate_condition = Some(terminate_condition);
        self
    }
}
