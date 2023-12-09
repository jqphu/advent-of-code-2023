def day1():
    total = 0
    try:
        while True:
            # Example input: Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            input_string = input()
            # Split out the "Card 1:"
            _, numbers_part = input_string.split(":", 1)
            parts = numbers_part.strip().split("|")

            # Converting each part into a set of integers
            set1 = {int(num) for num in parts[0].split()}
            set2 = {int(num) for num in parts[1].split()}

            intersect = set1.intersection(set2)
            if len(intersect) > 0:
                total += 2**(len(intersect) - 1)

    except EOFError:
        print(total)

def day2():
    # Copies of every card
    #
    # We start with 1 of everything
    copies = [1] * 202

    current_card = 0

    try:
        while True:
            # Example input: Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            input_string = input()
            # Split out the "Card 1:"
            _, numbers_part = input_string.split(":", 1)
            parts = numbers_part.strip().split("|")

            # Converting each part into a set of integers
            set1 = {int(num) for num in parts[0].split()}
            set2 = {int(num) for num in parts[1].split()}

            intersect = set1.intersection(set2)

            # Depending on how many matches, increment the next cards which is a multiple of our current cards.
            for i in range(len(intersect)):
                copies[current_card + i + 1] += 1 * copies[current_card]

            current_card += 1
            # Based on ow

    except EOFError:
        print(sum(copies))



if __name__ == "__main__":
    day2()
