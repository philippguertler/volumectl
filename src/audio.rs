extern crate libpulse_binding;

use libpulse_binding::context::Context;
use libpulse_binding::proplist::Proplist;
use libpulse_binding::mainloop::standard::Mainloop;
use libpulse_binding::mainloop::standard::IterateResult;
use libpulse_binding::def::Retval;
use libpulse_binding::callbacks::ListResult::*;
use libpulse_binding::context::State::*;
use libpulse_binding::operation::State::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use crate::rule::Rule;
use std::error::Error;
use self::libpulse_binding::context::introspect::{Introspector, SinkInputInfo};
use self::libpulse_binding::volume::{ChannelVolumes, Volume, VolumeLinear};
use self::libpulse_binding::sample::CHANNELS_MAX;

struct SinkInfo {
    index: u32,
    channels: u8
}

pub fn set_volume(rules: Vec<Rule>, volume: f64) -> Result<(), Box<Error>> {
    let mut proplist: Proplist = Proplist::new().unwrap();
    proplist.sets(libpulse_binding::proplist::properties::APPLICATION_NAME, "volumectl").unwrap();

    let mut mainloop = Mainloop::new().unwrap();

    let mut context = Context::new_with_proplist(
        &mainloop,
        "volumectl",
        &proplist
    ).unwrap();

    context.connect(None, libpulse_binding::context::flags::NOAUTOSPAWN, None).unwrap();

    wait_ready(&mut mainloop, &context);
    let mut intro = context.introspect();
    let sinks = get_matching_sink_inputs(&intro, &mut mainloop, rules);

    set_volumes(&mut intro,&mut mainloop, sinks.borrow().deref(), volume);


    mainloop.quit(Retval(0));
    context.disconnect();

    Ok(())
}

fn wait_ready(mainloop: &mut Mainloop, context: &Context) {
    loop {
        match mainloop.iterate(true) {
            IterateResult::Quit(_) |
            IterateResult::Err(_) => {
                eprintln!("iterate state was not success, quitting...");
                return;
            },
            IterateResult::Success(_) => {},
        }
        match context.get_state() {
            Ready => { break; },
            Failed |
            Terminated => {
                eprintln!("context state failed/terminated, quitting...");
                return;
            },
            _ => {},
        }
    }
}

fn get_matching_sink_inputs(intro: &Introspector, mainloop: &mut Mainloop, rules: Vec<Rule>) -> Rc<RefCell<Vec<SinkInfo>>> {
    let inputs: Rc<RefCell<Vec<SinkInfo>>> = Rc::new(RefCell::new(Vec::new()));
    let input_refs = inputs.clone();

    let rules_ref = Rc::new(rules);

    let op = intro.get_sink_input_info_list( move |res| {
        match res {
            Item(item) => {
                if any_match(rules_ref.deref(), item) {
                    if let Some(name) = item.proplist.gets("application.process.binary") {
                        println!("Setting volume for {}", name);
                    }
                    input_refs.borrow_mut().push(SinkInfo {
                        index: item.index,
                        channels: item.volume.channels
                    });
                }
            },
            End => {},
            Error => eprintln!("Error")
        }
    });

    while op.get_state() == Running {
        mainloop.iterate(true);
    }


    inputs
}


fn any_match(rules: &Vec<Rule>, sink: &SinkInputInfo) -> bool {
    for rule in rules {
        if let Some(value) = sink.proplist.gets(rule.property.as_str()) {
            if rule.pattern.is_match(value.as_str()) {
                return true;
            }
        }
    }

    false
}

fn set_volumes(intro: &mut Introspector, mainloop: &mut Mainloop, sinks: &[SinkInfo], volume: f64) {
    let vol = Volume::from(VolumeLinear(volume));
    let done = Rc::new(RefCell::new(0));

    for sink in sinks {
        let done_ref = done.clone();
        let volumes = ChannelVolumes {
            channels: sink.channels,
            values: [vol; CHANNELS_MAX]
        };
        intro.set_sink_input_volume(sink.index, &volumes, Some(Box::new(move |_success| {
            let old = *done_ref.borrow();
            done_ref.replace(old + 1);
        })));
    }

    while *done.borrow() < sinks.len() {
        mainloop.iterate(true);
    }
}
