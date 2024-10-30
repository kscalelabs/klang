fn wave(limb: string, joint: number, amount: number) : "Wave a hand back and forth" {
    let pos = get_position(limb: limb, joint: joint);
    let pos = set_position(limb: limb, joint: joint, pos: pos + amount);
    let pos = set_position(limb: limb, joint: joint, pos: pos - amount);
    return pos;
}

fn main() : "Wave a hand back and forth a few times" {
    wave(limb: "right_arm", joint: 0, amount: 45deg);
}
