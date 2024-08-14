# /usr/bin/env klang
# Defines a simple klang program.
# This program is designed to get a robot to clean up soda cans.

action <Teleop> {
    outcomes {
        success [Success]
        failure [Failure]
    }
}

action <Find {item}> {
    notes {
        prefer [Move quickly]
        avoid [Bumping into other objects]
        avoid [Moving too hastily]
        avoid [Moving slowly or staying in the same place for a long time]
    }

    outcomes {
        success [Found {item}]
        failure [Haven't moved in a while]
    }
}

action <Pick up {item}> {
    outcomes {
        success [Holding {item}]
        failure [Knocked over {item}]
        retry [Not holding {item}]
    }
}

action <Put {item} into {container}> {
    outcomes {
        success [{item} is inside {container} and we are not holding it]
        failure [Dropped {item} outside the {container}]
        retry [Still holding {item}]
    }
}

action <Empty the soda can into the sink> {
    outcomes {
        success [Successfully emptied the soda can into the sink]
        failure [Spilled soda outside the sink]
        retry [There is still soda in the can]
    }
}

loop {
    <Find a soda can>
    <Pick up the soda can>
    <Find the sink>
    <Empty the soda can into the sink>
    <Find the recycling bin>
    <Put the soda can into the recycling bin>
} until {
    [No more soda cans]
}
