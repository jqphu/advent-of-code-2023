def day1():
    times = [int(s) for s in input().split(':', 1)[1].split()]
    distances = [int(s) for s in input().split(':', 1)[1].split()]
    total = 1

    for i in range(0, len(times)):
        beat_record = 0
        time = times[i]
        distance = distances[i]
        for hold_for in range(0, time):
            if hold_for * (time - hold_for) > distance:
                beat_record += 1
        print(beat_record)
        total *= beat_record

    print(total)

# Mathematically given the time is
# time: 71530
# distance: 940200
#
# Let's try to deduce mathematical formula
#
# distance >= (time - held_time) * held_time

# distance_travelled = held_time * (time - held_time)
# let x = held_time
# Need to solve for x where
# distance_travelled < x * time - x^2
# x^2 - x*time + distance_travelled < 0
# x^2 - 71530*x + 940200 < 0
#
# sol 13.14 < x < 71516.85
# 71516 - 13

if __name__ == "__main__":
    day1()
