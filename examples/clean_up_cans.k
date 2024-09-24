from robot import move, ppo_standing, nn

fn pick_up_apple_manual() : "Pick up an apple manually" {
    x = 30deg;
    y = 30deg;
    *move(limb="left_arm", joint=2, pos=x + y);
}

fn pick_up_apple() : "Pick up an apple" {
    *nn(
        "Pick up the apple",
        pos=["Move hand close to the apple"],
        neg=["Move quickly"],
        on_failure=pick_up_apple_manual,
    );
}

fn random_arm_movement() : "Move the arm in a random manner" {
    for i : [1, 2, 3] {
        *(
            move(limb="right_arm", joint=2, pos=30deg + i * 0.1rad),
            move(limb="right_arm", joint=3, pos=30deg + i * 0.5rad),
        );
    }

    counter = 0;
    flag = true;
    while counter < 5 {
        *(
            move("right_arm", joint=2, pos=30deg + (flag ? 0.1rad : -0.1rad)),
            move("right_arm", joint=3, pos=30deg + (flag ? 0.5rad : -0.5rad)),
        );
        flag = !flag;
        counter += 1;
    }
}

fn main() {
    *(pick_up_apple(), ppo_standing(), random_arm_movement());
}
