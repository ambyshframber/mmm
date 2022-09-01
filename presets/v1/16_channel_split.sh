#!/bin/env sh

{
    echo n in main $1
    for n in $(seq 16); do
        echo n channelf fil_$n $n
        echo n channelm mer_$n 1
        echo n out out_$n
        echo con main fil_$n
        echo con fil_$n mer_$n
        echo con mer_$n out_$n
    done
}
