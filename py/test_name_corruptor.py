import os
from pathlib import Path

# content of test_sample.py
def parse_patterns(str):
    result = []
    sequences = [seq.strip() for seq in str.split(";")]
    for seq in sequences:
        parts = [part.strip() for part in seq.split("=>")]
        result.extend(zip(parts, parts[1:]))
    return result

class NameCorruptor:
    def __init__(self, patterns):
        self.patterns = patterns
        self.cursor = len(patterns)

    def corrupt_once(self, name):
        starting_cursor = self.cursor
        cursor = (starting_cursor + 1) % len(self.patterns)
        while cursor != starting_cursor:
            # grab a pattern like ("d", "t")
            (pattern, replacement) = self.patterns[cursor]

            # replace it - eg "david".replace("d", "t") => "tavit"
            new_name = name.replace(pattern, replacement)
            if new_name != name:
                # if the name changed, we're done
                self.cursor = cursor
                # todo -- put 'relax' back in
                return new_name

            # if not, keep going with the next pattern
            cursor = (cursor + 1) % len(self.patterns)

        # if we get here, we didn't find any patterns that worked
        return name


def test_parse_single_sequence():
    assert parse_patterns("a => b => c") == [("a", "b"), ("b", "c")]

def test_parse_multiple_sequences():
    assert parse_patterns("a => b => c; d => e => f") == [("a", "b"), ("b", "c"), ("d", "e"), ("e", "f")]

def test_corrupt_once():
    corruptor = NameCorruptor(parse_patterns("th => ff"))
    assert corruptor.corrupt_once("agatha") == "agaffa"

def test_corrupt_several():
    patterns = parse_patterns("""
        pir => per;
        ie => iey;
        sa => za => tsa => tzah;
        th => dd => t;
        gnu => gnae;
        cel => ciel => sel => tzel;
        lot => lod;
        ric => rick => rik => rijk;
        ph => ff => f => v => vh;
        na => ne;
        er => aer;
        dwa => dva => tva => cha;
        ao => ai => aiwa => awa => a;
        d => t;
        tta => tva;
        lle => lla => llya;
        in => en => un => um => ium;
        i => ih => y;
        por => pro;
        b => p => f;
        co => ko => kho;
        an => in => ain;
        zu => tzu;
        ace => ache => eiche;
        tt => t;
        ys => iz => it => itz => its => itsa => itsah;
        ia => aya;
        ena => ina => iyna;
        era => ira => idra;
        ick => ich => ech => eckh
        """)

    corruptor = NameCorruptor(patterns)
    remaining = 0
    N = 4
    file_path = Path(__file__).parent.parent / "src" / "name-list.txt"
    print(file_path)

    names = file_path.read_text().split('\n')

    print(len(names))

    for name in names:
        print(name)
        name = name.lower()
        generated = [name]
        message = f"{name:12}"
        for i in range(N):
            next = generated[-1]
            corrupted = corruptor.corrupt_once(next)
            generated.append(corrupted)
            message += f" => {corrupted:12}"
        final_name = generated[-1]
        if final_name == name:
            remaining += 1
            print(message)
        else:
            print(message)

    assert False
