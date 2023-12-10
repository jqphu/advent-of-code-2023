import sys
from dataclasses import dataclass


# Range [start, end)
@dataclass
class LookupEntry:
    start_range: int
    end_range: int
    offset: int


# Note: we modify the input to remove all the headers and just split by double new line.
# Assume the input ranges never overlap.
def day1():
    input_data = sys.stdin.read()
    parts = input_data.split("\n\n")
    # Seeds as integers.
    seeds = [int(s) for s in parts[0].split(" ")]

    maps = parts[1:]

    for map in maps:
        # Mapping from range to offset.
        #
        # e.g. if number is 32, look for which lookup key contains 32 and add that to the offset.
        #
        # Sorted by start_range!
        processed_lookups = [LookupEntry(0, 99999999999999999999, 0)]

        map = map.strip()

        for lookup in map.split("\n"):
            lookup_numbers = [int(s) for s in lookup.split(" ")]
            # print("Lookup:", lookup_numbers)
            # 50
            destination_number = lookup_numbers[0]
            # 98
            source_number = lookup_numbers[1]
            # 2
            amount = lookup_numbers[2]

            # (0,200): 0
            for i, item in enumerate(processed_lookups):
                # We need to spit this current entry into two.
                if source_number < item.end_range:
                    # 200
                    original_end_range = item.end_range

                    # End the current range early.
                    # (0, 98): 0
                    item.end_range = source_number

                    # Insert the current range after this current element
                    # (98, 100): 50
                    processed_lookups.insert(
                        i + 1,
                        LookupEntry(
                            source_number, source_number + amount, destination_number
                        ),
                    )

                    if original_end_range > source_number + amount:
                        # print("Original range extension")
                        # If there's still more, use the old range.
                        # (100, 200): 100
                        processed_lookups.insert(
                            i + 2,
                            LookupEntry(
                                source_number + amount,
                                original_end_range,
                                item.offset
                                + (source_number + amount - item.start_range),
                            ),
                        )
                    break

            # print("Processed:", processed_lookups)
        new_seeds = []
        for seed in seeds:
            # Find the seeds
            for item in processed_lookups:
                # inside this range.
                if seed < item.end_range:
                    value = (seed - item.start_range) + item.offset
                    new_seeds.append(value)
                    break
        seeds = new_seeds
        print("|seed:", seeds)

    print(min(seeds))


# Note: we modify the input to remove all the headers and just split by double new line.
# Assume the input ranges never overlap.
def day2():
    input_data = sys.stdin.read()
    parts = input_data.split("\n\n")
    # Seeds as integers.
    seeds = [int(s) for s in parts[0].split(" ")]

    maps = parts[1:]

    for map in maps:
        # Mapping from range to offset.
        #
        # e.g. if number is 32, look for which lookup key contains 32 and add that to the offset.
        #
        # Sorted by start_range!
        processed_lookups = [LookupEntry(0, 99999999999999999999, 0)]

        map = map.strip()

        for lookup in map.split("\n"):
            lookup_numbers = [int(s) for s in lookup.split(" ")]
            # print("Lookup:", lookup_numbers)
            # 50
            destination_number = lookup_numbers[0]
            # 98
            source_number = lookup_numbers[1]
            # 2
            amount = lookup_numbers[2]

            # (0,200): 0
            for i, item in enumerate(processed_lookups):
                # We need to spit this current entry into two.
                if source_number < item.end_range:
                    # 200
                    original_end_range = item.end_range

                    # End the current range early.
                    # (0, 98): 0
                    item.end_range = source_number

                    # Insert the current range after this current element
                    # (98, 100): 50
                    processed_lookups.insert(
                        i + 1,
                        LookupEntry(
                            source_number, source_number + amount, destination_number
                        ),
                    )

                    if original_end_range > source_number + amount:
                        # print("Original range extension")
                        # If there's still more, use the old range.
                        # (100, 200): 100
                        processed_lookups.insert(
                            i + 2,
                            LookupEntry(
                                source_number + amount,
                                original_end_range,
                                item.offset
                                + (source_number + amount - item.start_range),
                            ),
                        )
                    break

        print("Processed:", processed_lookups, seeds)
        new_seeds = []
        j = 0
        while j < len(seeds):
            start = seeds[j]
            length = seeds[j + 1]
            end = start + length + 1

            # Find the sequences to add
            for item in processed_lookups:
                # inside this range.
                if start < item.end_range:
                    end_range = min(end, item.end_range)
                    new_seeds.append(item.offset + (start - item.start_range))
                    #print(start, item.start_range, item.offset)
                    #print("end", end_range, "start", start)
                    # Length is the end - start.
                    new_seeds.append(
                        end_range - start - 1
                    )

                    # This means the item was fully encapsulated by this range.
                    if item.end_range >= end:
                        #print("breaking")
                        break
                    else:
                        # Start counting from this end
                        start = end_range
                        # The new end is still the same so we don't change it
            #print("new seeds", new_seeds)
            j += 2

        seeds = new_seeds
        print("|seed:", seeds)

    print(min(seeds[::2]))


if __name__ == "__main__":
    day2()
