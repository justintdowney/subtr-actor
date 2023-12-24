use crate::{BoostPad, BOOST_COOLDOWN, FRAMES_PER_SECOND};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct PadStateModeler {
    pad_to_disable_range: HashMap<BoostPad, Vec<(usize, usize)>>,
    disabled_pads: HashSet<BoostPad>,
    last_picked_up_index: HashMap<BoostPad, usize>
}

impl PadStateModeler {
    pub fn new() -> Self {
        Self {
            pad_to_disable_range: HashMap::new(),
            disabled_pads: HashSet::new(),
            last_picked_up_index: HashMap::new()
        }
    }

    pub fn disable_pad(&mut self, index: usize, boost_pad: &BoostPad) {
        let upperbound = index + (BOOST_COOLDOWN * FRAMES_PER_SECOND) as usize;
        self.pad_to_disable_range
            .entry(boost_pad.clone())
            .and_modify(|indices| indices.push((index, upperbound)))
            .or_insert(vec![(index, upperbound)]);
    }

    pub fn get_disabled_pads(&self) -> &HashSet<BoostPad> {
        &self.disabled_pads
    }

    pub fn update(&mut self, index: usize) {
        self.disabled_pads.clear();
    
        // Update disabled pads based on ranges
        for (pad, ranges) in &mut self.pad_to_disable_range {
            // Linear search for the current index within the ranges
            for i in (0..ranges.len()).rev() {
                let (start, end) = ranges[i];
                if index >= start && index <= end {
                    self.disabled_pads.insert(pad.clone());
                    self.last_picked_up_index.insert(pad.clone(), index);
                }
            }
        }

        if index % 1000 == 0 {
            println!("{}: PAD_TO_DISABLE: {:?}", index, self.pad_to_disable_range);
            println!("{}: DISABLED_PADS: {:?}", index, self.disabled_pads);
            println!("{}: LAST_PICKED_UP_INDEX: {:?}", index, self.last_picked_up_index);
        }
    
/*         // Update boost pickups
        let keys: Vec<BoostPad> = self.pad_to_disable_range.keys().cloned().collect();
    
        for pad in keys {
            if let Some(&last_picked_up_index) = self.last_picked_up_index.get(&pad) {
                if index > last_picked_up_index && index <= last_picked_up_index + (BOOST_COOLDOWN * FRAMES_PER_SECOND) as usize {
                    self.disable_pad(index, &pad);
                    self.last_picked_up_index.insert(pad.clone(), index);
                }
            }
        } */
    }
    

    pub fn reset(&mut self) {
        self.pad_to_disable_range.clear();
        self.disabled_pads.clear();
        self.last_picked_up_index.clear();
    }
}