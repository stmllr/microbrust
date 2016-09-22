pub struct State<'a> {
    pub note_priority: &'a str,
    pub velocity_response: &'a str,
    pub play: &'a str,
    pub seq_retrig: &'a str,
    pub next_seq: &'a str,
    pub step_on: &'a str,
    pub step: &'a str,
    pub lfo_key_retrig: &'a str,
    pub env_legato_mode: &'a str,
    pub gate: &'a str,
    pub sync: &'a str,
    pub bend_range: &'a str,
    pub midi_recv_chan: &'a str,
    pub midi_send_chan: &'a str,
}

impl<'a> State<'a> {
    pub fn new() -> State<'a> {
        State {
            note_priority: "UNKNOWN",
            velocity_response: "UNKNOWN",
            play: "UNKNOWN",
            seq_retrig: "UNKNOWN",
            next_seq: "UNKNOWN",
            step_on: "UNKNOWN",
            step: "UNKNOWN",
            lfo_key_retrig: "UNKNOWN",
            env_legato_mode: "UNKNOWN",
            gate: "UNKNOWN",
            sync: "UNKNOWN",
            bend_range: "UNKNOWN",
            midi_recv_chan: "UNKNOWN",
            midi_send_chan: "UNKNOWN",
        }
    }

    pub fn print(&self) {
        println!("ARTURIA MICROBRUTE STATE");
        println!("[n] Note priority: {}", self.note_priority);
        println!("[v] Velocity response: {}", self.velocity_response);
        println!("[p] Play: {}", self.play);
        println!("[x] Next sequence: {}", self.next_seq);
        println!("[q] Sequence retrig: {}", self.seq_retrig);
        println!("[t] Step: {}", self.step);
        println!("[o] Step on: {}", self.step_on);
        println!("[l] LFO key retrig: {}", self.lfo_key_retrig);
        println!("[e] Env legatio mode: {}", self.env_legato_mode);
        println!("[g] Gate: {}", self.gate);
        println!("[y] Sync: {}", self.sync);
        println!("[b] Bend range: {}", self.bend_range);
        println!("[r] MIDI receive channel: {}", self.midi_recv_chan);
        println!("[s] MIDI send channel: {}", self.midi_send_chan);
    }


    pub fn set(&mut self, param: &'a str, value: &'a str) {
        match param {
            "note_priority" => self.note_priority = value,
            "velocity_response" => self.velocity_response = value,
            "play" => self.play = value,
            "seq_retrig" => self.seq_retrig = value,
            "next_seq" => self.next_seq = value,
            "step_on" => self.step_on = value,
            "step" => self.step = value,
            "lfo_key_retrig" => self.lfo_key_retrig = value,
            "env_legato_mode" => self.env_legato_mode = value,
            "gate" => self.gate = value,
            "sync" => self.sync = value,
            "bend_range" => self.bend_range = value,
            "midi_recv_chan" => self.midi_recv_chan = value,
            "midi_send_chan" => self.midi_send_chan = value,
            _ => ()
        }
    }
}
