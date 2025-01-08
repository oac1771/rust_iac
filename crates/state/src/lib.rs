mod provider;
mod state;

use provider::dummy_provider::prelude::DummyProvider;
use state::State;

fn _plan() {
    let provider = DummyProvider;
    let state = State::new(provider);

    state.plan()
}
