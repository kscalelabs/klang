> wave [arm] arm {
    > wave [arm] joint [joint] twice {
        move joint [joint] on [arm] to 90
        move joint [joint] on [arm] to 0
    }

    " wave [arm] joint [1] twice
    " wave [arm] joint [2] twice
    " wave [arm] joint [3] twice
}

> wave both arms {
    " wave [right] arm
    " wave [left] arm
}

" wave both arms
