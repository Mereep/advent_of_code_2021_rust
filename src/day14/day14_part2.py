from collections import Counter


def main():
    polymer, instructions = open('input.txt').read().split('\n\n')
    instructions = {line.split(' -> ')[0]: line.split(' -> ')[1] for line in instructions.split('\n')}

    # transform instructions by substituting 20 rounds
    # So that we can replace a pattern like CB to its 20 times substituted variant
    # this can be done on a normal pc
    blown_up_instructions = {}
    for key, value in instructions.items():
        curr = key
        for i in range(0, 20):
            curr = blow_up(curr, instructions)
        blown_up_instructions[key] = curr

    # do exactly one substitution (manageable by a PC)
    curr = polymer[1]
    for pos in range(0, len(polymer)-1):
        curr += blown_up_instructions[polymer[pos: pos+2]][1:]
        polymer = curr

    # now we cannot blow that thing up anymore
    # (not manageable on a normal PC due to memory constraints)
    # we instead just COUNT how many chars there WOULD be without actually substing
    counters: dict[str, Counter] = {}
    curr_counter = Counter()
    for pos in range(0, len(polymer) - 1):
        subst = polymer[pos:pos+2]
        if subst not in counters:
            counters[subst] = Counter(blown_up_instructions[subst][1:])

        curr_counter += counters[subst]

    print(max(curr_counter.values()) - min(curr_counter.values()))


def blow_up(single_instruction: str, instructions: dict[str, str]) -> str:
    """
    Takes a single instruction and blows it up using all other instructions
    that means it applies all rules exactly once
    :param single_instruction:
    :param instructions:
    :return:
    """
    output = ''
    for i in range(len(single_instruction) - 1):
        to_subst = single_instruction[i:i + 2]
        output += to_subst[0]
        if to_subst in instructions:
           output += instructions[to_subst]

    output += single_instruction[-1]
    return output


if __name__ == '__main__':
    main()
