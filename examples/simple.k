> wave [arm] arm {
    > wave [arm] joint [joint] twice {
        move [arm] joint [joint] to 90
        move [arm] joint [joint] to 0
    }

    wave [arm] joint 1 twice
    wave [arm] joint 2 twice
    wave [arm] joint 3 twice
}

> wave both arms {
    " wave [right] arm
    " wave [left] arm
}

" wave both arms
