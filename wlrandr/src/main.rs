// TODO: Handle output management?

use sctk::{
    output::{
        OutputInfo,
        OutputState,
        OutputHandler,
    },
    registry::{
        RegistryState,
        ProvidesRegistryState
    },
    reexports::client::{
        protocol::wl_output,
        Connection, QueueHandle,
    },
    delegate_registry, delegate_output, registry_handlers,
};

struct WlRandr {
    registry_state: RegistryState,
    output_state: OutputState
}

impl ProvidesRegistryState for WlRandr {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    registry_handlers! {
        OutputState
    }
}

delegate_registry!(WlRandr);

impl OutputHandler for WlRandr {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &sctk::reexports::client::Connection,
        _qh: &sctk::reexports::client::QueueHandle<Self>,
        _output: sctk::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }
}

delegate_output!(WlRandr);

fn print_info(info: &OutputInfo) {
    println!("model: {} [", info.model);

    if let Some(name) = info.name.as_ref() {
        println!("  name: {}", name);
    }

    if let Some(description) = info.description.as_ref() {
        println!("  description: {}", description);
    }

    println!("  make: {}", info.make);
    println!("  location: {}x{}", info.location.0, info.location.1);
    println!("  subpixel: {:?}", info.subpixel);
    println!("  physical_size: {}x{}mm", info.physical_size.0, info.physical_size.1);
    println!("  modes: [");

    info.modes
        .iter()
        .for_each(|mode| {
            println!("    {}", mode);
            println!("  ]");
        });

    println!("]");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let conn = Connection::connect_to_env()?;

    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let registry_state = RegistryState::new(&conn, &qh);

    let output_delegate = OutputState::new();

    let mut list_outputs = WlRandr {
        registry_state,
        output_state: output_delegate
    };

    while !list_outputs.registry_state.ready() {
        event_queue.blocking_dispatch(&mut list_outputs)?;
    }

    event_queue.sync_roundtrip(&mut list_outputs)?;

    list_outputs.output_state
        .outputs()
        .for_each(|output| {
            print_info(
                &list_outputs.output_state
                    .info(&output)
                    .ok_or_else(|| "Output has no info".to_owned()).unwrap(),
            );
        });

    Ok(())
}
